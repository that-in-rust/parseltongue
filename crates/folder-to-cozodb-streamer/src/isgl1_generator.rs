//! ISGL1 key generation using tree-sitter for code parsing.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tree_sitter::{Language as TreeSitterLanguage, Parser, Tree};
use parseltongue_core::entities::Language;
use crate::errors::*;

/// ISGL1 key generator interface
pub trait Isgl1KeyGenerator: Send + Sync {
    /// Generate ISGL1 key from parsed code entity
    fn generate_key(&self, entity: &ParsedEntity) -> Result<String>;

    /// Parse source code into structured entities
    fn parse_source(&self, source: &str, file_path: &Path) -> Result<Vec<ParsedEntity>>;

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
    rust_language: TreeSitterLanguage,
    python_language: Option<TreeSitterLanguage>,
    parsers: HashMap<Language, Arc<Mutex<Parser>>>,
}

impl Isgl1KeyGeneratorImpl {
    /// Create new ISGL1 key generator
    pub fn new() -> Self {
        let mut generators = HashMap::new();

        // Initialize Rust parser
        let mut rust_parser = Parser::new();
        rust_parser
            .set_language(tree_sitter_rust::language())
            .expect("Error loading Rust grammar");
        generators.insert(Language::Rust, Arc::new(Mutex::new(rust_parser)));

        Self {
            rust_language: tree_sitter_rust::language(),
            python_language: None, // TODO: Add Python support
            parsers: generators,
        }
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
            entity.language.to_string(),
            type_str,
            entity.name,
            self.sanitize_path(&entity.file_path),
            entity.line_range.0,
            entity.line_range.1
        )
    }

    /// Sanitize file path for ISGL1 key
    fn sanitize_path(&self, path: &str) -> String {
        path.replace('/', "_")
            .replace('\\', "_")
            .replace('.', "_")
    }
}

impl Isgl1KeyGenerator for Isgl1KeyGeneratorImpl {
    fn generate_key(&self, entity: &ParsedEntity) -> Result<String> {
        Ok(self.format_key(entity))
    }

    fn parse_source(&self, source: &str, file_path: &Path) -> Result<Vec<ParsedEntity>> {
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
        self.extract_entities(&tree, source, file_path, language_type, &mut entities);

        Ok(entities)
    }

    fn get_language_type(&self, file_path: &Path) -> Result<Language> {
        match file_path.extension().and_then(|s| s.to_str()) {
            Some("rs") => Ok(Language::Rust),
            Some("py") => {
                if self.python_language.is_some() {
                    Ok(Language::Python)
                } else {
                    Err(StreamerError::UnsupportedFileType {
                        path: file_path.to_string_lossy().to_string(),
                    })
                }
            }
            _ => Err(StreamerError::UnsupportedFileType {
                path: file_path.to_string_lossy().to_string(),
            }),
        }
    }
}

impl Isgl1KeyGeneratorImpl {
    /// Extract entities from parse tree
    fn extract_entities(
        &self,
        tree: &Tree,
        source: &str,
        file_path: &Path,
        language: Language,
        entities: &mut Vec<ParsedEntity>,
    ) {
        let root_node = tree.root_node();
        self.walk_node(&root_node, source, file_path, language, entities);
    }

    /// Walk tree nodes and extract entities
    fn walk_node(
        &self,
        node: &tree_sitter::Node<'_>,
        source: &str,
        file_path: &Path,
        language: Language,
        entities: &mut Vec<ParsedEntity>,
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

        // Recursively process child nodes
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_node(&child, source, file_path, language, entities);
        }
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
        let entities = generator.parse_source(source, file_path).unwrap();

        assert!(!entities.is_empty());
        assert_eq!(entities.len(), 2); // One function, one struct

        let function = &entities[0];
        assert_eq!(function.entity_type, EntityType::Function);
        assert_eq!(function.name, "test_function");
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
        let entities = generator.parse_source(source, file_path).unwrap();

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
}