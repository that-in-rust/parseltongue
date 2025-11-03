// Test file to explore tree-sitter AST structure for complex patterns

use std::collections::HashMap;

struct Config {
    settings: HashMap<String, String>,
}

impl Config {
    fn new() -> Self {
        Self {
            settings: create_defaults(),  // Call within struct construction
        }
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.settings.get(key)  // Method call
    }
}

fn create_defaults() -> HashMap<String, String> {
    HashMap::new()
}

fn main() {
    let config = Config::new();
    println!("{:?}", config.get("test"));  // Macro + method call

    // Chained method calls
    let users = vec![1, 2, 3];
    let result: Vec<i32> = users.iter().map(|x| validate(*x)).collect();

    // Calls in control flow
    if validate(5) {
        process(5)
    } else {
        fallback()
    }

    // Calls within macro
    vec![create_item(), create_item()];
}

fn validate(x: i32) -> bool {
    x > 0
}

fn process(x: i32) -> i32 {
    x * 2
}

fn fallback() -> i32 {
    0
}

fn create_item() -> i32 {
    42
}
