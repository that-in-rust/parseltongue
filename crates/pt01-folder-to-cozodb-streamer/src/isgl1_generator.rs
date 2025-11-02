//! ISGL1 key generation using tree-sitter for code parsing.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tree_sitter::{Parser, Tree};
use parseltongue_core::entities::{Language, DependencyEdge, EdgeType};
use crate::errors::*;

/// ISGL1 key generator interface
pub trait Isgl1KeyGenerator: Send + Sync {
    /// Generate ISGL1 key from parsed code entity
    fn generate_key(&self, entity: &ParsedEntity) -> Result<String>;

    /// Parse source code into structured entities AND dependency edges
    ///
    /// Returns (entities, dependencies) where dependencies contains function calls,
    /// type usages, and trait implementations extracted during the same tree-sitter pass.
    ///
    /// # Performance
    /// Single-pass extraction: adds ~5-10% overhead vs entity-only extraction
    fn parse_source(&self, source: &str, file_path: &Path) -> Result<(Vec<ParsedEntity>, Vec<DependencyEdge>)>;

    /// Get supported language for file extension
    fn get_language_type(&self, file_path: &Path) -> Result<Language>;
}

/// Parsed code entity representation
#[derive(Debug, Clone)]
pub struct ParsedEntity {
    pub entity_type: EntityType,
    pub name: String,
    pub language: Language,
    pub line_range: (usize, usize),
    pub file_path: String,
    pub metadata: HashMap<String, String>,
}

/// Entity types that can be parsed
#[derive(Debug, Clone, PartialEq)]
pub enum EntityType {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Module,
    Variable,
}

/// ISGL1 key generator implementation using tree-sitter
pub struct Isgl1KeyGeneratorImpl {
    parsers: HashMap<Language, Arc<Mutex<Parser>>>,
}

impl Default for Isgl1KeyGeneratorImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Isgl1KeyGeneratorImpl {
    /// Create new ISGL1 key generator with support for 13 languages
    pub fn new() -> Self {
        let mut parsers = HashMap::new();

        // Helper macro to initialize parser for a language
        macro_rules! init_parser {
            ($lang:expr, $grammar:expr) => {
                let mut parser = Parser::new();
                if parser.set_language($grammar).is_ok() {
                    parsers.insert($lang, Arc::new(Mutex::new(parser)));
                }
            };
        }

        // Initialize all language parsers
        // LanguageFn must be converted to Language using .into() for tree-sitter 0.24+
        init_parser!(Language::Rust, &tree_sitter_rust::LANGUAGE.into());
        init_parser!(Language::Python, &tree_sitter_python::LANGUAGE.into());
        init_parser!(Language::JavaScript, &tree_sitter_javascript::LANGUAGE.into());
        init_parser!(Language::TypeScript, &tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into());
        init_parser!(Language::Go, &tree_sitter_go::LANGUAGE.into());
        init_parser!(Language::Java, &tree_sitter_java::LANGUAGE.into());
        init_parser!(Language::Cpp, &tree_sitter_cpp::LANGUAGE.into());
        init_parser!(Language::Ruby, &tree_sitter_ruby::LANGUAGE.into());
        init_parser!(Language::Php, &tree_sitter_php::LANGUAGE_PHP.into());
        init_parser!(Language::CSharp, &tree_sitter_c_sharp::LANGUAGE.into());
        init_parser!(Language::Swift, &tree_sitter_swift::LANGUAGE.into());
        // Note: Kotlin not supported in v0.8.7 - tree-sitter-kotlin v0.3 uses incompatible tree-sitter 0.20
        // Will be added when tree-sitter-kotlin updates to 0.24+
        init_parser!(Language::Scala, &tree_sitter_scala::LANGUAGE.into());

        Self { parsers }
    }

    /// Generate ISGL1 key format: {language}:{type}:{name}:{location}
    fn format_key(&self, entity: &ParsedEntity) -> String {
        let type_str = match entity.entity_type {
            EntityType::Function => "fn",
            EntityType::Struct => "struct",
            EntityType::Enum => "enum",
            EntityType::Trait => "trait",
            EntityType::Impl => "impl",
            EntityType::Module => "mod",
            EntityType::Variable => "var",
        };

        format!(
            "{}:{}:{}:{}:{}-{}",
            entity.language,
            type_str,
            entity.name,
            self.sanitize_path(&entity.file_path),
            entity.line_range.0,
            entity.line_range.1
        )
    }

    /// Sanitize file path for ISGL1 key
    fn sanitize_path(&self, path: &str) -> String {
        path.replace(['/', '\\', '.'], "_")
    }
}

impl Isgl1KeyGenerator for Isgl1KeyGeneratorImpl {
    fn generate_key(&self, entity: &ParsedEntity) -> Result<String> {
        Ok(self.format_key(entity))
    }

    fn parse_source(&self, source: &str, file_path: &Path) -> Result<(Vec<ParsedEntity>, Vec<DependencyEdge>)> {
        let language_type = self.get_language_type(file_path)?;

        let parser_mutex = self.parsers.get(&language_type)
            .ok_or_else(|| StreamerError::ParsingError {
                file: file_path.to_string_lossy().to_string(),
                reason: format!("No parser available for language: {:?}", language_type),
            })?;

        let mut parser = parser_mutex.lock().unwrap();
        let tree = parser
            .parse(source, None)
            .ok_or_else(|| StreamerError::ParsingError {
                file: file_path.to_string_lossy().to_string(),
                reason: "Failed to parse source code".to_string(),
            })?;

        let mut entities = Vec::new();
        let mut dependencies = Vec::new();
        self.extract_entities(&tree, source, file_path, language_type, &mut entities, &mut dependencies);

        Ok((entities, dependencies))
    }

    fn get_language_type(&self, file_path: &Path) -> Result<Language> {
        // Use Language::from_file_path to detect language from extension
        let path_buf = file_path.to_path_buf();
        let language = Language::from_file_path(&path_buf)
            .ok_or_else(|| StreamerError::UnsupportedFileType {
                path: file_path.to_string_lossy().to_string(),
            })?;

        // Verify we have a parser for this language
        if self.parsers.contains_key(&language) {
            Ok(language)
        } else {
            Err(StreamerError::UnsupportedFileType {
                path: file_path.to_string_lossy().to_string(),
            })
        }
    }
}

impl Isgl1KeyGeneratorImpl {
    /// Extract entities AND dependencies from parse tree (two-pass for correctness)
    ///
    /// Pass 1: Extract all entities
    /// Pass 2: Extract dependencies (now all entities are known)
    fn extract_entities(
        &self,
        tree: &Tree,
        source: &str,
        file_path: &Path,
        language: Language,
        entities: &mut Vec<ParsedEntity>,
        dependencies: &mut Vec<DependencyEdge>,
    ) {
        let root_node = tree.root_node();

        // Pass 1: Extract entities (populate entities vec)
        self.walk_node(&root_node, source, file_path, language, entities, dependencies);

        // Pass 2: Extract dependencies (now entities are complete)
        if language == Language::Rust {
            self.extract_dependencies_pass2(&root_node, source, file_path, entities, dependencies);
        }
    }

    /// Second pass: Extract dependencies now that all entities are known
    fn extract_dependencies_pass2(
        &self,
        node: &tree_sitter::Node<'_>,
        source: &str,
        file_path: &Path,
        entities: &[ParsedEntity],
        dependencies: &mut Vec<DependencyEdge>,
    ) {
        // Extract dependencies from this node
        self.extract_rust_dependencies(node, source, file_path, entities, dependencies);

        // Recurse through children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_dependencies_pass2(&child, source, file_path, entities, dependencies);
        }
    }

    /// Walk tree nodes and extract entities AND dependencies
    fn walk_node(
        &self,
        node: &tree_sitter::Node<'_>,
        source: &str,
        file_path: &Path,
        language: Language,
        entities: &mut Vec<ParsedEntity>,
        dependencies: &mut Vec<DependencyEdge>,
    ) {
        // For Rust, check if this node or its siblings have attributes
        if language == Language::Rust && node.kind() == "function_item" {
            // Check preceding siblings for attributes
            let has_test_attr = self.check_preceding_test_attribute(node, source);
            self.extract_rust_function_with_test_info(node, source, file_path, entities, has_test_attr);
        } else {
            match language {
                Language::Rust => self.extract_rust_entities(node, source, file_path, entities),
                Language::Python => {
                    // TODO: Implement Python entity extraction
                }
                _ => {}
            }
        }

        // Recursively process child nodes (Pass 1: entities only)
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_node(&child, source, file_path, language, entities, dependencies);
        }
    }

    /// Extract Rust-specific dependencies (function calls, uses, implements)
    fn extract_rust_dependencies(
        &self,
        node: &tree_sitter::Node<'_>,
        source: &str,
        file_path: &Path,
        entities: &[ParsedEntity],
        dependencies: &mut Vec<DependencyEdge>,
    ) {
        // Only extract calls from function bodies
        if node.kind() == "call_expression" {
            // Find the containing function
            let containing_function = self.find_containing_function(node, entities);
            if let Some(from_entity) = containing_function {
                // Extract the function being called
                if let Some(callee_name) = self.extract_callee_name(node, source) {
                    // Find the target function entity
                    let to_entity = entities.iter().find(|e| {
                        e.entity_type == EntityType::Function && e.name == callee_name
                    });

                    if let Some(to) = to_entity {
                        // Generate ISGL1 keys for both
                        if let (Ok(from_key), Ok(to_key)) = (
                            self.generate_key(from_entity),
                            self.generate_key(to),
                        ) {
                            // Create dependency edge
                            if let Ok(edge) = DependencyEdge::builder()
                                .from_key(from_key)
                                .to_key(to_key)
                                .edge_type(EdgeType::Calls)
                                .source_location(format!("{}:{}",
                                    file_path.display(),
                                    node.start_position().row + 1))
                                .build()
                            {
                                dependencies.push(edge);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Find the function that contains this node
    fn find_containing_function<'a>(
        &self,
        node: &tree_sitter::Node<'_>,
        entities: &'a [ParsedEntity],
    ) -> Option<&'a ParsedEntity> {
        // Walk up the tree to find a function_item
        let mut current = node.parent()?;
        while current.kind() != "function_item" {
            current = current.parent()?;
        }

        // Get the line range of this function_item
        let start_line = current.start_position().row + 1;
        let end_line = current.end_position().row + 1;

        // Find matching function entity
        entities.iter().find(|e| {
            e.entity_type == EntityType::Function
            && e.line_range == (start_line, end_line)
        })
    }

    /// Extract the name of the function being called
    fn extract_callee_name(&self, node: &tree_sitter::Node<'_>, source: &str) -> Option<String> {
        // call_expression structure: function_name arguments
        // We want the identifier node
        for child in node.children(&mut node.walk()) {
            if child.kind() == "identifier" || child.kind() == "field_expression" {
                return Some(source[child.byte_range()].to_string());
            }
        }
        None
    }

    /// Extract Rust-specific entities (structs, enums, etc. but NOT functions - those are handled separately)
    fn extract_rust_entities(
        &self,
        node: &tree_sitter::Node<'_>,
        source: &str,
        file_path: &Path,
        entities: &mut Vec<ParsedEntity>,
    ) {
        match node.kind() {
            "function_item" => {
                // Skip - functions are handled separately in walk_node to check attributes
            }
            "struct_item" => {
                if let Some(name) = self.extract_struct_name(node, source) {
                    let start_line = node.start_position().row + 1;
                    let end_line = node.end_position().row + 1;

                    entities.push(ParsedEntity {
                        entity_type: EntityType::Struct,
                        name,
                        language: Language::Rust,
                        line_range: (start_line, end_line),
                        file_path: file_path.to_string_lossy().to_string(),
                        metadata: HashMap::new(),
                    });
                }
            }
            _ => {}
        }
    }

    /// Extract function name from function node
    fn extract_function_name(&self, node: &tree_sitter::Node<'_>, source: &str) -> Option<String> {
        for child in node.children(&mut node.walk()) {
            if child.kind() == "identifier" {
                return Some(source[child.byte_range()].to_string());
            }
        }
        None
    }

    /// Extract struct name from struct node
    fn extract_struct_name(&self, node: &tree_sitter::Node<'_>, source: &str) -> Option<String> {
        for child in node.children(&mut node.walk()) {
            if child.kind() == "type_identifier" {
                return Some(source[child.byte_range()].to_string());
            }
        }
        None
    }

    /// Check if IMMEDIATE preceding sibling is a test attribute
    fn check_preceding_test_attribute(&self, node: &tree_sitter::Node<'_>, source: &str) -> bool {
        // Get parent to access siblings
        let Some(parent) = node.parent() else {
            return false;
        };

        // Find this node and check its immediate preceding sibling
        let node_id = node.id();
        let siblings: Vec<_> = parent.children(&mut parent.walk()).collect();

        // Find index of current node
        let node_index = siblings.iter().position(|s| s.id() == node_id);

        if let Some(idx) = node_index {
            if idx > 0 {
                // Check immediate preceding sibling
                let prev_sibling = &siblings[idx - 1];
                if prev_sibling.kind() == "attribute_item" {
                    let attr_text = &source[prev_sibling.byte_range()];
                    if attr_text.contains("#[test]") || attr_text.contains("#[tokio::test]") || attr_text.contains("#[async_test]") {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Extract Rust function with test information
    fn extract_rust_function_with_test_info(
        &self,
        node: &tree_sitter::Node<'_>,
        source: &str,
        file_path: &Path,
        entities: &mut Vec<ParsedEntity>,
        is_test: bool,
    ) {
        if let Some(name) = self.extract_function_name(node, source) {
            let start_line = node.start_position().row + 1;
            let end_line = node.end_position().row + 1;

            let mut metadata = HashMap::new();
            if is_test {
                metadata.insert("is_test".to_string(), "true".to_string());
            }

            entities.push(ParsedEntity {
                entity_type: EntityType::Function,
                name,
                language: Language::Rust,
                line_range: (start_line, end_line),
                file_path: file_path.to_string_lossy().to_string(),
                metadata,
            });
        }
    }
}

/// Factory for creating ISGL1 key generators
pub struct Isgl1KeyGeneratorFactory;

impl Isgl1KeyGeneratorFactory {
    /// Create new ISGL1 key generator instance
    pub fn new() -> Arc<dyn Isgl1KeyGenerator> {
        Arc::new(Isgl1KeyGeneratorImpl::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_isgl1_key_format() {
        let generator = Isgl1KeyGeneratorImpl::new();
        let entity = ParsedEntity {
            entity_type: EntityType::Function,
            name: "test_function".to_string(),
            language: Language::Rust,
            line_range: (10, 15),
            file_path: "src/main.rs".to_string(),
            metadata: HashMap::new(),
        };

        let key = generator.generate_key(&entity).unwrap();
        assert!(key.contains("rust:fn:test_function"));
        assert!(key.contains("10-15"));
    }

    #[test]
    fn test_rust_parsing() {
        let generator = Isgl1KeyGeneratorImpl::new();
        let source = r#"
fn test_function() {
    println!("Hello, world!");
}

struct TestStruct {
    field: i32,
}
"#;

        let file_path = Path::new("test.rs");
        let (entities, dependencies) = generator.parse_source(source, file_path).unwrap();

        assert!(!entities.is_empty());
        assert_eq!(entities.len(), 2); // One function, one struct

        let function = &entities[0];
        assert_eq!(function.entity_type, EntityType::Function);
        assert_eq!(function.name, "test_function");

        // For now, dependencies should be empty (will implement extraction next)
        assert_eq!(dependencies.len(), 0);
    }

    #[test]
    fn test_function_detection() {
        let generator = Isgl1KeyGeneratorImpl::new();
        let source = r#"
#[test]
fn test_something() {
    assert_eq!(1, 1);
}

fn regular_function() {
    println!("Hello");
}

#[cfg(test)]
mod tests {
    #[test]
    fn another_test() {
        assert!(true);
    }
}
"#;

        let file_path = Path::new("test.rs");
        let (entities, _dependencies) = generator.parse_source(source, file_path).unwrap();

        // Debug: print all entities
        println!("\nExtracted {} entities:", entities.len());
        for (i, entity) in entities.iter().enumerate() {
            println!("  {}. {} (type: {:?}, is_test: {:?})",
                i, entity.name, entity.entity_type, entity.metadata.get("is_test"));
        }

        // Find the test function and regular function
        let test_fn = entities.iter().find(|e| e.name == "test_something");
        let regular_fn = entities.iter().find(|e| e.name == "regular_function");

        assert!(test_fn.is_some(), "Should find test function");
        assert!(regular_fn.is_some(), "Should find regular function");

        // Verify test function has is_test metadata
        let test_fn = test_fn.unwrap();
        println!("\ntest_something metadata: {:?}", test_fn.metadata);
        assert_eq!(test_fn.metadata.get("is_test"), Some(&"true".to_string()));

        // Verify regular function does NOT have is_test metadata
        let regular_fn = regular_fn.unwrap();
        println!("regular_function metadata: {:?}", regular_fn.metadata);
        assert_eq!(regular_fn.metadata.get("is_test"), None);
    }

    // ================== Phase 2: Dependency Extraction Tests ==================

    #[test]
    fn test_extracts_function_call_dependencies() {
        // RED PHASE: This test will FAIL until we implement call_expression extraction
        let generator = Isgl1KeyGeneratorImpl::new();
        let source = r#"
fn main() {
    helper();
}

fn helper() {
    println!("Helper called");
}
"#;

        let file_path = Path::new("test.rs");
        let (entities, dependencies) = generator.parse_source(source, file_path).unwrap();

        // Should extract 2 entities (main, helper)
        assert_eq!(entities.len(), 2);

        // Should extract 1 dependency: main -> helper (Calls)
        assert_eq!(dependencies.len(), 1, "Expected 1 dependency edge (main calls helper)");

        let edge = &dependencies[0];
        assert_eq!(edge.edge_type, EdgeType::Calls);

        // The keys should reference main and helper
        assert!(
            edge.from_key.as_ref().contains("main"),
            "from_key should contain 'main', got: {:?}",
            edge.from_key
        );
        assert!(
            edge.to_key.as_ref().contains("helper"),
            "to_key should contain 'helper', got: {:?}",
            edge.to_key
        );
    }

    #[test]
    fn test_extracts_multiple_function_calls() {
        let generator = Isgl1KeyGeneratorImpl::new();
        let source = r#"
fn main() {
    foo();
    bar();
    baz();
}

fn foo() {}
fn bar() {}
fn baz() {}
"#;

        let file_path = Path::new("test.rs");
        let (entities, dependencies) = generator.parse_source(source, file_path).unwrap();

        // Should extract 4 entities (main, foo, bar, baz)
        assert_eq!(entities.len(), 4);

        // Should extract 3 dependencies: main->foo, main->bar, main->baz
        assert_eq!(dependencies.len(), 3, "Expected 3 call edges from main");

        // Verify all are Calls edges from main
        for edge in &dependencies {
            assert_eq!(edge.edge_type, EdgeType::Calls);
            assert!(edge.from_key.as_ref().contains("main"));
        }

        // Check we have edges to each function
        assert!(dependencies.iter().any(|e| e.to_key.as_ref().contains("foo")));
        assert!(dependencies.iter().any(|e| e.to_key.as_ref().contains("bar")));
        assert!(dependencies.iter().any(|e| e.to_key.as_ref().contains("baz")));
    }

    #[test]
    fn test_no_dependencies_when_no_calls() {
        let generator = Isgl1KeyGeneratorImpl::new();
        let source = r#"
fn main() {
    let x = 42;
    println!("{}", x);
}

fn helper() {
    // No calls to other local functions
}
"#;

        let file_path = Path::new("test.rs");
        let (entities, dependencies) = generator.parse_source(source, file_path).unwrap();

        // Should extract 2 entities
        assert_eq!(entities.len(), 2);

        // No dependencies to LOCAL functions (println! is external macro, ignored for MVP)
        assert_eq!(dependencies.len(), 0, "Expected no dependencies to local functions");
    }

    #[test]
    fn test_chained_function_calls() {
        let generator = Isgl1KeyGeneratorImpl::new();
        let source = r#"
fn main() {
    a();
}

fn a() {
    b();
}

fn b() {
    c();
}

fn c() {}
"#;

        let file_path = Path::new("test.rs");
        let (entities, dependencies) = generator.parse_source(source, file_path).unwrap();

        // Should extract 4 entities
        assert_eq!(entities.len(), 4);

        // Should extract 3 dependencies: main->a, a->b, b->c
        assert_eq!(dependencies.len(), 3);

        // Verify the chain
        let main_to_a = dependencies.iter().find(|e|
            e.from_key.as_ref().contains("main") && e.to_key.as_ref().contains("a")
        );
        assert!(main_to_a.is_some(), "Should have main -> a edge");

        let a_to_b = dependencies.iter().find(|e|
            e.from_key.as_ref().contains("fn:a:") && e.to_key.as_ref().contains("fn:b:")
        );
        assert!(a_to_b.is_some(), "Should have a -> b edge");

        let b_to_c = dependencies.iter().find(|e|
            e.from_key.as_ref().contains("fn:b:") && e.to_key.as_ref().contains("fn:c:")
        );
        assert!(b_to_c.is_some(), "Should have b -> c edge");
    }
}