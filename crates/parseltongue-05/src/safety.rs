//! Safety checks and validation for file writing operations

use crate::error::FileWriterResult;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Safety levels for file writing operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WriteSafetyLevel {
    /// No safety checks - fastest but most dangerous
    None,
    /// Basic safety checks (file existence, permissions)
    Basic,
    /// Standard safety checks + backup creation
    Standard,
    /// Maximum safety checks + validation + backup + verification
    Strict,
}

impl Default for WriteSafetyLevel {
    fn default() -> Self {
        Self::Standard
    }
}

/// Result of a safety check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteSafetyCheck {
    /// Overall safety status
    pub is_safe: bool,
    /// Safety level used
    pub safety_level: WriteSafetyLevel,
    /// Specific safety issues found
    pub issues: Vec<SafetyIssue>,
    /// Recommendations for safe operation
    pub recommendations: Vec<String>,
    /// Estimated risk level (0.0 - 1.0)
    pub risk_score: f32,
}

/// Individual safety issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyIssue {
    /// Type of safety issue
    pub issue_type: SafetyIssueType,
    /// Description of the issue
    pub description: String,
    /// Severity of the issue
    pub severity: SafetySeverity,
    /// File path affected
    pub path: String,
}

/// Types of safety issues
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SafetyIssueType {
    /// File doesn't exist (for modifications)
    FileNotFound,
    /// Permission denied for read/write
    PermissionDenied,
    /// File is currently locked by another process
    FileLocked,
    /// File is too large for safe operation
    FileTooLarge,
    /// Disk space is low
    LowDiskSpace,
    /// File is in a critical system directory
    SystemFile,
    /// Concurrent modification detected
    ConcurrentModification,
    /// File has unknown encoding
    UnknownEncoding,
}

/// Severity levels for safety issues
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SafetySeverity {
    /// Information only
    Info,
    /// Warning - proceed with caution
    Warning,
    /// Error - operation should not proceed
    Error,
    /// Critical - operation must be aborted
    Critical,
}

impl WriteSafetyCheck {
    /// Create a new safety check
    pub fn new(safety_level: WriteSafetyLevel) -> Self {
        Self {
            is_safe: true,
            safety_level,
            issues: Vec::new(),
            recommendations: Vec::new(),
            risk_score: 0.0,
        }
    }

    /// Add a safety issue
    pub fn add_issue(&mut self, issue: SafetyIssue) {
        self.risk_score = self.risk_score.max(match issue.severity {
            SafetySeverity::Info => 0.1,
            SafetySeverity::Warning => 0.3,
            SafetySeverity::Error => 0.7,
            SafetySeverity::Critical => 1.0,
        });

        if issue.severity >= SafetySeverity::Error {
            self.is_safe = false;
        }

        self.issues.push(issue);
    }

    /// Add a recommendation
    pub fn add_recommendation(&mut self, recommendation: String) {
        self.recommendations.push(recommendation);
    }

    /// Check if operation is safe to proceed
    pub fn is_safe_to_proceed(&self) -> bool {
        self.is_safe && self.risk_score < 0.8
    }
}

/// Trait for performing safety checks
pub trait SafetyChecker: Send + Sync {
    /// Perform safety checks on a file path
    fn check_file_safety(&self, path: &Path) -> FileWriterResult<WriteSafetyCheck>;
}

/// Default safety checker implementation
pub struct DefaultSafetyChecker {
    safety_level: WriteSafetyLevel,
    max_file_size: u64,
    min_disk_space: u64,
}

impl DefaultSafetyChecker {
    /// Create a new safety checker with default settings
    pub fn new() -> Self {
        Self {
            safety_level: WriteSafetyLevel::Standard,
            max_file_size: 100 * 1024 * 1024,   // 100MB
            min_disk_space: 1024 * 1024 * 1024, // 1GB
        }
    }

    /// Get the current safety level
    pub fn safety_level(&self) -> WriteSafetyLevel {
        self.safety_level
    }

    /// Create a safety checker with custom settings
    pub fn with_config(
        safety_level: WriteSafetyLevel,
        max_file_size: u64,
        min_disk_space: u64,
    ) -> Self {
        Self {
            safety_level,
            max_file_size,
            min_disk_space,
        }
    }
}

impl Default for DefaultSafetyChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SafetyChecker for DefaultSafetyChecker {
    fn check_file_safety(&self, path: &Path) -> FileWriterResult<WriteSafetyCheck> {
        let mut check = WriteSafetyCheck::new(self.safety_level);

        // Skip all checks if safety level is None
        if self.safety_level == WriteSafetyLevel::None {
            return Ok(check);
        }

        // Basic safety checks
        if self.safety_level >= WriteSafetyLevel::Basic {
            self.check_file_exists(path, &mut check);
            self.check_permissions(path, &mut check);
        }

        // Standard safety checks
        if self.safety_level >= WriteSafetyLevel::Standard {
            self.check_file_size(path, &mut check);
            self.check_disk_space(path, &mut check);
        }

        // Strict safety checks
        if self.safety_level >= WriteSafetyLevel::Strict {
            self.check_concurrent_modification(path, &mut check);
            self.check_system_file(path, &mut check);
        }

        Ok(check)
    }
}

impl DefaultSafetyChecker {
    fn check_file_exists(&self, path: &Path, check: &mut WriteSafetyCheck) {
        if !path.exists() {
            check.add_issue(SafetyIssue {
                issue_type: SafetyIssueType::FileNotFound,
                description: format!("File does not exist: {}", path.display()),
                severity: SafetySeverity::Error,
                path: path.to_string_lossy().to_string(),
            });
            check.add_recommendation(
                "Create the file first or ensure the path is correct".to_string(),
            );
        }
    }

    fn check_permissions(&self, path: &Path, check: &mut WriteSafetyCheck) {
        // Check read permission
        if path.exists() && !path.is_file() {
            check.add_issue(SafetyIssue {
                issue_type: SafetyIssueType::PermissionDenied,
                description: format!("Path is not a file: {}", path.display()),
                severity: SafetySeverity::Error,
                path: path.to_string_lossy().to_string(),
            });
            return;
        }

        // Check if parent directory is writable
        if let Some(parent) = path.parent() {
            match std::fs::metadata(parent) {
                Ok(metadata) => {
                    if metadata.permissions().readonly() {
                        check.add_issue(SafetyIssue {
                            issue_type: SafetyIssueType::PermissionDenied,
                            description: format!("Directory is not writable: {}", parent.display()),
                            severity: SafetySeverity::Error,
                            path: parent.to_string_lossy().to_string(),
                        });
                    }
                }
                Err(_) => {
                    check.add_issue(SafetyIssue {
                        issue_type: SafetyIssueType::PermissionDenied,
                        description: format!("Cannot access directory: {}", parent.display()),
                        severity: SafetySeverity::Error,
                        path: parent.to_string_lossy().to_string(),
                    });
                }
            }
        }
    }

    fn check_file_size(&self, path: &Path, check: &mut WriteSafetyCheck) {
        if let Ok(metadata) = std::fs::metadata(path) {
            let file_size = metadata.len();
            if file_size > self.max_file_size {
                check.add_issue(SafetyIssue {
                    issue_type: SafetyIssueType::FileTooLarge,
                    description: format!(
                        "File is too large: {} bytes (max: {} bytes)",
                        file_size, self.max_file_size
                    ),
                    severity: SafetySeverity::Error,
                    path: path.to_string_lossy().to_string(),
                });
                check.add_recommendation(
                    "Consider using chunked processing or increase the size limit".to_string(),
                );
            }
        }
    }

    fn check_disk_space(&self, path: &Path, check: &mut WriteSafetyCheck) {
        if let Some(parent) = path.parent() {
            match std::fs::metadata(parent) {
                Ok(_) => {
                    // Simple disk space check (simplified - real implementation would use more sophisticated methods)
                    // For now, just add a recommendation for large files
                    check.add_recommendation(
                        "Ensure sufficient disk space is available for backup creation".to_string(),
                    );
                }
                Err(_) => {
                    check.add_issue(SafetyIssue {
                        issue_type: SafetyIssueType::LowDiskSpace,
                        description: format!("Cannot check disk space for: {}", parent.display()),
                        severity: SafetySeverity::Warning,
                        path: parent.to_string_lossy().to_string(),
                    });
                }
            }
        }
    }

    fn check_concurrent_modification(&self, path: &Path, check: &mut WriteSafetyCheck) {
        // Simple concurrent modification check
        if path.exists() {
            if let Ok(metadata) = std::fs::metadata(path) {
                if let Ok(modified) = metadata.modified() {
                    let now = std::time::SystemTime::now();
                    if let Ok(elapsed) = now.duration_since(modified) {
                        // If file was modified in the last 5 seconds, warn about concurrent modification
                        if elapsed.as_secs() < 5 {
                            check.add_issue(SafetyIssue {
                                issue_type: SafetyIssueType::ConcurrentModification,
                                description: format!(
                                    "File was recently modified: {} ({} seconds ago)",
                                    path.display(),
                                    elapsed.as_secs()
                                ),
                                severity: SafetySeverity::Warning,
                                path: path.to_string_lossy().to_string(),
                            });
                            check.add_recommendation(
                                "Wait a few seconds before proceeding or verify no other processes are modifying this file"
                                    .to_string(),
                            );
                        }
                    }
                }
            }
        }
    }

    fn check_system_file(&self, path: &Path, check: &mut WriteSafetyCheck) {
        let path_str = path.to_string_lossy();

        // Check if file is in common system directories
        let system_prefixes = [
            "/bin/",
            "/sbin/",
            "/usr/bin/",
            "/usr/sbin/",
            "/etc/",
            "/System/",
            "/Windows/",
            "/Program Files/",
        ];

        for prefix in &system_prefixes {
            if path_str.starts_with(prefix) {
                check.add_issue(SafetyIssue {
                    issue_type: SafetyIssueType::SystemFile,
                    description: format!("File is in system directory: {}", path.display()),
                    severity: SafetySeverity::Critical,
                    path: path.to_string_lossy().to_string(),
                });
                check.add_recommendation(
                    "Modifying system files can be dangerous. Proceed with extreme caution."
                        .to_string(),
                );
                break;
            }
        }
    }
}
