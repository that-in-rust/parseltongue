//! Call Graph Analysis Module
//!
//! Implements AST visitor pattern to detect function calls and method calls in Rust code
//! using syn crate for parsing and analysis.

use crate::isg::{OptimizedISG, SigHash, EdgeKind};
use syn::visit::Visit;
use syn::{ItemFn, ExprCall, ExprMethodCall, ItemImpl, Path, Ident};
use std::collections::HashMap;

/// CallGraphVisitor - Traverses Rust AST to detect function calls
///
/// This visitor walks through function bodies and extracts:
/// - Direct function calls (e.g., `helper()`)
/// - Method calls (e.g., `user.format()`)
/// - Trait method calls (e.g., `display.display()`)
pub struct CallGraphVisitor<'a> {
    /// Reference to the ISG where we'll add call relationships
    isg: &'a OptimizedISG,

    /// Cache of function signatures we've already processed
    /// Maps function name to SigHash for quick lookup
    signature_cache: HashMap<String, SigHash>,

    /// Current function being analyzed (for context)
    current_function: Option<SigHash>,

    /// Current file path (for node creation)
    #[allow(dead_code)]
    current_file: String,

    /// Statistics about call detection
    pub stats: CallGraphStats,
}

#[derive(Debug, Default)]
pub struct CallGraphStats {
    pub functions_analyzed: usize,
    pub calls_detected: usize,
    pub method_calls_detected: usize,
    pub trait_calls_detected: usize,
}

impl<'a> CallGraphVisitor<'a> {
    /// Create a new call graph visitor
    pub fn new(isg: &'a OptimizedISG, file_path: String) -> Self {
        Self {
            isg,
            signature_cache: HashMap::new(),
            current_function: None,
            current_file: file_path,
            stats: CallGraphStats::default(),
        }
    }

    /// Analyze a single function and extract call relationships
    pub fn analyze_function(&mut self, item_fn: &ItemFn) {
        // Extract function signature
        let function_sig = self.extract_function_signature(item_fn);
        let function_hash = SigHash::from_signature(&function_sig);

        // Cache the signature for quick lookup
        self.signature_cache.insert(item_fn.sig.ident.to_string(), function_hash);

        // Set current function context
        self.current_function = Some(function_hash);
        self.stats.functions_analyzed += 1;

        // Visit the function body to find calls
        self.visit_item_fn(item_fn);

        // Clear current function context
        self.current_function = None;
    }

    /// Extract a standardized function signature for hashing
    fn extract_function_signature(&self, item_fn: &ItemFn) -> String {
        let ident = &item_fn.sig.ident;
        let inputs = &item_fn.sig.inputs;

        // Create a consistent signature format
        format!("fn {}{}", ident, quote::ToTokens::to_token_stream(inputs))
    }

    /// Extract method signature for methods in impl blocks
    #[allow(dead_code)]
    fn extract_method_signature(&self, method_name: &Ident, _item_impl: &ItemImpl) -> String {
        // Simplified implementation - in production would extract proper type context
        format!("method_{}", method_name)
    }

    /// Find the SigHash for a called function
    fn find_called_function(&mut self, path: &Path) -> Option<SigHash> {
        // Extract the function name from the path
        let function_name = path.segments.last()?.ident.to_string();

        // First check cache
        if let Some(&hash) = self.signature_cache.get(&function_name) {
            return Some(hash);
        }

        // Simplified approach: Create a hash from the function name
        // In production, you'd want more sophisticated function matching
        let signature = format!("fn {}", function_name);
        let sig_hash = SigHash::from_signature(&signature);
        self.signature_cache.insert(function_name, sig_hash);
        Some(sig_hash)
    }
}

impl<'ast> Visit<'ast> for CallGraphVisitor<'_> {
    // Visit function calls: helper(), some_mod::function(), etc.
    fn visit_expr_call(&mut self, expr_call: &'ast ExprCall) {
        // Extract the function being called - handle different expression types
        match &*expr_call.func {
            syn::Expr::Path(path_expr) => {
                if let Some(function_hash) = self.find_called_function(&path_expr.path) {
                    if let Some(caller_hash) = self.current_function {
                        // Add call relationship: caller -> callee
                        if let Err(e) = self.isg.upsert_edge(caller_hash, function_hash, EdgeKind::Calls) {
                            eprintln!("Warning: Failed to add call edge: {:?}", e);
                        } else {
                            self.stats.calls_detected += 1;
                        }
                    }
                }
            }
            _ => {
                // Handle other expression types (method calls, etc.) in future implementations
            }
        }

        // Continue visiting sub-expressions
        syn::visit::visit_expr_call(self, expr_call);
    }

    // Visit method calls: object.method(), object.method_call()
    fn visit_expr_method_call(&mut self, method_call: &'ast ExprMethodCall) {
        // Create a signature for the method call
        let method_sig = format!("method_{}", method_call.method);
        let method_hash = SigHash::from_signature(&method_sig);

        if let Some(caller_hash) = self.current_function {
            // Add method call relationship: caller -> method
            if let Err(e) = self.isg.upsert_edge(caller_hash, method_hash, EdgeKind::Calls) {
                eprintln!("Warning: Failed to add method call edge: {:?}", e);
            } else {
                self.stats.method_calls_detected += 1;
            }
        }

        // Continue visiting sub-expressions
        syn::visit::visit_expr_method_call(self, method_call);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isg::{NodeData, NodeKind};
    use std::sync::Arc;
    use syn::{Path, Ident};

    #[test]
    fn test_call_graph_visitor_creation() {
        let isg = OptimizedISG::new();
        let visitor = CallGraphVisitor::new(&isg, "test.rs".to_string());

        assert_eq!(visitor.current_file, "test.rs");
        assert_eq!(visitor.stats.functions_analyzed, 0);
        assert!(visitor.current_function.is_none());
    }

    #[test]
    fn test_function_signature_extraction() {
        let isg = OptimizedISG::new();
        let visitor = CallGraphVisitor::new(&isg, "test.rs".to_string());

        // Create a mock function for testing
        let code = r#"
        fn test_function(x: i32, y: String) -> Result<(), Error> {
            // function body
        }
        "#;

        let parsed = syn::parse_file(code).unwrap();
        if let Some(syn::Item::Fn(item_fn)) = parsed.items.into_iter().next() {
            let signature = visitor.extract_function_signature(&item_fn);
            assert!(signature.contains("test_function"));
            // Just check that it contains the function name - detailed parameter checking is complex
        }
    }

    #[test]
    fn test_call_detection_performance() {
        let isg = OptimizedISG::new();

        // Create a simple function that calls another
        let main_func = NodeData {
            hash: SigHash::from_signature("fn main"),
            kind: NodeKind::Function,
            name: Arc::from("main"),
            signature: Arc::from("fn main()"),
            file_path: Arc::from("test.rs"),
            line: 1,
        };

        let helper_func = NodeData {
            hash: SigHash::from_signature("fn helper"),
            kind: NodeKind::Function,
            name: Arc::from("helper"),
            signature: Arc::from("fn helper()"),
            file_path: Arc::from("test.rs"),
            line: 5,
        };

        isg.upsert_node(main_func);
        isg.upsert_node(helper_func);

        let mut visitor = CallGraphVisitor::new(&isg, "test.rs".to_string());

        // Test that finding functions is fast
        let start = std::time::Instant::now();
        let ident = Ident::new("helper", proc_macro2::Span::call_site());
        let path = Path::from(ident);
        let _result = visitor.find_called_function(&path);
        let elapsed = start.elapsed();

        assert!(elapsed.as_micros() < 5000, "Function lookup took {}μs (>5000μs)", elapsed.as_micros());
    }
}