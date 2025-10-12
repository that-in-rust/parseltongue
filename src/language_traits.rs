use std::error::Error;
use std::fmt;
use tree_sitter::{Tree, Node};

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub file: String,
    pub line: Option<usize>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error in {}: {}", self.file, self.message)
    }
}

impl Error for ParseError {}

/// Trait for language parsers that can work with different parsing backends
pub trait LanguageParser: Send + Sync {
    /// Parse a single file and return extracted entities
    fn parse_file(&self, file_path: &str, code: &str) -> Result<Vec<Entity>, ParseError>;

    /// Return file extensions this parser supports
    fn supported_extensions(&self) -> Vec<&'static str>;

    /// Return the language name (for logging/CLI)
    fn language_name(&self) -> &'static str;

    /// Optional: Extract dependencies between entities
    /// Default implementation works with entity list, but parsers can override for more sophisticated analysis
    fn extract_dependencies(&self, entities: &[Entity]) -> Vec<Dependency> {
        Vec::new() // Default implementation
    }

    /// Optional: Extract dependencies using tree-sitter Tree (for parsers that support it)
    /// Default implementation falls back to entity-based analysis
    fn extract_dependencies_with_tree(&self, _tree: &Tree, _source_code: &str) -> Vec<Dependency> {
        Vec::new() // Default implementation - parsers can override
    }

    /// Check if this parser supports tree-sitter Tree-based analysis
    fn supports_tree_analysis(&self) -> bool {
        false // Default implementation - parsers can override
    }
}

/// Trait specifically for tree-sitter based parsers
pub trait TreeSitterParser: LanguageParser {
    /// Get tree-sitter language for this parser
    fn get_language(&self) -> tree_sitter::Language;

    /// Parse code and return tree-sitter Tree for advanced analysis
    fn parse_to_tree(&self, code: &str) -> Result<Tree, ParseError>;

    fn supports_tree_analysis(&self) -> bool {
        true // Tree-sitter parsers support tree analysis by definition
    }

    fn extract_dependencies(&self, entities: &[Entity]) -> Vec<Dependency> {
        // For tree-sitter parsers, we might want to override this to use tree-based analysis
        self.extract_dependencies_with_tree(&self.parse_to_tree("").unwrap_or_else(|_| {
            // If we can't parse empty string, create a minimal tree
            let mut parser = tree_sitter::Parser::new();
                // Last resort - create an empty tree
                Tree::new(tree_sitter::InputEdit::default(), tree_sitter::Range::default())
            })
        }), "")
    }
}

/// Extract entities from tree-sitter nodes recursively
pub fn extract_entities_from_tree(
    node: Node,
    source_code: &str,
    file_path: &str,
    mut entities: &mut Vec<Entity>
) {
    match node.kind() {
        "function_definition" | "function_item" => {
            if let Some(entity) = extract_function_entity(node, source_code, file_path) {
                entities.push(entity);
            }
        }
        "class_definition" | "struct_item" => {
            if let Some(entity) = extract_class_entity(node, source_code, file_path) {
                entities.push(entity);
            }
        }
        "trait_item" => {
            if let Some(entity) = extract_trait_entity(node, source_code, file_path) {
                entities.push(entity);
            }
        }
        _ => {
            // Recursively process children
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                extract_entities_from_tree(child, source_code, file_path, &mut entities);
            }
        }
    }
}

fn extract_function_entity(node: Node, source_code: &str, file_path: &str) -> Option<Entity> {
    let name_node = node.child_by_field_name("name")?;
    let name = name_node.utf8_text(source_code.as_bytes()).ok()?.to_string();

    let start_byte = node.start_byte();
    let end_byte = node.end_byte();
    let signature = source_code.get(start_byte..end_byte)?.trim().to_string();
    let full_signature = signature.clone();

    Some(Entity {
        kind: EntityKind::Function,
        name,
        signature,
        full_signature,
        file_path: file_path.to_string(),
        line: node.start_position().row as u32 + 1,
        column: node.start_position().column as u32 + 1,
        metadata: EntityMetadata {
            visibility: Visibility::Public, // Default - can be enhanced with more analysis
            modifiers: vec!["function".to_string()],
            generic_params: Vec::new(),
            doc_comment: None,
        },
    })
}

fn extract_class_entity(node: Node, source_code: &str, file_path: &str) -> Option<Entity> {
    let name_node = node.child_by_field_name("name")?;
    let name = name_node.utf8_text(source_code.as_bytes()).ok()?.to_string();

    let start_byte = node.start_byte();
    let end_byte = node.end_byte();
    let signature = source_code.get(start_byte..end_byte)?.trim().to_string();
    let full_signature = signature.clone();

    Some(Entity {
        kind: EntityKind::Class,
        name,
        signature,
        full_signature,
        file_path: file_path.to_string(),
        line: node.start_position().row as u32 + 1,
        column: node.start_position().column as u32 + 1,
        metadata: EntityMetadata {
            visibility: Visibility::Public,
            modifiers: vec!["class".to_string()],
            generic_params: Vec::new(),
            doc_comment: None,
        },
    })
}

fn extract_trait_entity(node: Node, source_code: &str, file_path: &str) -> Option<Entity> {
    let name_node = node.child_by_field_name("name")?;
    let name = name_node.utf8_text(source_code.as_bytes()).ok()?.to_string();

    let start_byte = node.start_byte();
    let end_byte = node.end_byte();
    let signature = source_code.get(start_byte..end_byte)?.trim().to_string();
    let full_signature = signature.clone();

    Some(Entity {
        kind: EntityKind::Trait,
        name,
        signature,
        full_signature,
        file_path: file_path.to_string(),
        line: node.start_position().row as u32 + 1,
        column: node.start_position().column as u32 + 1,
        metadata: EntityMetadata {
            visibility: Visibility::Public,
            modifiers: vec!["trait".to_string()],
            generic_params: Vec::new(),
            doc_comment: None,
        },
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dependency {
    pub from: String,  // Entity name
    pub to: String,    // Dependency name
    pub kind: DependencyKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DependencyKind {
    Implements,
    Extends,
    Uses,
    Calls,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    pub kind: EntityKind,
    pub name: String,
    pub signature: String,      // Human-readable representation
    pub full_signature: String, // Complete signature for hashing
    pub file_path: String,
    pub line: u32,
    pub column: u32,
    pub metadata: EntityMetadata, // Language-specific data
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityKind {
    Function,
    Class,
    Interface,
    Trait,      // Rust-specific, but keep for compatibility
    Struct,     // Rust-specific
    Module,
    Enum,
    Method,     // For class/interface methods
    Field,      // For struct/class fields
}

#[derive(Debug, Clone)]
pub struct EntityMetadata {
    pub visibility: Visibility,
    pub modifiers: Vec<String>, // public, static, async, etc.
    pub generic_params: Vec<String>,
    pub doc_comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

use std::collections::HashMap;

pub struct ParserRegistry {
    parsers: HashMap<String, Box<dyn LanguageParser>>,
}

impl ParserRegistry {
    pub fn new() -> Self {
        Self {
            parsers: HashMap::new(),
        }
    }

    pub fn register(&mut self, language: &str, parser: Box<dyn LanguageParser>) {
        self.parsers.insert(language.to_string(), parser);
    }

    pub fn get_parser(&self, language: &str) -> Option<&Box<dyn LanguageParser>> {
        self.parsers.get(language)
    }

    pub fn detect_language(&self, file_path: &str) -> Option<&str> {
        for (lang, parser) in &self.parsers {
            for ext in parser.supported_extensions() {
                if file_path.ends_with(ext) {
                    return Some(lang);
                }
            }
        }
        None
    }
}
