/// Debug the full Swift code to see what's parsed
use tree_sitter::{Parser, Language, Query, QueryCursor, StreamingIterator};

#[test]
fn debug_full_swift_code() {
    let swift_code = r#"
// Test Swift file for entity extraction
import Foundation

// Function
func calculateSum(a: Int, b: Int) -> Int {
    return a + b
}

// Class
class UserManager {
    var users: [String] = []

    func addUser(name: String) {
        users.append(name)
    }
}

// Struct
struct Point {
    var x: Double
    var y: Double

    func distance(to other: Point) -> Double {
        let dx = x - other.x
        let dy = y - other.y
        return sqrt(dx * dx + dy * dy)
    }
}

// Enum
enum Direction {
    case north
    case south
    case east
    case west
}

// Protocol
protocol Drawable {
    func draw()
}
"#;

    let mut parser = Parser::new();
    let swift_lang: Language = tree_sitter_swift::LANGUAGE.into();
    parser.set_language(&swift_lang).unwrap();

    let tree = parser.parse(swift_code, None).unwrap();

    println!("\n=== Swift Parse Tree (Top Level) ===");
    let root = tree.root_node();
    let mut cursor = root.walk();

    for child in root.children(&mut cursor) {
        let text_snippet = &swift_code[child.byte_range()];
        let first_line = text_snippet.lines().next().unwrap_or("");
        println!(
            "{} at lines {}-{}: {}",
            child.kind(),
            child.start_position().row + 1,
            child.end_position().row + 1,
            first_line.chars().take(60).collect::<String>()
        );
    }

    // Now test the query
    let swift_query = include_str!("../../../entity_queries/swift.scm");
    let query = Query::new(&swift_lang, swift_query).unwrap();

    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, tree.root_node(), swift_code.as_bytes());

    println!("\n=== Query Matches ===");
    while let Some(m) = matches.next() {
        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let text = &swift_code[capture.node.byte_range()];

            if *capture_name == "name" {
                let def_type = m.captures.iter()
                    .find(|c| query.capture_names()[c.index as usize].starts_with("definition."))
                    .map(|c| &query.capture_names()[c.index as usize])
                    .unwrap_or(&"unknown");

                println!("{}: {} (line {})", def_type, text.trim(), capture.node.start_position().row + 1);
            }
        }
    }
}
