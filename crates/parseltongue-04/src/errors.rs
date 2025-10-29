use thiserror::Error;

/// Errors that can occur during validation operations
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Syntax error at line {line}, column {column}: {message}")]
    SyntaxError {
        line: usize,
        column: usize,
        message: String,
        code_snippet: Option<String>,
    },

    #[error("Type error at line {line}, column {column}: expected {expected}, found {found}")]
    TypeError {
        line: usize,
        column: usize,
        expected: String,
        found: String,
        message: String,
    },

    #[error("Borrow checker error at line {line}, column {column}: {message}")]
    BorrowError {
        line: usize,
        column: usize,
        message: String,
        borrow_kind: String,
    },

    #[error("Compilation error: {message}")]
    CompilationError {
        message: String,
        help_text: Option<String>,
        error_code: Option<String>,
    },

    #[error("Test failure: {test_name} - {message}")]
    TestError {
        test_name: String,
        message: String,
        stdout: Option<String>,
        stderr: Option<String>,
    },

    #[error("Validation timeout after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),
}

/// Severity levels for validation errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Hint = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

impl ValidationError {
    /// Get the severity level of this error
    pub fn severity(&self) -> Severity {
        match self {
            Self::SyntaxError { .. }
            | Self::TypeError { .. }
            | Self::BorrowError { .. }
            | Self::CompilationError { .. }
            | Self::TestError { .. } => Severity::Error,
            Self::Timeout { .. } => Severity::Error,
            Self::Io(_) => Severity::Error,
            Self::Parse(_) => Severity::Error,
        }
    }

    /// Get the line number if available
    pub fn line(&self) -> Option<usize> {
        match self {
            Self::SyntaxError { line, .. }
            | Self::TypeError { line, .. }
            | Self::BorrowError { line, .. } => Some(*line),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_severity() {
        let syntax_err = ValidationError::SyntaxError {
            line: 1,
            column: 5,
            message: "unexpected token".to_string(),
            code_snippet: None,
        };
        assert_eq!(syntax_err.severity(), Severity::Error);
    }

    #[test]
    fn test_error_line_extraction() {
        let type_err = ValidationError::TypeError {
            line: 42,
            column: 10,
            expected: "i32".to_string(),
            found: "String".to_string(),
            message: "type mismatch".to_string(),
        };
        assert_eq!(type_err.line(), Some(42));
    }
}
