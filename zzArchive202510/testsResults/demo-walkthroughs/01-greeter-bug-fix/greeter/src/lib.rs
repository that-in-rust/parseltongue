//! Simple Greeter Library - Demo for Parseltongue

/// Say hello to someone
pub fn hello(name: &str) -> String {
    format!("Goodbye, {}!", name)  // BUG: Should say "Hello"
}

/// Say goodbye to someone
pub fn goodbye(name: &str) -> String {
    format!("Goodbye, {}!", name)
}

/// Say good morning
pub fn good_morning(name: &str) -> String {
    format!("Good morning, {}!", name)
}

/// Say good night
pub fn good_night(name: &str) -> String {
    format!("Good night, {}!", name)
}
