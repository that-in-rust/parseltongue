/// Explore Swift AST to find correct node type names
use tree_sitter::{Parser, Language};

#[test]
fn explore_swift_ast_node_types() {
    let swift_code = r#"
// Swift test code
func myFunction() {
    print("Hello")
}

class MyClass {
    var name: String
}

struct MyStruct {
    var id: Int
}

protocol MyProtocol {
    func doSomething()
}

enum MyEnum {
    case first
    case second
}
"#;

    let mut parser = Parser::new();
    let swift_lang: Language = tree_sitter_swift::LANGUAGE.into();
    parser.set_language(&swift_lang).unwrap();

    let tree = parser.parse(swift_code, None).unwrap();
    let root = tree.root_node();

    println!("\n=== Swift AST Structure ===");
    print_tree(root, swift_code, 0);
}

fn print_tree(node: tree_sitter::Node, source: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    let kind = node.kind();

    // Print node kind and optionally its text if it's small
    if node.child_count() == 0 {
        let text = &source[node.byte_range()];
        if text.len() < 50 {
            println!("{}{}  \"{}\"", indent, kind, text.replace('\n', "\\n"));
        } else {
            println!("{}{}", indent, kind);
        }
    } else {
        println!("{}{}", indent, kind);
    }

    // Recursively print children
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            print_tree(child, source, depth + 1);
        }
    }
}

#[test]
fn list_all_swift_node_types() {
    let swift_lang: Language = tree_sitter_swift::LANGUAGE.into();

    println!("\n=== All Swift Grammar Node Types ===");
    for i in 0..swift_lang.node_kind_count() {
        if let Some(node_type) = swift_lang.node_kind_for_id(i as u16) {
            println!("{}: {}", i, node_type);
        }
    }
}
