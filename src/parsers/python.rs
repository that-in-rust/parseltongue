use crate::language_traits::{LanguageParser, Entity, EntityKind, EntityMetadata, Visibility, ParseError, Dependency, DependencyKind, extract_entities_from_tree};
use tree_sitter::{Tree, Node};
use tree_sitter_python;

pub struct PythonParser;

impl LanguageParser for PythonParser {
    fn parse_file(&self, file_path: &str, code: &str) -> Result<Vec<Entity>, ParseError> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_python::language())
            .map_err(|e| ParseError {
                message: format!("Failed to set Python language: {}", e),
                file: file_path.to_string(),
                line: None,
            })?;

        let tree = parser.parse(code, None)
            .ok_or_else(|| ParseError {
                message: "Failed to parse Python code".to_string(),
                file: file_path.to_string(),
                line: None,
            })?;

        let mut entities = Vec::new();
        let root_node = tree.root_node();
        extract_entities_from_tree(root_node, code, file_path, &mut entities);

        Ok(entities)
    }

    fn parse_to_tree(&self, code: &str) -> Result<Tree, ParseError> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_python::language())
            .map_err(|e| ParseError {
                message: format!("Failed to set Python language: {}", e),
                file: "".to_string(),
                line: None,
            })?;

        parser.parse(code, None)
            .ok_or_else(|| ParseError {
                message: "Failed to parse Python code".to_string(),
                file: "".to_string(),
                line: None,
            })
    }

    fn supported_extensions(&self) -> Vec<&'static str> {
        vec![".py"]
    }

    fn language_name(&self) -> &'static str {
        "python"
    }

    fn extract_dependencies(&self, tree: &Tree, source_code: &str) -> Vec<Dependency> {
        let mut dependencies = Vec::new();

        // Extract class inheritance
        let mut cursor = tree.root_node().walk();
        for node in tree.root_node().children(&mut cursor) {
            if node.kind() == "class_definition" {
                if let Some(class_deps) = extract_class_dependencies(node, source_code) {
                    dependencies.extend(class_deps);
                }
            }
        }

        dependencies
    }

    fn get_language(&self) -> tree_sitter::Language {
        tree_sitter_python::language()
    }
}

fn extract_class_dependencies(node: Node, source_code: &str) -> Option<Vec<Dependency>> {
    let mut dependencies = Vec::new();

    // Find class name
    if let Some(name_node) = node.child_by_field_name("name") {
        let class_name = name_node.utf8_text(source_code.as_bytes()).ok()?;

        // Find base classes
        if let Some(bases_node) = node.child_by_field_name("bases") {
            let mut cursor = bases_node.walk();
            for base in bases_node.children(&mut cursor) {
                if base.kind() == "identifier" {
                    let base_name = base.utf8_text(source_code.as_bytes()).ok()?;
                    dependencies.push(Dependency {
                        from: class_name.to_string(),
                        to: base_name.to_string(),
                        kind: DependencyKind::Extends,
                    });
                }
            }
        }
    }

    Some(dependencies)
}
