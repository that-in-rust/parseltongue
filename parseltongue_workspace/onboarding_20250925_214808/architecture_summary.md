# Codebase Architecture Summary
Generated: Thu Sep 25 21:48:08 IST 2025
Files analyzed:       63

## Entity Counts
- Functions:        0
- Structs:        0
- Traits:        0

## Key Entry Points
3395:                "fn main()".to_string(),
3467:                hash: SigHash::from_signature("fn main"),
3470:                signature: Arc::from("fn main()"),
3506:        let main_hash = SigHash::from_signature("fn main");
3575:        ("fn main()", 0x51c9_68a4_8c8e_1a7f_u64), // Example - actual values will be computed
3757:    let main_hash = SigHash::from_signature("fn main");
3788:    let main_hash = SigHash::from_signature("fn main");
4349:        "fn main()",
4429:        ("fn main()", NodeKind::Function, "main"),
4461:        ("fn main()", "fn create_user(name: String, age: u32) -> User", EdgeKind::Calls),
66:    pub fn new(workspace_root: PathBuf) -> Self {
609:    pub fn new(username: String, email: String, password_hash: String) -> Self {
705:    pub fn new(title: String, content: String, author_id: Uuid) -> Self {
819:    pub fn new(db: Database) -> Self {
932:    pub fn new(db: Database) -> Self {
1026:    pub fn new(connection_string: String) -> Self {
2513:        pub fn new(workspace_root: PathBuf) -> Self {
2978:    pub fn new(database_url: String) -> Self {
2992:    pub fn new(id: u64, name: String) -> Self {
3017:    pub fn new() -> Self {
3258:                "impl Module{}Struct{} {{\n    pub fn new() -> Self {{\n        Self {{ field: String::new() }}\n    }}\n    \n    pub fn process(&self) -> String {{\n        self.field.clone()\n    }}\n}}\n\n",
3877:    pub fn new(workspace_root: PathBuf) -> Self {
4759:    pub fn new(name: String) -> Self {{
4814:    pub fn new() -> Self {{
6077:    pub fn new(config: Config) -> Self {
7457:    pub fn new(id: u64, name: String, email: String) -> Self {
7474:    pub fn new() -> Self {
7593:    pub fn new() -> Self {
7787:    pub fn new(config: CrossPlatformTestConfig) -> Self {
9355:    pub fn new(baseline_path: &str) -> Self {
818:impl UserService {
870:impl Service for UserService {
931:impl PostService {
983:impl Service for PostService {
3016:impl UserService {
7473:impl UserService {
7592:impl UserService {
28633:                impl<C: Connection> UserService<C> {
30527:            impl UserService {

## Directory Structure
  ./target/release/build/serde_core-6ade472c96897272/out/private.rs
  ./target/release/build/serde-c1c57d176245ab0c/out/private.rs
  ./tests/workspace_manager_tests.rs
  ./tests/end_to_end_workflow_validation.rs
  ./tests/integration_workspace_manager.rs
  ./tests/output_formatter_tdd_tests.rs
  ./tests/workspace_minimal_test.rs
  ./tests/cli_end_to_end_integration.rs
  ./tests/cross_platform_integration.rs
  ./tests/standalone_workspace_test.rs
  ./tests/platform_reference_data.rs
  ./tests/task_23_performance_validation.rs
  ./tests/pt_shell_script_tests.rs
  ./tests/workflow_orchestrator_tdd_tests.rs
  ./tests/system_integration_final_wiring.rs
  ./tests/system_integration_tests.rs
  ./tests/discovery_integration_tests.rs
  ./tests/workspace_integration_test.rs
  ./tests/jtbd_workflow_commands_tests.rs
  ./tests/task_23_simple_validation.rs
