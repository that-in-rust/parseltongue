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
        match language {
            Language::Rust => self.extract_rust_entities(node, source, file_path, entities),
            Language::Python => {
                // TODO: Implement Python entity extraction
            }
            _ => {}
        }

        // Recursively process child nodes
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk_node(&child, source, file_path, language, entities);
        }
    }

    /// Extract Rust-specific entities
    fn extract_rust_entities(
        &self,
        node: &tree_sitter::Node<'_>,
        source: &str,
        file_path: &Path,
        entities: &mut Vec<ParsedEntity>,
    ) {
        match node.kind() {
            "function_item" => {
                if let Some(name) = self.extract_function_name(node, source) {
                    let start_line = node.start_position().row + 1;
                    let end_line = node.end_position().row + 1;

                    entities.push(ParsedEntity {
                        entity_type: EntityType::Function,
                        name,
                        language: Language::Rust,
                        line_range: (start_line, end_line),
                        file_path: file_path.to_string_lossy().to_string(),
                        metadata: HashMap::new(),
                    });
                }
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
}