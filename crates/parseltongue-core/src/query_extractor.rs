//! Query-Based Entity Extractor
//!
//! Uses tree-sitter's query system for declarative entity extraction.
//!
//! Performance Contract: <20ms per 1K LOC
//! Memory Contract: <1MB per query file

use std::collections::HashMap;
use std::path::Path;
use anyhow::{Context, Result};
use tree_sitter::{Query, QueryCursor, Tree, Parser};

use crate::entities::{Language, DependencyEdge};

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
    Class,
    Method,
    Typedef,
    Namespace,
}

/// Query-based extractor using .scm query files
pub struct QueryBasedExtractor {
    queries: HashMap<Language, String>,
    parsers: HashMap<Language, Parser>,
}

impl QueryBasedExtractor {
    /// Create new extractor with embedded query files
    pub fn new() -> Result<Self> {
        let mut queries = HashMap::new();

        // Embed query files at compile time
        queries.insert(
            Language::Rust,
            include_str!("../../../entity_queries/rust.scm").to_string()
        );
        queries.insert(
            Language::Python,
            include_str!("../../../entity_queries/python.scm").to_string()
        );
        queries.insert(
            Language::C,
            include_str!("../../../entity_queries/c.scm").to_string()
        );
        queries.insert(
            Language::Cpp,
            include_str!("../../../entity_queries/cpp.scm").to_string()
        );
        queries.insert(
            Language::Ruby,
            include_str!("../../../entity_queries/ruby.scm").to_string()
        );

        // Initialize parsers
        let mut parsers = HashMap::new();
        Self::init_parser(&mut parsers, Language::Rust, &tree_sitter_rust::LANGUAGE.into())?;
        Self::init_parser(&mut parsers, Language::Python, &tree_sitter_python::LANGUAGE.into())?;
        Self::init_parser(&mut parsers, Language::C, &tree_sitter_c::LANGUAGE.into())?;
        Self::init_parser(&mut parsers, Language::Cpp, &tree_sitter_cpp::LANGUAGE.into())?;
        Self::init_parser(&mut parsers, Language::Ruby, &tree_sitter_ruby::LANGUAGE.into())?;

        Ok(Self { queries, parsers })
    }

    fn init_parser(
        parsers: &mut HashMap<Language, Parser>,
        lang: Language,
        grammar: &tree_sitter::Language
    ) -> Result<()> {
        let mut parser = Parser::new();
        parser.set_language(grammar)
            .context(format!("Failed to set language for {:?}", lang))?;
        parsers.insert(lang, parser);
        Ok(())
    }

    /// Parse source code using queries
    pub fn parse_source(
        &mut self,
        source: &str,
        file_path: &Path,
        language: Language,
    ) -> Result<(Vec<ParsedEntity>, Vec<DependencyEdge>)> {
        // Get parser
        let parser = self.parsers.get_mut(&language)
            .context(format!("No parser for language {:?}", language))?;

        // Parse tree
        let tree = parser.parse(source, None)
            .context("Failed to parse source")?;

        // Get query
        let query_source = self.queries.get(&language)
            .context(format!("No query for language {:?}", language))?;

        // Execute query
        let entities = self.execute_query(&tree, source, file_path, language, query_source)?;

        Ok((entities, vec![])) // Dependencies TODO: Phase 3
    }

    fn execute_query(
        &self,
        tree: &Tree,
        source: &str,
        file_path: &Path,
        language: Language,
        query_source: &str,
    ) -> Result<Vec<ParsedEntity>> {
        // Create query
        let ts_lang = self.get_ts_language(language)?;
        let query = Query::new(&ts_lang, query_source)
            .context("Failed to create query")?;

        // Execute query
        let mut cursor = QueryCursor::new();
        let mut entities = Vec::new();

        for m in cursor.matches(&query, tree.root_node(), source.as_bytes()) {
            if let Some(entity) = self.process_match(&m, &query, source, file_path, language) {
                entities.push(entity);
            }
        }

        Ok(entities)
    }

    fn process_match<'a>(
        &self,
        m: &tree_sitter::QueryMatch<'a, 'a>,
        query: &Query,
        source: &str,
        file_path: &Path,
        language: Language,
    ) -> Option<ParsedEntity> {
        let mut entity_name = None;
        let mut entity_type = None;
        let mut node = None;

        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];

            if *capture_name == "name" {
                entity_name = Some(source[capture.node.byte_range()].to_string());
            } else if capture_name.starts_with("definition.") {
                entity_type = self.parse_entity_type(capture_name);
                node = Some(capture.node);
            }
        }

        if let (Some(name), Some(entity_type), Some(node)) = (entity_name, entity_type, node) {
            Some(ParsedEntity {
                entity_type,
                name,
                language,
                line_range: (
                    node.start_position().row + 1,
                    node.end_position().row + 1,
                ),
                file_path: file_path.to_string_lossy().to_string(),
                metadata: HashMap::new(),
            })
        } else {
            None
        }
    }

    fn parse_entity_type(&self, capture_name: &str) -> Option<EntityType> {
        match capture_name {
            "definition.function" => Some(EntityType::Function),
            "definition.struct" => Some(EntityType::Struct),
            "definition.class" => Some(EntityType::Class),
            "definition.enum" => Some(EntityType::Enum),
            "definition.trait" => Some(EntityType::Trait),
            "definition.impl" => Some(EntityType::Impl),
            "definition.module" => Some(EntityType::Module),
            "definition.method" => Some(EntityType::Method),
            "definition.typedef" => Some(EntityType::Typedef),
            "definition.namespace" => Some(EntityType::Namespace),
            _ => None,
        }
    }

    fn get_ts_language(&self, language: Language) -> Result<tree_sitter::Language> {
        Ok(match language {
            Language::Rust => tree_sitter_rust::LANGUAGE.into(),
            Language::Python => tree_sitter_python::LANGUAGE.into(),
            Language::C => tree_sitter_c::LANGUAGE.into(),
            Language::Cpp => tree_sitter_cpp::LANGUAGE.into(),
            Language::Ruby => tree_sitter_ruby::LANGUAGE.into(),
            _ => anyhow::bail!("Unsupported language: {:?}", language),
        })
    }
}
