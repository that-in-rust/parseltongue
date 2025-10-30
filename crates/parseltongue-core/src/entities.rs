//! Core entity types for Parseltongue.
//!
//! Defines the fundamental data structures used across all tools,
//! following the CozoDB schema specification and temporal versioning.

use crate::error::{ParseltongError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt;

/// Language identifiers supported by Parseltongue
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Java,
    Cpp,
    Go,
    Ruby,
    Php,
    CSharp,
    Swift,
    Kotlin,
    Scala,
}

impl Language {
    /// Get file extensions associated with this language
    pub fn file_extensions(&self) -> Vec<&'static str> {
        match self {
            Language::Rust => vec!["rs"],
            Language::JavaScript => vec!["js", "jsx"],
            Language::TypeScript => vec!["ts", "tsx"],
            Language::Python => vec!["py"],
            Language::Java => vec!["java"],
            Language::Cpp => vec!["cpp", "cc", "cxx", "c", "h", "hpp"],
            Language::Go => vec!["go"],
            Language::Ruby => vec!["rb"],
            Language::Php => vec!["php"],
            Language::CSharp => vec!["cs"],
            Language::Swift => vec!["swift"],
            Language::Kotlin => vec!["kt", "kts"],
            Language::Scala => vec!["scala", "sc"],
        }
    }

    /// Detect language from file path
    pub fn from_file_path(path: &PathBuf) -> Option<Self> {
        let extension = path.extension()?.to_str()?;

        for language in [
            Language::Rust,
            Language::JavaScript,
            Language::TypeScript,
            Language::Python,
            Language::Java,
            Language::Cpp,
            Language::Go,
            Language::Ruby,
            Language::Php,
            Language::CSharp,
            Language::Swift,
            Language::Kotlin,
            Language::Scala,
        ] {
            if language.file_extensions().contains(&extension) {
                return Some(language);
            }
        }

        None
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::Rust => write!(f, "rust"),
            Language::JavaScript => write!(f, "javascript"),
            Language::TypeScript => write!(f, "typescript"),
            Language::Python => write!(f, "python"),
            Language::Java => write!(f, "java"),
            Language::Cpp => write!(f, "cpp"),
            Language::Go => write!(f, "go"),
            Language::Ruby => write!(f, "ruby"),
            Language::Php => write!(f, "php"),
            Language::CSharp => write!(f, "csharp"),
            Language::Swift => write!(f, "swift"),
            Language::Kotlin => write!(f, "kotlin"),
            Language::Scala => write!(f, "scala"),
        }
    }
}

/// Entity types within the codebase
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntityType {
    Function,
    Method,
    Struct,
    Enum,
    Trait,
    Interface,
    Module,
    ImplBlock {
        trait_name: Option<String>,
        struct_name: String,
    },
    Macro,
    ProcMacro,
    TestFunction,
    Class,
    Variable,
    Constant,
}

/// Temporal action for state transitions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemporalAction {
    Create,
    Edit,
    Delete,
}

/// Alias for backward compatibility
pub type FutureAction = TemporalAction;

impl TemporalAction {
    /// Validate action compatibility with temporal indicators
    pub fn validate_with_indicators(
        &self,
        current_ind: bool,
        future_ind: bool,
    ) -> Result<()> {
        match (current_ind, future_ind, self) {
            (true, false, TemporalAction::Delete) => Ok(()),
            (true, true, TemporalAction::Edit) => Ok(()),
            (false, true, TemporalAction::Create) => Ok(()),
            _ => Err(ParseltongError::TemporalError {
                details: format!(
                    "Invalid temporal combination: current={}, future={}, action={:?}",
                    current_ind, future_ind, self
                ),
            }),
        }
    }
}

/// Temporal state tracking for entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemporalState {
    /// Entity exists in current state
    pub current_ind: bool,
    /// Entity will exist in future state
    pub future_ind: bool,
    /// Action to transition from current to future
    pub future_action: Option<TemporalAction>,
}

impl TemporalState {
    /// Create new initial state (for Tool 1 indexing)
    ///
    /// PRD Spec (P01:96-101): Tool 1 initializes entities as:
    /// - current_ind: 1 (exists in current codebase)
    /// - future_ind: 0 (future state unknown until Tool 2 processes)
    /// - Future_Action: None
    pub fn initial() -> Self {
        Self {
            current_ind: true,
            future_ind: false,  // Future state unknown at index time
            future_action: None,
        }
    }

    /// Create new unchanged state (for entities reviewed by Tool 2)
    ///
    /// Represents: Entity exists in current codebase, LLM decided no changes needed
    pub fn unchanged() -> Self {
        Self {
            current_ind: true,
            future_ind: true,  // Unchanged state exists in both present and future
            future_action: None,
        }
    }

    /// Create new creation state
    pub fn create() -> Self {
        Self {
            current_ind: false,
            future_ind: true,
            future_action: Some(TemporalAction::Create),
        }
    }

    /// Create new edit state
    pub fn edit() -> Self {
        Self {
            current_ind: true,
            future_ind: true,
            future_action: Some(TemporalAction::Edit),
        }
    }

    /// Create new delete state
    pub fn delete() -> Self {
        Self {
            current_ind: true,
            future_ind: false,
            future_action: Some(TemporalAction::Delete),
        }
    }

    /// Validate temporal state consistency
    pub fn validate(&self) -> Result<()> {
        // Cannot have both indicators false
        if !self.current_ind && !self.future_ind {
            return Err(ParseltongError::TemporalError {
                details: "Both current_ind and future_ind cannot be false".to_string(),
            });
        }

        // Validate action compatibility
        if let Some(ref action) = self.future_action {
            action.validate_with_indicators(self.current_ind, self.future_ind)?;
        }

        // If no action, indicators should be the same
        if self.future_action.is_none() && self.current_ind != self.future_ind {
            return Err(ParseltongError::TemporalError {
                details: "Temporal indicators differ but no action specified".to_string(),
            });
        }

        Ok(())
    }

    /// Check if this state represents a change
    pub fn is_changed(&self) -> bool {
        self.future_action.is_some()
    }
}

/// Interface signature for code entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InterfaceSignature {
    /// Type of entity
    pub entity_type: EntityType,
    /// Name of the entity
    pub name: String,
    /// Visibility level
    pub visibility: Visibility,
    /// File path containing this entity
    pub file_path: PathBuf,
    /// Line range where entity is defined
    pub line_range: LineRange,
    /// Module path for this entity
    pub module_path: Vec<String>,
    /// Documentation comment if available
    pub documentation: Option<String>,
    /// Language-specific signature data
    pub language_specific: LanguageSpecificSignature,
}

/// Visibility levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Crate,
    Module,
}

/// Line range in a file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineRange {
    /// Start line (1-based, inclusive)
    pub start: u32,
    /// End line (1-based, inclusive)
    pub end: u32,
}

impl LineRange {
    /// Create new line range
    pub fn new(start: u32, end: u32) -> Result<Self> {
        if start == 0 || end == 0 {
            return Err(ParseltongError::ValidationError {
                field: "line numbers".to_string(),
                expected: "1-based line numbers".to_string(),
                actual: format!("start={}, end={}", start, end),
            });
        }

        if start > end {
            return Err(ParseltongError::ValidationError {
                field: "line range".to_string(),
                expected: "start <= end".to_string(),
                actual: format!("start={}, end={}", start, end),
            });
        }

        Ok(Self { start, end })
    }

    /// Get the span (number of lines)
    pub fn span(&self) -> u32 {
        self.end - self.start + 1
    }

    /// Check if a line is within this range
    pub fn contains(&self, line: u32) -> bool {
        line >= self.start && line <= self.end
    }
}

/// Language-specific signature data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "language")]
pub enum LanguageSpecificSignature {
    #[serde(rename = "rust")]
    Rust(RustSignature),
    #[serde(rename = "javascript")]
    JavaScript(JavascriptSignature),
    #[serde(rename = "typescript")]
    TypeScript(TypeScriptSignature),
    #[serde(rename = "python")]
    Python(PythonSignature),
    #[serde(rename = "java")]
    Java(JavaSignature),
}

/// Rust-specific signature
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RustSignature {
    /// Generic parameters
    pub generics: Vec<String>,
    /// Lifetime parameters
    pub lifetimes: Vec<String>,
    /// Where clauses
    pub where_clauses: Vec<String>,
    /// Attributes
    pub attributes: Vec<String>,
    /// Trait implementations if this is an impl block
    pub trait_impl: Option<TraitImpl>,
}

/// Trait implementation information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TraitImpl {
    /// Trait being implemented
    pub trait_name: String,
    /// Type implementing the trait
    pub for_type: String,
}

/// JavaScript-specific signature
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JavascriptSignature {
    /// Function parameters
    pub parameters: Vec<Parameter>,
    /// Return type annotation (if available)
    pub return_type: Option<String>,
    /// Async function
    pub is_async: bool,
    /// Arrow function
    pub is_arrow: bool,
}

/// TypeScript-specific signature
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeScriptSignature {
    /// Function parameters with types
    pub parameters: Vec<TypedParameter>,
    /// Return type
    pub return_type: Option<String>,
    /// Generic parameters
    pub generics: Vec<String>,
    /// Async function
    pub is_async: bool,
}

/// Python-specific signature
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PythonSignature {
    /// Function parameters
    pub parameters: Vec<PythonParameter>,
    /// Return type annotation
    pub return_type: Option<String>,
    /// Async function
    pub is_async: bool,
    /// Decorators
    pub decorators: Vec<String>,
}

/// Java-specific signature
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JavaSignature {
    /// Access modifier
    pub access_modifier: AccessModifier,
    /// Method parameters with types
    pub parameters: Vec<JavaParameter>,
    /// Return type
    pub return_type: String,
    /// Exception types thrown
    pub throws: Vec<String>,
    /// Static method
    pub is_static: bool,
    /// Generic parameters
    pub generics: Vec<String>,
}

/// Parameter types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypedParameter {
    pub name: String,
    pub type_annotation: String,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PythonParameter {
    pub name: String,
    pub type_annotation: Option<String>,
    pub default_value: Option<String>,
    pub is_varargs: bool,
    pub is_kwargs: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JavaParameter {
    pub name: String,
    pub type_annotation: String,
    pub is_varargs: bool,
}

/// Access modifiers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccessModifier {
    Public,
    Private,
    Protected,
    Package,
}

/// Core code entity with temporal versioning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeEntity {
    /// Unique ISGL1 key
    pub isgl1_key: String,

    /// Temporal state
    pub temporal_state: TemporalState,

    /// Interface signature
    pub interface_signature: InterfaceSignature,

    /// Current code content
    pub current_code: Option<String>,

    /// Future code content
    pub future_code: Option<String>,

    /// TDD classification
    pub tdd_classification: TddClassification,

    /// LSP metadata (Rust-enhanced)
    pub lsp_metadata: Option<LspMetadata>,

    /// Entity metadata
    pub metadata: EntityMetadata,
}

/// Entity classification for TDD workflow
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntityClass {
    /// Test implementation (unit tests, integration tests, etc.)
    TestImplementation,
    /// Production code implementation
    CodeImplementation,
}

impl fmt::Display for EntityClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntityClass::TestImplementation => write!(f, "TEST"),
            EntityClass::CodeImplementation => write!(f, "CODE"),
        }
    }
}

/// TDD classification for test-driven development
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TddClassification {
    /// Entity classification (test vs production code)
    pub entity_class: EntityClass,
    /// Testability level
    pub testability: TestabilityLevel,
    /// Complexity assessment
    pub complexity: ComplexityLevel,
    /// Number of dependencies
    pub dependencies: usize,
    /// Estimated test coverage
    pub test_coverage_estimate: f64,
    /// Whether this is on critical path
    pub critical_path: bool,
    /// Change risk assessment
    pub change_risk: RiskLevel,
}

/// Testability levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TestabilityLevel {
    High,
    Medium,
    Low,
}

/// Complexity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
}

/// Risk levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

/// LSP metadata from rust-analyzer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LspMetadata {
    /// Type information
    pub type_information: TypeInformation,
    /// Usage analysis
    pub usage_analysis: UsageAnalysis,
    /// Semantic tokens
    pub semantic_tokens: Vec<SemanticToken>,
}

/// Type information from LSP
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeInformation {
    /// Resolved type
    pub resolved_type: String,
    /// Module path
    pub module_path: Vec<String>,
    /// Generic parameters
    pub generic_parameters: Vec<String>,
    /// Definition location
    pub definition_location: Option<Location>,
}

/// Usage analysis from LSP
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsageAnalysis {
    /// Total references
    pub total_references: usize,
    /// Usage locations
    pub usage_locations: Vec<Location>,
    /// Dependent entities
    pub dependents: Vec<String>,
}

/// Location in code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub file_path: PathBuf,
    pub line: u32,
    pub character: u32,
}

/// Semantic token from LSP
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SemanticToken {
    pub position: Location,
    pub length: u32,
    pub token_type: String,
    pub modifiers: Vec<String>,
}

/// Entity metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityMetadata {
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modification timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Hash of entity content
    pub content_hash: String,
    /// Additional key-value metadata
    pub additional: HashMap<String, String>,
}

impl CodeEntity {
    /// Create new entity (for Tool 1 indexing)
    ///
    /// Initializes with TemporalState::initial() per PRD:
    /// - current_ind: true (exists in current codebase)
    /// - future_ind: false (future state unknown until Tool 2)
    /// - Future_Action: None
    pub fn new(
        isgl1_key: String,
        interface_signature: InterfaceSignature,
    ) -> Result<Self> {
        let entity = Self {
            temporal_state: TemporalState::initial(),  // Tool 1 initial state: (1,0,None)
            interface_signature,
            current_code: None,
            future_code: None,
            tdd_classification: TddClassification::default(),
            lsp_metadata: None,
            metadata: EntityMetadata::new()?,
            isgl1_key,
        };

        Ok(entity)
    }

    /// Apply temporal change
    pub fn apply_temporal_change(
        &mut self,
        action: TemporalAction,
        future_code: Option<String>,
    ) -> Result<()> {
        match action {
            TemporalAction::Create => {
                self.temporal_state = TemporalState::create();
                self.future_code = future_code;
            }
            TemporalAction::Edit => {
                self.temporal_state = TemporalState::edit();
                self.future_code = future_code;
            }
            TemporalAction::Delete => {
                self.temporal_state = TemporalState::delete();
                self.future_code = None;
            }
        }

        self.temporal_state.validate()?;
        Ok(())
    }

    /// Check if entity is modified
    pub fn is_modified(&self) -> bool {
        self.temporal_state.is_changed()
    }

    /// Get effective code (current or future based on state)
    pub fn effective_code(&self) -> Option<&String> {
        if self.temporal_state.future_action.is_some() {
            self.future_code.as_ref()
        } else {
            self.current_code.as_ref()
        }
    }

    /// Validate entity consistency
    pub fn validate(&self) -> Result<()> {
        // Validate ISGL1 key format
        self.validate_isgl1_key()?;

        // Validate temporal state
        self.temporal_state.validate()?;

        // Validate line range
        LineRange::new(self.interface_signature.line_range.start, self.interface_signature.line_range.end)
            .map_err(|e| ParseltongError::ValidationError {
                field: "line_range".to_string(),
                expected: "valid line range".to_string(),
                actual: e.to_string(),
            })?;

        // Validate code consistency
        self.validate_code_consistency()?;

        Ok(())
    }

    fn validate_isgl1_key(&self) -> Result<()> {
        if self.isgl1_key.is_empty() {
            return Err(ParseltongError::InvalidIsgl1Key {
                key: self.isgl1_key.clone(),
                reason: "ISGL1 key cannot be empty".to_string(),
            });
        }

        if !self.isgl1_key.contains('-') {
            return Err(ParseltongError::InvalidIsgl1Key {
                key: self.isgl1_key.clone(),
                reason: "ISGL1 key must contain hyphens".to_string(),
            });
        }

        Ok(())
    }

    fn validate_code_consistency(&self) -> Result<()> {
        // If entity exists in current state, it should have current code
        if self.temporal_state.current_ind && self.current_code.is_none() {
            return Err(ParseltongError::ValidationError {
                field: "current_code".to_string(),
                expected: "present when current_ind is true".to_string(),
                actual: "None".to_string(),
            });
        }

        // If entity will exist in future state, it should have future code
        if self.temporal_state.future_ind && self.future_code.is_none() {
            return Err(ParseltongError::ValidationError {
                field: "future_code".to_string(),
                expected: "present when future_ind is true".to_string(),
                actual: "None".to_string(),
            });
        }

        Ok(())
    }

    /// Generate hash-based ISGL1 key for new entities
    ///
    /// Creates stable identity keys for entities that don't exist yet in the codebase.
    /// Uses SHA-256 hash to ensure uniqueness and collision avoidance.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file where entity will be created
    /// * `entity_name` - Name of the entity (function, struct, etc.)
    /// * `entity_type` - Type of entity (Function, Struct, Enum, etc.)
    /// * `timestamp` - Creation timestamp for uniqueness
    ///
    /// # Returns
    ///
    /// ISGL1 key in format: `{sanitized_filepath}-{entity_name}-{type_abbrev}-{hash8}`
    ///
    /// # Example
    ///
    /// ```
    /// use parseltongue_core::entities::{CodeEntity, EntityType};
    /// use chrono::Utc;
    ///
    /// let key = CodeEntity::generate_new_entity_key(
    ///     "src/lib.rs",
    ///     "new_feature",
    ///     &EntityType::Function,
    ///     Utc::now()
    /// );
    /// // Returns: "src_lib_rs-new_feature-fn-abc12345"
    /// ```
    pub fn generate_new_entity_key(
        file_path: &str,
        entity_name: &str,
        entity_type: &EntityType,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> String {
        use sha2::{Sha256, Digest};

        // Sanitize file path: replace /, \, and . with _
        let sanitized_path = file_path
            .replace('/', "_")
            .replace('\\', "_")
            .replace('.', "_");

        // Get type abbreviation
        let type_abbrev = match entity_type {
            EntityType::Function => "fn",
            EntityType::Method => "method",
            EntityType::Struct => "struct",
            EntityType::Enum => "enum",
            EntityType::Trait => "trait",
            EntityType::Interface => "interface",
            EntityType::Module => "mod",
            EntityType::ImplBlock { .. } => "impl",
            EntityType::Macro => "macro",
            EntityType::ProcMacro => "proc_macro",
            EntityType::TestFunction => "test",
            EntityType::Class => "class",
            EntityType::Variable => "var",
            EntityType::Constant => "const",
        };

        // Create hash input: filepath + name + type + timestamp
        let mut hasher = Sha256::new();
        hasher.update(file_path.as_bytes());
        hasher.update(entity_name.as_bytes());
        hasher.update(format!("{:?}", entity_type).as_bytes());
        hasher.update(timestamp.to_rfc3339().as_bytes());

        // Get hash result and take first 8 characters
        let hash_bytes = hasher.finalize();
        let hash_str = format!("{:x}", hash_bytes);
        let short_hash = &hash_str[0..8];

        // Format: sanitized_path-entity_name-type_abbrev-hash8
        format!("{}-{}-{}-{}", sanitized_path, entity_name, type_abbrev, short_hash)
    }
}

impl Default for TddClassification {
    fn default() -> Self {
        Self {
            entity_class: EntityClass::CodeImplementation,
            testability: TestabilityLevel::Medium,
            complexity: ComplexityLevel::Simple,
            dependencies: 0,
            test_coverage_estimate: 0.0,
            critical_path: false,
            change_risk: RiskLevel::Medium,
        }
    }
}

impl EntityMetadata {
    pub fn new() -> Result<Self> {
        Ok(Self {
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
            content_hash: String::new(), // Will be set when content is available
            additional: HashMap::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temporal_state_validation() {
        let state = TemporalState::unchanged();
        assert!(state.validate().is_ok());
        assert!(!state.is_changed());

        let edit_state = TemporalState::edit();
        assert!(edit_state.validate().is_ok());
        assert!(edit_state.is_changed());
    }

    #[test]
    fn invalid_temporal_state() {
        let invalid_state = TemporalState {
            current_ind: false,
            future_ind: false,
            future_action: None,
        };

        assert!(invalid_state.validate().is_err());
    }

    #[test]
    fn line_range_validation() {
        let valid_range = LineRange::new(1, 5).unwrap();
        assert_eq!(valid_range.span(), 5);
        assert!(valid_range.contains(3));
        assert!(!valid_range.contains(6));

        // Invalid range (start > end)
        assert!(LineRange::new(10, 5).is_err());

        // Invalid range (zero-based)
        assert!(LineRange::new(0, 5).is_err());
    }

    #[test]
    fn language_detection() {
        let rust_path = PathBuf::from("src/main.rs");
        assert_eq!(Language::from_file_path(&rust_path), Some(Language::Rust));

        let js_path = PathBuf::from("app.js");
        assert_eq!(Language::from_file_path(&js_path), Some(Language::JavaScript));

        let unknown_path = PathBuf::from("file.xyz");
        assert_eq!(Language::from_file_path(&unknown_path), None);
    }

    #[test]
    fn code_entity_validation() {
        let mut entity = CodeEntity::new(
            "src/main.rs-main-main".to_string(),
            InterfaceSignature {
                entity_type: EntityType::Function,
                name: "main".to_string(),
                visibility: Visibility::Public,
                file_path: PathBuf::from("src/main.rs"),
                line_range: LineRange::new(1, 10).unwrap(),
                module_path: vec!["main".to_string()],
                documentation: None,
                language_specific: LanguageSpecificSignature::Rust(RustSignature {
                    generics: vec![],
                    lifetimes: vec![],
                    where_clauses: vec![],
                    attributes: vec![],
                    trait_impl: None,
                }),
            },
        ).unwrap();

        // Set current_code and future_code to satisfy validation requirements
        entity.current_code = Some("fn main() { println!(\"Hello, world!\"); }".to_string());
        entity.future_code = Some("fn main() { println!(\"Hello, world!\"); }".to_string());

        match entity.validate() {
            Ok(()) => (),
            Err(e) => {
                println!("Validation error: {:?}", e);
                panic!("Entity validation failed: {:?}", e);
            }
        }

        // Test temporal change
        entity.apply_temporal_change(
            TemporalAction::Edit,
            Some("fn main() { println!(\"Hello\"); }".to_string()),
        ).unwrap();

        assert!(entity.is_modified());
        assert!(entity.effective_code().is_some());
    }

    #[test]
    fn test_generate_new_entity_key_basic() {
        use chrono::TimeZone;

        let timestamp = chrono::Utc.with_ymd_and_hms(2025, 10, 30, 12, 0, 0).unwrap();
        let key = CodeEntity::generate_new_entity_key(
            "src/lib.rs",
            "new_feature",
            &EntityType::Function,
            timestamp
        );

        // Should follow format: filepath-name-type-hash8
        assert!(key.contains("src_lib_rs"));
        assert!(key.contains("new_feature"));
        assert!(key.contains("-fn-"));

        // Hash should be 8 characters
        let parts: Vec<&str> = key.split('-').collect();
        assert!(parts.len() >= 4, "Key should have at least 4 parts separated by hyphens");
        let hash_part = parts.last().unwrap();
        assert_eq!(hash_part.len(), 8, "Hash should be exactly 8 characters");
    }

    #[test]
    fn test_generate_new_entity_key_different_types() {
        use chrono::TimeZone;

        let timestamp = chrono::Utc.with_ymd_and_hms(2025, 10, 30, 12, 0, 0).unwrap();

        // Test Function type
        let fn_key = CodeEntity::generate_new_entity_key(
            "src/lib.rs",
            "test_fn",
            &EntityType::Function,
            timestamp
        );
        assert!(fn_key.contains("-fn-"));

        // Test Struct type
        let struct_key = CodeEntity::generate_new_entity_key(
            "src/lib.rs",
            "TestStruct",
            &EntityType::Struct,
            timestamp
        );
        assert!(struct_key.contains("-struct-"));

        // Test Enum type
        let enum_key = CodeEntity::generate_new_entity_key(
            "src/lib.rs",
            "TestEnum",
            &EntityType::Enum,
            timestamp
        );
        assert!(enum_key.contains("-enum-"));

        // Test Trait type
        let trait_key = CodeEntity::generate_new_entity_key(
            "src/lib.rs",
            "TestTrait",
            &EntityType::Trait,
            timestamp
        );
        assert!(trait_key.contains("-trait-"));

        // Test Module type
        let mod_key = CodeEntity::generate_new_entity_key(
            "src/lib.rs",
            "test_module",
            &EntityType::Module,
            timestamp
        );
        assert!(mod_key.contains("-mod-"));
    }

    #[test]
    fn test_generate_new_entity_key_path_sanitization() {
        use chrono::TimeZone;

        let timestamp = chrono::Utc.with_ymd_and_hms(2025, 10, 30, 12, 0, 0).unwrap();

        // Test forward slashes
        let key1 = CodeEntity::generate_new_entity_key(
            "src/models/user.rs",
            "UserProfile",
            &EntityType::Struct,
            timestamp
        );
        assert!(key1.contains("src_models_user_rs"));
        assert!(!key1.contains('/'));

        // Test dots in filename
        assert!(key1.contains("_rs"));
        assert!(!key1.contains(".rs"));

        // Test backslashes (Windows paths)
        let key2 = CodeEntity::generate_new_entity_key(
            "src\\models\\user.rs",
            "UserProfile",
            &EntityType::Struct,
            timestamp
        );
        assert!(key2.contains("src_models_user_rs"));
        assert!(!key2.contains('\\'));
    }

    #[test]
    fn test_generate_new_entity_key_uniqueness() {
        use chrono::TimeZone;

        // Same inputs but different timestamps should produce different keys
        let timestamp1 = chrono::Utc.with_ymd_and_hms(2025, 10, 30, 12, 0, 0).unwrap();
        let timestamp2 = chrono::Utc.with_ymd_and_hms(2025, 10, 30, 12, 1, 0).unwrap();

        let key1 = CodeEntity::generate_new_entity_key(
            "src/lib.rs",
            "new_feature",
            &EntityType::Function,
            timestamp1
        );

        let key2 = CodeEntity::generate_new_entity_key(
            "src/lib.rs",
            "new_feature",
            &EntityType::Function,
            timestamp2
        );

        assert_ne!(key1, key2, "Different timestamps should produce different keys");

        // Extract hash parts to verify they're different
        let hash1 = key1.split('-').last().unwrap();
        let hash2 = key2.split('-').last().unwrap();
        assert_ne!(hash1, hash2, "Hash parts should be different");
    }

    #[test]
    fn test_generate_new_entity_key_format() {
        use chrono::TimeZone;

        let timestamp = chrono::Utc.with_ymd_and_hms(2025, 10, 30, 12, 0, 0).unwrap();
        let key = CodeEntity::generate_new_entity_key(
            "src/models/user.rs",
            "UserProfile",
            &EntityType::Struct,
            timestamp
        );

        // Expected format: src_models_user_rs-UserProfile-struct-abc12345
        let parts: Vec<&str> = key.split('-').collect();

        // Should have exactly 4 parts: path, name, type, hash
        assert_eq!(parts.len(), 4, "Key should have exactly 4 hyphen-separated parts");

        // Verify each part
        assert_eq!(parts[0], "src_models_user_rs");
        assert_eq!(parts[1], "UserProfile");
        assert_eq!(parts[2], "struct");
        assert_eq!(parts[3].len(), 8, "Hash should be 8 characters");

        // Hash should be lowercase hexadecimal
        assert!(parts[3].chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
    }

    #[test]
    fn test_generate_new_entity_key_impl_block() {
        use chrono::TimeZone;

        let timestamp = chrono::Utc.with_ymd_and_hms(2025, 10, 30, 12, 0, 0).unwrap();

        // Test ImplBlock type (should default to "impl")
        let impl_key = CodeEntity::generate_new_entity_key(
            "src/lib.rs",
            "MyStruct",
            &EntityType::ImplBlock {
                trait_name: Some("Display".to_string()),
                struct_name: "MyStruct".to_string(),
            },
            timestamp
        );
        assert!(impl_key.contains("-impl-"));
    }

    #[test]
    fn test_entity_class_enum() {
        // Test that EntityClass enum exists with correct variants
        let test_class = EntityClass::TestImplementation;
        let code_class = EntityClass::CodeImplementation;

        assert_eq!(test_class, EntityClass::TestImplementation);
        assert_eq!(code_class, EntityClass::CodeImplementation);
    }

    #[test]
    fn test_tdd_classification_has_entity_class_field() {
        // Test that TddClassification has entity_class field
        let tdd = TddClassification::default();

        // Default should be CodeImplementation
        assert_eq!(tdd.entity_class, EntityClass::CodeImplementation);
    }

    #[test]
    fn test_entity_class_serialization() {
        // Test that EntityClass can be serialized/deserialized
        let test_impl = EntityClass::TestImplementation;
        let json = serde_json::to_string(&test_impl).unwrap();
        let deserialized: EntityClass = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, EntityClass::TestImplementation);
    }
}