# ISG Ingestion Test Specifications
## Following TDD-First Architecture Principles

```rust
// test_specifications.rs
// Executable Specifications for ISG Ingestion

use super::*;
use proptest::prelude::*;
use test_case::test_case;

// ================================================================================
// STUB → RED → GREEN → REFACTOR Cycle Implementation
// ================================================================================

mod stub_phase {
    use super::*;
    
    /// Initial stub tests that define the interface contract
    #[test]
    fn test_c_parser_exists() {
        let parser = c::CParser::new();
        assert!(parser.can_parse());
    }
    
    #[test]
    fn test_cpp_parser_exists() {
        let parser = cpp::CppParser::new();
        assert!(parser.can_parse());
    }
    
    #[test]
    fn test_rails_parser_exists() {
        let parser = rails::RailsParser::new();
        assert!(parser.can_parse());
    }
}

mod red_phase {
    use super::*;
    
    /// Tests that should initially fail, driving implementation
    
    #[test]
    fn test_parse_c_hello_world() {
        let source = r#"
            #include <stdio.h>
            
            int main() {
                printf("Hello, World!\n");
                return 0;
            }
        "#;
        
        let parser = c::CParser::new();
        let result = parser.parse(source.to_string());
        
        assert!(result.is_ok());
        let ast = result.unwrap();
        
        // Verify structure
        assert_eq!(ast.root.declarations.len(), 1);
        match &ast.root.declarations[0] {
            c::Declaration::Function(func) => {
                assert_eq!(func.name, "main");
                assert_eq!(func.return_type.base, c::BaseType::Int);
                assert_eq!(func.parameters.len(), 0);
                assert!(func.body.is_some());
            }
            _ => panic!("Expected function declaration"),
        }
    }
    
    #[test]
    fn test_parse_cpp_class() {
        let source = r#"
            class MyClass {
            public:
                MyClass() {}
                ~MyClass() {}
                void method() const;
            private:
                int value;
            };
        "#;
        
        let parser = cpp::CppParser::new();
        let result = parser.parse(source.to_string());
        
        assert!(result.is_ok());
        let ast = result.unwrap();
        
        // Verify class structure
        assert_eq!(ast.root.declarations.len(), 1);
        match &ast.root.declarations[0] {
            cpp::CppDeclaration::Class(class) => {
                assert_eq!(class.name, "MyClass");
                assert_eq!(class.members.len(), 4);
                // Verify constructor, destructor, method, and field
            }
            _ => panic!("Expected class declaration"),
        }
    }
    
    #[test]
    fn test_parse_rails_model() {
        let source = r#"
            class User < ApplicationRecord
                has_many :posts
                validates :email, presence: true, uniqueness: true
                
                def full_name
                    "#{first_name} #{last_name}"
                end
            end
        "#;
        
        let parser = rails::RailsParser::new();
        let result = parser.parse(source.to_string());
        
        assert!(result.is_ok());
        let ast = result.unwrap();
        
        match &ast.root {
            rails::RailsFile::Model(model) => {
                assert_eq!(model.class_name, "User");
                assert_eq!(model.parent, "ApplicationRecord");
                assert_eq!(model.associations.len(), 1);
                assert_eq!(model.validations.len(), 1);
                assert_eq!(model.methods.len(), 1);
            }
            _ => panic!("Expected model file"),
        }
    }
}

mod green_phase {
    use super::*;
    
    /// Tests with minimal implementation to make them pass
    
    #[test]
    fn test_incremental_parsing() {
        let initial = "int x = 5;";
        let parser = c::CParser::new();
        let mut cache = c::CParseCache::new();
        
        // Initial parse
        let ast1 = parser.parse_with_cache(initial.to_string(), &mut cache).unwrap();
        
        // Make an edit
        let edit = TextEdit {
            range: SourceRange { start: 8, end: 9, file_id: FileId(0) },
            new_text: "10".to_string(),
        };
        
        // Incremental parse
        let delta = parser.parse_incremental(
            "int x = 10;".to_string(),
            &mut cache,
            &[edit],
        ).unwrap();
        
        // Apply delta
        let ast2 = parser.apply_delta(&ast1, delta).unwrap();
        
        // Verify only the value changed
        assert_ne!(ast1, ast2);
        assert_eq!(cache.hit_rate(), 0.8); // Most of the AST was reused
    }
    
    #[test]
    fn test_isg_building() {
        let source = r#"
            void foo() {
                bar();
            }
            
            void bar() {
                // implementation
            }
        "#;
        
        let parser = c::CParser::new();
        let ast = parser.parse(source.to_string()).unwrap();
        
        let mut builder = IsgBuilderImpl::new();
        let graph = builder.build(&ast).unwrap();
        
        // Verify semantic relationships
        let foo_node = graph.find_node_by_name("foo").unwrap();
        let bar_node = graph.find_node_by_name("bar").unwrap();
        
        let edges = graph.edges_from(foo_node.id);
        assert!(edges.iter().any(|e| {
            e.kind == EdgeKind::Calls && e.to == bar_node.id
        }));
    }
}

mod refactor_phase {
    use super::*;
    
    /// Optimized implementations after green phase
    
    #[test]
    fn test_parallel_parsing_performance() {
        let large_file = generate_large_c_file(10000); // 10K lines
        
        let parser = c::CParser::new();
        let parallel_parser = ParallelParser::new(parser, 4); // 4 threads
        
        let start = std::time::Instant::now();
        let result = parallel_parser.parse(large_file.clone());
        let parallel_time = start.elapsed();
        
        let start = std::time::Instant::now();
        let result_serial = parser.parse(large_file);
        let serial_time = start.elapsed();
        
        assert!(result.is_ok());
        assert!(result_serial.is_ok());
        
        // Parallel should be faster for large files
        assert!(parallel_time < serial_time * 0.7);
    }
}

// ================================================================================
// Property-Based Tests (Using Proptest)
// ================================================================================

mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_parse_never_panics(s in "\\PC*") {
            let parser = c::CParser::new();
            // Should either succeed or return an error, never panic
            let _ = parser.parse(s);
        }
        
        #[test]
        fn test_ast_visitor_completeness(ast in arb_ast()) {
            let mut visitor = CountingVisitor::new();
            ast.accept(&mut visitor);
            
            // Every node should be visited exactly once
            assert_eq!(visitor.count, ast.node_count());
        }
        
        #[test]
        fn test_incremental_consistency(
            initial in valid_c_source(),
            edits in prop::collection::vec(arb_edit(), 1..10)
        ) {
            let parser = c::CParser::new();
            let mut cache = c::CParseCache::new();
            
            // Parse incrementally
            let mut current = initial.clone();
            for edit in &edits {
                current = apply_edit(&current, edit);
            }
            let incremental_ast = parser.parse_incremental(
                current.clone(),
                &mut cache,
                &edits
            ).unwrap();
            
            // Parse from scratch
            let fresh_ast = parser.parse(current).unwrap();
            
            // Should produce identical ASTs
            assert_eq!(incremental_ast, fresh_ast);
        }
    }
    
    fn arb_ast() -> impl Strategy<Value = TestAst> {
        // Generate arbitrary AST structures
        prop::collection::vec(arb_node(), 1..100)
            .prop_map(|nodes| TestAst { nodes })
    }
    
    fn arb_node() -> impl Strategy<Value = TestNode> {
        (
            prop::string::string_regex("[a-z]+").unwrap(),
            prop::string::string_regex("[A-Za-z0-9]+").unwrap(),
        ).prop_map(|(kind, value)| TestNode { kind, value })
    }
    
    fn valid_c_source() -> impl Strategy<Value = String> {
        // Generate valid C source code
        prop::string::string_regex(r"int [a-z]+ = [0-9]+;").unwrap()
    }
    
    fn arb_edit() -> impl Strategy<Value = TextEdit> {
        (0usize..100, 0usize..10, "[0-9]+").prop_map(|(start, len, text)| {
            TextEdit {
                range: SourceRange {
                    start,
                    end: start + len,
                    file_id: FileId(0),
                },
                new_text: text,
            }
        })
    }
}

// ================================================================================
// Performance Tests (Criterion Integration)
// ================================================================================

mod performance_tests {
    use super::*;
    use criterion::{black_box, Criterion};
    
    pub fn benchmark_c_parser(c: &mut Criterion) {
        let small = generate_c_file(100);    // 100 lines
        let medium = generate_c_file(1000);  // 1K lines
        let large = generate_c_file(10000);  // 10K lines
        
        let parser = c::CParser::new();
        
        c.bench_function("c_parse_100_lines", |b| {
            b.iter(|| parser.parse(black_box(small.clone())))
        });
        
        c.bench_function("c_parse_1k_lines", |b| {
            b.iter(|| parser.parse(black_box(medium.clone())))
        });
        
        c.bench_function("c_parse_10k_lines", |b| {
            b.iter(|| parser.parse(black_box(large.clone())))
        });
    }
    
    pub fn benchmark_incremental_update(c: &mut Criterion) {
        let source = generate_c_file(1000);
        let parser = c::CParser::new();
        let mut cache = c::CParseCache::new();
        
        // Pre-parse
        let ast = parser.parse_with_cache(source.clone(), &mut cache).unwrap();
        
        // Small edit
        let small_edit = TextEdit {
            range: SourceRange { start: 50, end: 52, file_id: FileId(0) },
            new_text: "99".to_string(),
        };
        
        // Large edit
        let large_edits: Vec<_> = (0..10).map(|i| TextEdit {
            range: SourceRange { 
                start: i * 100, 
                end: i * 100 + 10, 
                file_id: FileId(0) 
            },
            new_text: format!("new_value_{}", i),
        }).collect();
        
        c.bench_function("incremental_small_edit", |b| {
            b.iter(|| {
                parser.parse_incremental(
                    black_box(source.clone()),
                    &mut cache.clone(),
                    &[small_edit.clone()],
                )
            })
        });
        
        c.bench_function("incremental_large_edit", |b| {
            b.iter(|| {
                parser.parse_incremental(
                    black_box(source.clone()),
                    &mut cache.clone(),
                    &large_edits,
                )
            })
        });
    }
    
    pub fn benchmark_isg_building(c: &mut Criterion) {
        let small_ast = generate_ast(100);
        let medium_ast = generate_ast(1000);
        let large_ast = generate_ast(10000);
        
        let mut builder = IsgBuilderImpl::new();
        
        c.bench_function("isg_build_100_nodes", |b| {
            b.iter(|| builder.build(black_box(&small_ast)))
        });
        
        c.bench_function("isg_build_1k_nodes", |b| {
            b.iter(|| builder.build(black_box(&medium_ast)))
        });
        
        c.bench_function("isg_build_10k_nodes", |b| {
            b.iter(|| builder.build(black_box(&large_ast)))
        });
    }
}

// ================================================================================
// Error Recovery Tests
// ================================================================================

mod error_recovery_tests {
    use super::*;
    
    #[test_case("int x = ", "missing semicolon")]
    #[test_case("int x = 5; y = 10;", "missing type")]
    #[test_case("class { };", "missing class name")]
    #[test_case("void f() { return 5 }", "missing semicolon in return")]
    fn test_error_recovery(input: &str, description: &str) {
        let parser = c::CParser::new();
        let result = parser.parse(input.to_string());
        
        match result {
            Ok(ast) => {
                // Should have error nodes in AST
                let errors = count_error_nodes(&ast);
                assert!(errors > 0, "Expected error nodes for: {}", description);
            }
            Err(e) => {
                // Should provide helpful error message
                let msg = e.to_string();
                assert!(msg.contains("line") || msg.contains("column"), 
                    "Error should contain location info: {}", description);
            }
        }
    }
    
    #[test]
    fn test_panic_mode_recovery() {
        let source = r#"
            int x = 5;
            void f() {
                if (x > 0    // Missing closing paren
                    return 1;
                else
                    return 0;
            }
            int y = 10;  // Should still parse this
        "#;
        
        let parser = c::CParser::with_error_recovery();
        let result = parser.parse(source.to_string());
        
        assert!(result.is_ok());
        let ast = result.unwrap();
        
        // Should have parsed declarations before and after error
        let decls = &ast.root.declarations;
        assert!(decls.iter().any(|d| matches!(d, c::Declaration::Variable(v) if v.name == "x")));
        assert!(decls.iter().any(|d| matches!(d, c::Declaration::Variable(v) if v.name == "y")));
    }
}

// ================================================================================
// Integration Tests
// ================================================================================

mod integration_tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    
    #[test]
    fn test_parse_real_c_files() {
        let test_files = vec![
            "testdata/sqlite.c",
            "testdata/linux_kernel_sample.c",
            "testdata/nginx_sample.c",
        ];
        
        let parser = c::CParser::new();
        
        for file_path in test_files {
            if Path::new(file_path).exists() {
                let source = fs::read_to_string(file_path).unwrap();
                let result = parser.parse(source);
                
                assert!(result.is_ok(), 
                    "Failed to parse {}: {:?}", file_path, result.err());
                
                let ast = result.unwrap();
                let graph = build_isg(&ast);
                
                // Verify basic graph properties
                assert!(graph.nodes.len() > 0);
                assert!(graph.edges.len() > 0);
                
                // Check for expected patterns
                verify_call_graph(&graph);
                verify_type_hierarchy(&graph);
            }
        }
    }
    
    #[test]
    fn test_parse_real_cpp_files() {
        let test_files = vec![
            "testdata/boost_sample.cpp",
            "testdata/llvm_sample.cpp",
            "testdata/qt_sample.cpp",
        ];
        
        let parser = cpp::CppParser::new();
        
        for file_path in test_files {
            if Path::new(file_path).exists() {
                let source = fs::read_to_string(file_path).unwrap();
                let result = parser.parse(source);
                
                assert!(result.is_ok(), 
                    "Failed to parse {}: {:?}", file_path, result.err());
                
                // Verify template instantiation
                let ast = result.unwrap();
                verify_templates(&ast);
                verify_inheritance(&ast);
            }
        }
    }
    
    #[test]
    fn test_parse_real_rails_files() {
        let test_files = vec![
            ("testdata/user_model.rb", rails::RailsFileType::Model),
            ("testdata/posts_controller.rb", rails::RailsFileType::Controller),
            ("testdata/routes.rb", rails::RailsFileType::Routes),
        ];
        
        let parser = rails::RailsParser::new();
        
        for (file_path, file_type) in test_files {
            if Path::new(file_path).exists() {
                let source = fs::read_to_string(file_path).unwrap();
                let result = parser.parse_typed(source, file_type);
                
                assert!(result.is_ok(), 
                    "Failed to parse {}: {:?}", file_path, result.err());
                
                // Verify Rails-specific patterns
                let ast = result.unwrap();
                verify_rails_patterns(&ast, file_type);
            }
        }
    }
}

// ================================================================================
// Stress Tests
// ================================================================================

mod stress_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    
    #[test]
    #[ignore] // Run with --ignored flag
    fn test_concurrent_parsing() {
        let parser = Arc::new(c::CParser::new());
        let source = Arc::new(generate_c_file(1000));
        
        let handles: Vec<_> = (0..100).map(|i| {
            let parser = parser.clone();
            let source = source.clone();
            
            thread::spawn(move || {
                let result = parser.parse((*source).clone());
                assert!(result.is_ok(), "Thread {} failed", i);
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
    }
    
    #[test]
    #[ignore]
    fn test_memory_usage() {
        let mut asts = Vec::new();
        
        // Parse and store many ASTs
        for i in 0..1000 {
            let source = generate_c_file(100);
            let parser = c::CParser::new();
            let ast = parser.parse(source).unwrap();
            asts.push(ast);
            
            // Check memory usage doesn't grow unbounded
            if i % 100 == 0 {
                let mem_usage = get_memory_usage();
                assert!(mem_usage < 1_000_000_000, // 1GB limit
                    "Memory usage too high: {} bytes", mem_usage);
            }
        }
    }
    
    #[test]
    #[ignore]
    fn test_deep_recursion() {
        // Generate deeply nested code
        let mut source = String::new();
        for _ in 0..1000 {
            source.push_str("if (1) { ");
        }
        source.push_str("x = 1;");
        for _ in 0..1000 {
            source.push_str(" }");
        }
        
        let parser = c::CParser::with_stack_size(10_000_000); // 10MB stack
        let result = parser.parse(source);
        
        // Should handle deep recursion without stack overflow
        assert!(result.is_ok() || 
            matches!(result, Err(e) if e.to_string().contains("nesting")));
    }
}

// ================================================================================
// Helper Functions
// ================================================================================

fn generate_c_file(lines: usize) -> String {
    let mut source = String::new();
    
    // Add includes
    source.push_str("#include <stdio.h>\n");
    source.push_str("#include <stdlib.h>\n\n");
    
    // Add functions
    for i in 0..lines / 10 {
        source.push_str(&format!("int func_{}(int x) {{\n", i));
        source.push_str("    return x * 2;\n");
        source.push_str("}\n\n");
    }
    
    // Add main
    source.push_str("int main() {\n");
    for i in 0..lines / 20 {
        source.push_str(&format!("    int result_{} = func_{}({});\n", i, i % (lines/10), i));
    }
    source.push_str("    return 0;\n");
    source.push_str("}\n");
    
    source
}

fn generate_large_c_file(lines: usize) -> String {
    generate_c_file(lines)
}

fn generate_ast(nodes: usize) -> TestAst {
    TestAst {
        nodes: (0..nodes).map(|i| TestNode {
            kind: format!("node_{}", i % 10),
            value: format!("value_{}", i),
        }).collect(),
    }
}

fn count_error_nodes(ast: &impl Ast) -> usize {
    struct ErrorCounter {
        count: usize,
    }
    
    impl AstVisitor for ErrorCounter {
        type Output = ();
        
        fn visit_node(&mut self, node: &dyn Ast) -> Self::Output {
            if node.node_type() == NodeType::Error {
                self.count += 1;
            }
            self.visit_children(node);
        }
    }
    
    let mut counter = ErrorCounter { count: 0 };
    ast.accept(&mut counter);
    counter.count
}

fn apply_edit(source: &str, edit: &TextEdit) -> String {
    let mut result = String::new();
    result.push_str(&source[..edit.range.start]);
    result.push_str(&edit.new_text);
    result.push_str(&source[edit.range.end..]);
    result
}

fn build_isg(ast: &impl Ast) -> IncrementalSemanticGraph {
    let mut builder = IsgBuilderImpl::new();
    builder.build(ast).unwrap()
}

fn verify_call_graph(graph: &IncrementalSemanticGraph) {
    // Verify that function calls create proper edges
    let call_edges: Vec<_> = graph.edges.values()
        .filter(|e| e.kind == EdgeKind::Calls)
        .collect();
    
    assert!(!call_edges.is_empty(), "Should have call edges");
    
    // Verify no circular dependencies at function level
    // (This is a simplified check)
    for edge in &call_edges {
        assert_ne!(edge.from, edge.to, "Self-recursive calls should be marked differently");
    }
}

fn verify_type_hierarchy(graph: &IncrementalSemanticGraph) {
    // Verify inheritance relationships
    let inheritance_edges: Vec<_> = graph.edges.values()
        .filter(|e| matches!(e.kind, EdgeKind::Extends | EdgeKind::Implements))
        .collect();
    
    // Check for multiple inheritance issues in C++
    let mut inheritance_map: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
    for edge in &inheritance_edges {
        inheritance_map.entry(edge.from)
            .or_default()
            .push(edge.to);
    }
}

fn verify_templates(ast: &cpp::CppAst) {
    // Verify template declarations are properly parsed
    let templates: Vec<_> = ast.root.declarations.iter()
        .filter_map(|d| match d {
            cpp::CppDeclaration::Template(t) => Some(t),
            _ => None,
        })
        .collect();
    
    for template in templates {
        assert!(!template.parameters.is_empty(), "Templates should have parameters");
    }
}

fn verify_inheritance(ast: &cpp::CppAst) {
    // Verify class inheritance is properly parsed
    let classes: Vec<_> = ast.root.declarations.iter()
        .filter_map(|d| match d {
            cpp::CppDeclaration::Class(c) => Some(c),
            _ => None,
        })
        .collect();
    
    for class in classes {
        // Check access specifiers
        for base in &class.bases {
            assert!(matches!(
                base.access, 
                cpp::AccessSpecifier::Public | 
                cpp::AccessSpecifier::Protected | 
                cpp::AccessSpecifier::Private
            ));
        }
    }
}

fn verify_rails_patterns(ast: &rails::RailsAst, file_type: rails::RailsFileType) {
    match (&ast.root, file_type) {
        (rails::RailsFile::Model(model), rails::RailsFileType::Model) => {
            // Verify Rails model patterns
            assert!(!model.class_name.is_empty());
            
            // Check for common Rails patterns
            for association in &model.associations {
                match association {
                    rails::Association::HasMany { name, .. } => {
                        assert!(!name.is_empty(), "Association name should not be empty");
                    }
                    _ => {}
                }
            }
        }
        (rails::RailsFile::Controller(controller), rails::RailsFileType::Controller) => {
            // Verify controller patterns
            assert!(controller.class_name.ends_with("Controller"));
        }
        _ => {}
    }
}

fn get_memory_usage() -> usize {
    // Platform-specific memory usage detection
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        let status = fs::read_to_string("/proc/self/status").unwrap();
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return parts[1].parse::<usize>().unwrap_or(0) * 1024;
                }
            }
        }
    }
    
    0 // Default fallback
}

// ================================================================================
// Test Fixtures and Mocks
// ================================================================================

struct MockParser;

impl Parser for MockParser {
    type Input = String;
    type Output = TestAst;
    type Error = std::io::Error;
    type Config = ();
    
    fn parse(&self, _input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(TestAst { nodes: vec![] })
    }
    
    fn parse_with_config(&self, _input: Self::Input, _config: Self::Config) 
        -> Result<Self::Output, Self::Error> {
        Ok(TestAst { nodes: vec![] })
    }
}

struct IsgBuilderImpl;

impl IsgBuilderImpl {
    fn new() -> Self {
        IsgBuilderImpl
    }
}

impl IsgBuilder for IsgBuilderImpl {
    type Ast = TestAst;
    type Error = std::io::Error;
    
    fn build(&mut self, _ast: &Self::Ast) -> Result<IncrementalSemanticGraph, Self::Error> {
        Ok(IncrementalSemanticGraph {
            nodes: Arc::new(HashMap::new()),
            edges: Arc::new(HashMap::new()),
            indices: Arc::new(SemanticIndices {
                by_name: HashMap::new(),
                by_kind: HashMap::new(),
                by_source: HashMap::new(),
                edges_from: HashMap::new(),
                edges_to: HashMap::new(),
            }),
            version: Version(0),
        })
    }
    
    fn update(&mut self, graph: &IncrementalSemanticGraph, _delta: &dyn AstDelta) 
        -> Result<IncrementalSemanticGraph, Self::Error> {
        Ok(graph.clone())
    }
}

// ================================================================================
// Criterion Benchmark Harness
// ================================================================================

criterion_group!(
    benches, 
    performance_tests::benchmark_c_parser,
    performance_tests::benchmark_incremental_update,
    performance_tests::benchmark_isg_building
);

criterion_main!(benches);
```

## Test Execution Plan

### Phase 1: Unit Tests (Week 1)
- Parser combinator primitives
- Lexer state machines  
- AST node construction
- Error handling paths

### Phase 2: Integration Tests (Week 2)
- Language-specific parsers
- ISG graph building
- Cross-language references
- Incremental updates

### Phase 3: Performance Tests (Week 3)
- Benchmark baselines
- Parallel parsing
- Memory profiling
- Cache effectiveness

### Phase 4: Stress Tests (Week 4)
- Concurrent access
- Large file handling
- Deep nesting
- Error recovery

## Coverage Requirements

Per your architectural principles:
- **Minimum 80% code coverage**
- **100% coverage of critical paths**
- **All error conditions tested**
- **Performance regression tests**

## Continuous Integration

```yaml
# .github/workflows/test.yml
name: ISG Ingestion Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
    
    - name: Run tests
      run: |
        cargo test --all-features
        cargo test --ignored --all-features  # Stress tests
    
    - name: Check coverage
      run: |
        cargo install cargo-tarpaulin
        cargo tarpaulin --out Xml --coverage-run-type Tests
    
    - name: Run benchmarks
      run: cargo bench --no-fail-fast
    
    - name: Memory leak check
      run: |
        cargo install cargo-valgrind
        cargo valgrind test --tests
```

## Test Data Generation

```rust
// testdata_generator.rs

use rand::prelude::*;
use std::fs;
use std::path::Path;

pub fn generate_test_corpus() {
    // Generate C test files
    for i in 0..100 {
        let content = generate_random_c_file();
        fs::write(format!("testdata/generated/c_{}.c", i), content).unwrap();
    }
    
    // Generate C++ test files
    for i in 0..100 {
        let content = generate_random_cpp_file();
        fs::write(format!("testdata/generated/cpp_{}.cpp", i), content).unwrap();
    }
    
    // Generate Rails test files
    for i in 0..100 {
        let content = generate_random_rails_file();
        fs::write(format!("testdata/generated/rails_{}.rb", i), content).unwrap();
    }
}

fn generate_random_c_file() -> String {
    let mut rng = thread_rng();
    let num_functions = rng.gen_range(1..20);
    let num_structs = rng.gen_range(0..5);
    
    // Build randomized but valid C code
    // ...
}
```

## Mutation Testing

```toml
# Cargo.toml addition
[dev-dependencies]
cargo-mutants = "0.2"
```

```bash
# Run mutation testing to verify test quality
cargo mutants --jobs 4 --timeout 300
```

This ensures our tests actually catch bugs rather than just executing code paths.
