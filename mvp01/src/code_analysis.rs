use tree_sitter::{Node, Parser};
use crate::ZipEntry;

pub fn analyze_file(entry: &ZipEntry) -> Result<ParsedFile> {
    let mut parser = Parser::new();
    let language = match detect_language(&entry.name) {
        LanguageType::Rust => tree_sitter_rust::language(),
        LanguageType::JavaScript => tree_sitter_javascript::language(),
        LanguageType::Python => tree_sitter_python::language(),
        LanguageType::Java => tree_sitter_java::language(),
        LanguageType::C => tree_sitter_c::language(),
        LanguageType::Cpp => tree_sitter_cpp::language(),
        LanguageType::Go => tree_sitter_go::language(),
        LanguageType::Unknown => return Err(anyhow::anyhow!("Unsupported language")),
    };
    parser.set_language(language).expect("Error loading language");

    let tree = parser.parse(&entry.content, None).expect("Failed to parse");
    let root_node = tree.root_node();

    let (loc, code, comments, blanks) = count_lines(&entry.content);
    let cyclomatic_complexity = calculate_cyclomatic_complexity(&root_node);
    let cognitive_complexity = calculate_cognitive_complexity(&root_node);

    Ok(ParsedFile {
        name: entry.name.clone(),
        language: detect_language(&entry.name),
        loc,
        code,
        comments,
        blanks,
        cyclomatic_complexity,
        cognitive_complexity,
    })
}

pub fn calculate_cyclomatic_complexity(node: &Node) -> usize {
    let mut complexity = 1;
    let mut cursor = node.walk();
    
    if cursor.goto_first_child() {
        loop {
            match cursor.node().kind() {
                "if_statement" | "while_statement" | "for_statement" | "match_expression" |
                "catch_clause" | "conditional_expression" | "binary_expression" => {
                    complexity += 1;
                }
                _ => {}
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    
    complexity
}

pub fn calculate_cognitive_complexity(node: &Node) -> usize {
    let mut complexity = 0;
    let mut nesting_level = 0;
    
    fn traverse(node: &Node, complexity: &mut usize, nesting_level: &mut usize) {
        match node.kind() {
            "if_statement" | "while_statement" | "for_statement" | "match_expression" => {
                *complexity += *nesting_level + 1;
                *nesting_level += 1;
            }
            "else_clause" => {
                *complexity += 1;
            }
            _ => {}
        }
        
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                traverse(&cursor.node(), complexity, nesting_level);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        
        if matches!(node.kind(), "if_statement" | "while_statement" | "for_statement" | "match_expression") {
            *nesting_level -= 1;
        }
    }
    
    traverse(node, &mut complexity, &mut nesting_level);
    complexity
}
