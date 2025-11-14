/// Debug protocol extraction
use tree_sitter::{Parser, Language};

#[test]
fn debug_protocol_ast() {
    let swift_code = r#"
protocol Drawable {
    func draw()
}
"#;

    let mut parser = Parser::new();
    let swift_lang: Language = tree_sitter_swift::LANGUAGE.into();
    parser.set_language(&swift_lang).unwrap();

    let tree = parser.parse(swift_code, None).unwrap();
    let root = tree.root_node();

    println!("\n=== Swift Protocol AST ===");
    print_tree(root, swift_code, 0);
}

fn print_tree(node: tree_sitter::Node, source: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    let kind = node.kind();

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

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            print_tree(child, source, depth + 1);
        }
    }
}
