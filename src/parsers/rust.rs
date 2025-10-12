use crate::language_traits::{LanguageParser, Entity, EntityKind, EntityMetadata, Visibility, ParseError, Dependency, DependencyKind};
use syn::{Item, ItemFn, ItemStruct, ItemTrait, ItemImpl};
use quote::quote;

pub struct RustParser;

impl LanguageParser for RustParser {
    fn parse_file(&self, file_path: &str, code: &str) -> Result<Vec<Entity>, ParseError> {
        let mut entities = Vec::new();

        let syntax_tree = match syn::parse_file(code) {
            Ok(tree) => tree,
            Err(e) => {
                return Err(ParseError {
                    message: format!("Parse error: {}", e),
                    file: file_path.to_string(),
                    line: None,
                });
            }
        };

        for item in &syntax_tree.items {
            match item {
                Item::Fn(item_fn) => {
                    let name = item_fn.sig.ident.to_string();
                    let signature = format!("fn {}", quote!(#item_fn.sig));
                    let full_signature = signature.clone();

                    entities.push(Entity {
                        kind: EntityKind::Function,
                        name,
                        signature,
                        full_signature,
                        file_path: file_path.to_string(),
                        line: 0, // TODO: Extract actual line number from syn
                        column: 0,
                        metadata: EntityMetadata {
                            visibility: Visibility::Public, // Default for Rust functions
                            modifiers: vec!["fn".to_string()],
                            generic_params: Vec::new(),
                            doc_comment: None,
                        },
                    });
                }

                Item::Struct(item_struct) => {
                    let name = item_struct.ident.to_string();
                    let signature = format!("struct {}", name);
                    let full_signature = signature.clone();

                    entities.push(Entity {
                        kind: EntityKind::Struct,
                        name,
                        signature,
                        full_signature,
                        file_path: file_path.to_string(),
                        line: 0,
                        column: 0,
                        metadata: EntityMetadata {
                            visibility: Visibility::Public,
                            modifiers: vec!["struct".to_string()],
                            generic_params: Vec::new(),
                            doc_comment: None,
                        },
                    });
                }

                Item::Trait(item_trait) => {
                    let name = item_trait.ident.to_string();
                    let signature = format!("trait {}", name);
                    let full_signature = signature.clone();

                    entities.push(Entity {
                        kind: EntityKind::Trait,
                        name,
                        signature,
                        full_signature,
                        file_path: file_path.to_string(),
                        line: 0,
                        column: 0,
                        metadata: EntityMetadata {
                            visibility: Visibility::Public,
                            modifiers: vec!["trait".to_string()],
                            generic_params: Vec::new(),
                            doc_comment: None,
                        },
                    });
                }

                _ => {
                    // Ignore other items for MVP
                }
            }
        }

        Ok(entities)
    }

    fn supported_extensions(&self) -> Vec<&'static str> {
        vec![".rs"]
    }

    fn language_name(&self) -> &'static str {
        "rust"
    }

    fn extract_dependencies(&self, entities: &[Entity]) -> Vec<Dependency> {
        let mut deps = Vec::new();

        // Find trait implementations using syn AST analysis
        // For now, we'll use a simpler approach since we don't have the full AST
        // In a production system, we'd want to enhance this

        deps
    }
}
