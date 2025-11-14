//! Parseltongue Tool 02: LLM-to-cozoDB-writer
//!
//! Ultra-minimalist tool for writing temporal code changes to CozoDB.
//! Receives manual temporal changes via CLI (from external LLM) and writes them to database.
//! Following S01 principles: NO automatic LLM calls, direct temporal state updates only.
//!
//! ## S01 Implementation (v0.7.1+)
//!
//! The ultra-minimalist implementation (see main.rs):
//! - Uses parseltongue-core::storage::CozoDbStorage directly
//! - NO LLM client infrastructure (deleted in v0.7.1)
//! - NO batch processing
//! - Direct temporal state updates only

#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![allow(missing_docs)]

pub mod cli;
pub mod errors;

// Re-export commonly used types
pub use errors::*;

/// L1 Core Type: Entity modification actions
///
/// Represents the three fundamental temporal state transitions in CozoDB:
/// - Create: (0, 1, "Create") - Entity doesn't exist yet, will exist after
/// - Edit: (1, 1, "Edit") - Entity exists now, will exist after (modified)
/// - Delete: (1, 0, "Delete") - Entity exists now, won't exist after
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityAction {
    Create,
    Edit,
    Delete,
}

impl EntityAction {
    /// Convert action to temporal state tuple (pure function)
    ///
    /// Returns: (current_ind, future_ind, action_string)
    ///
    /// # Examples
    /// ```ignore
    /// let (curr, fut, action) = EntityAction::Create.to_temporal_state();
    /// assert_eq!((curr, fut, action), ("false", "true", "Create"));
    /// ```
    pub const fn to_temporal_state(self) -> (&'static str, &'static str, &'static str) {
        match self {
            EntityAction::Create => ("false", "true", "Create"),
            EntityAction::Edit => ("true", "true", "Edit"),
            EntityAction::Delete => ("true", "false", "Delete"),
        }
    }
}

/// L1 Core Type: Simple interface configuration
#[derive(Debug, Clone)]
pub struct SimpleUpdateConfig {
    pub entity_key: String,
    pub action: EntityAction,
    pub future_code: Option<String>,
    pub db_path: String,
}

impl SimpleUpdateConfig {
    /// L3: Generate Datalog :put query from simple interface
    ///
    /// Following S01 Executable Specifications:
    /// - Create: (current_ind=0, future_ind=1, Future_Action="Create")
    /// - Edit: (current_ind=1, future_ind=1, Future_Action="Edit")
    /// - Delete: (current_ind=1, future_ind=0, Future_Action="Delete")
    ///
    /// # Preconditions
    /// - Create/Edit actions require `future_code` to be Some(_)
    /// - Delete action allows `future_code` to be None
    ///
    /// # Returns
    /// Valid CozoDB Datalog query string
    ///
    /// # Panics
    /// Panics if Create/Edit action is called without future_code.
    /// This is a precondition violation (contract-based programming).
    ///
    /// # Examples
    /// ```ignore
    /// let config = SimpleUpdateConfig {
    ///     entity_key: "rust:fn:hello:lib_rs:1-5".to_string(),
    ///     action: EntityAction::Edit,
    ///     future_code: Some("fn hello() {}".to_string()),
    ///     db_path: "test.db".to_string(),
    /// };
    /// let datalog = config.to_datalog();
    /// ```
    pub fn to_datalog(&self) -> String {
        // Precondition validation (contract-based programming)
        // Using panic! is intentional - this is a programming error, not a runtime error
        self.validate_preconditions();

        let (current_ind, future_ind, action_str) = self.action.to_temporal_state();
        let future_code_value = self.escape_future_code();

        // Generate Datalog matching actual CodeGraph schema (14 fields - includes entity_class)
        // Note: ISGL1_key => indicates primary key in :put syntax
        format!(
            r#"?[ISGL1_key, Current_Code, Future_Code, interface_signature, TDD_Classification,
              lsp_meta_data, current_ind, future_ind, Future_Action, file_path, language,
              last_modified, entity_type, entity_class] <-
            [["{}", null, {}, "", "",
              null, {}, {}, "{}", "",
              "", "", "", "CODE"]]

            :put CodeGraph {{
                ISGL1_key =>
                Current_Code, Future_Code, interface_signature, TDD_Classification,
                lsp_meta_data, current_ind, future_ind, Future_Action, file_path, language,
                last_modified, entity_type, entity_class
            }}"#,
            self.entity_key, future_code_value, current_ind, future_ind, action_str
        )
    }

    /// Validate preconditions (pure function - no side effects)
    fn validate_preconditions(&self) {
        match self.action {
            EntityAction::Create | EntityAction::Edit if self.future_code.is_none() => {
                panic!("{:?} action requires future_code", self.action);
            }
            _ => {}
        }
    }

    /// Escape future_code for Datalog (pure function)
    fn escape_future_code(&self) -> String {
        match &self.future_code {
            Some(code) => {
                // Properly escape quotes and backslashes for Datalog string literals
                let escaped = code
                    .replace('\\', "\\\\")  // Escape backslashes first
                    .replace('"', "\\\"");   // Then escape quotes
                format!("\"{}\"", escaped)
            }
            None => "null".to_string(),
        }
    }
}

/// L2: Interface mode (Progressive Disclosure pattern)
#[derive(Debug, Clone)]
pub enum InterfaceMode {
    Simple(SimpleUpdateConfig),
    Advanced(AdvancedQueryConfig),
}

/// Advanced query interface configuration
#[derive(Debug, Clone)]
pub struct AdvancedQueryConfig {
    pub query: String,
    pub db_path: String,
}

/// Tool configuration (S01 Ultra-Minimalist)
///
/// Deprecated: Use InterfaceMode instead
/// Kept for backward compatibility with existing code
#[derive(Debug, Clone)]
pub struct LlmWriterConfig {
    /// Datalog query to execute
    pub query: String,
    /// Database connection string
    pub db_path: String,
}

impl Default for LlmWriterConfig {
    fn default() -> Self {
        Self {
            query: String::new(),
            db_path: "parseltongue.db".to_string(),
        }
    }
}