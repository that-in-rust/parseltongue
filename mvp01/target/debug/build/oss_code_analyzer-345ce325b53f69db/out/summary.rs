#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSummary {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub language: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub loc: u32,
    #[prost(uint32, tag = "4")]
    pub code_lines: u32,
    #[prost(uint32, tag = "5")]
    pub comment_lines: u32,
    #[prost(uint32, tag = "6")]
    pub blank_lines: u32,
    #[prost(uint32, tag = "7")]
    pub function_count: u32,
    #[prost(uint32, tag = "8")]
    pub class_count: u32,
    #[prost(uint32, tag = "9")]
    pub cyclomatic_complexity: u32,
    #[prost(uint32, tag = "10")]
    pub cognitive_complexity: u32,
    #[prost(string, repeated, tag = "11")]
    pub code_smells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, uint32", tag = "12")]
    pub token_frequency: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        u32,
    >,
    #[prost(uint32, tag = "13")]
    pub ast_depth: u32,
    #[prost(uint32, tag = "14")]
    pub ast_node_count: u32,
    #[prost(string, repeated, tag = "15")]
    pub imports: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "16")]
    pub exported_symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "17")]
    pub max_nesting_depth: u32,
    #[prost(float, tag = "18")]
    pub comment_ratio: f32,
    #[prost(message, repeated, tag = "19")]
    pub dependencies: ::prost::alloc::vec::Vec<Dependency>,
    #[prost(uint32, tag = "20")]
    pub halstead_volume: u32,
    #[prost(uint32, tag = "21")]
    pub halstead_difficulty: u32,
    #[prost(uint32, tag = "22")]
    pub halstead_effort: u32,
    #[prost(float, tag = "23")]
    pub maintainability_index: f32,
    #[prost(string, repeated, tag = "24")]
    pub todos: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "25")]
    pub fixmes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "26")]
    pub lint_errors: u32,
    #[prost(uint32, tag = "27")]
    pub lint_warnings: u32,
    #[prost(string, repeated, tag = "28")]
    pub security_vulnerabilities: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dependency {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectSummary {
    #[prost(message, repeated, tag = "1")]
    pub files: ::prost::alloc::vec::Vec<FileSummary>,
    #[prost(uint32, tag = "2")]
    pub total_loc: u32,
    #[prost(map = "string, uint32", tag = "3")]
    pub language_breakdown: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        u32,
    >,
    #[prost(uint32, tag = "4")]
    pub total_files: u32,
    #[prost(uint32, tag = "5")]
    pub total_functions: u32,
    #[prost(uint32, tag = "6")]
    pub total_classes: u32,
    #[prost(float, tag = "7")]
    pub average_cyclomatic_complexity: f32,
    #[prost(float, tag = "8")]
    pub average_cognitive_complexity: f32,
    #[prost(string, repeated, tag = "9")]
    pub project_wide_code_smells: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(uint64, tag = "10")]
    pub timestamp: u64,
    #[prost(message, repeated, tag = "11")]
    pub project_dependencies: ::prost::alloc::vec::Vec<Dependency>,
    #[prost(uint32, tag = "12")]
    pub max_file_size: u32,
    #[prost(uint32, tag = "13")]
    pub min_file_size: u32,
    #[prost(float, tag = "14")]
    pub average_file_size: f32,
    #[prost(uint32, tag = "15")]
    pub max_function_size: u32,
    #[prost(uint32, tag = "16")]
    pub min_function_size: u32,
    #[prost(float, tag = "17")]
    pub average_function_size: f32,
    #[prost(map = "string, uint32", tag = "18")]
    pub file_type_breakdown: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        u32,
    >,
    #[prost(uint32, tag = "19")]
    pub total_unique_tokens: u32,
    #[prost(float, tag = "20")]
    pub code_duplication_percentage: f32,
    #[prost(float, tag = "21")]
    pub overall_maintainability_index: f32,
    #[prost(uint32, tag = "22")]
    pub total_todos: u32,
    #[prost(uint32, tag = "23")]
    pub total_fixmes: u32,
    #[prost(uint32, tag = "24")]
    pub total_lint_errors: u32,
    #[prost(uint32, tag = "25")]
    pub total_lint_warnings: u32,
    #[prost(string, repeated, tag = "26")]
    pub project_wide_security_vulnerabilities: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(float, tag = "27")]
    pub test_coverage_percentage: f32,
    #[prost(uint32, tag = "28")]
    pub total_test_cases: u32,
    #[prost(uint32, tag = "29")]
    pub passed_test_cases: u32,
    #[prost(uint32, tag = "30")]
    pub failed_test_cases: u32,
    #[prost(string, repeated, tag = "31")]
    pub third_party_libraries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, uint32", tag = "32")]
    pub contributor_statistics: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        u32,
    >,
    #[prost(uint64, tag = "33")]
    pub last_commit_timestamp: u64,
    #[prost(uint32, tag = "34")]
    pub total_commits: u32,
    #[prost(float, tag = "35")]
    pub average_commit_frequency: f32,
}
