//! Simple Calculator Library - Demo for Parseltongue
//!
//! This library contains a deliberate bug for demonstration purposes.
//! The `add` function incorrectly uses subtraction instead of addition.

/// Add two numbers together
///
/// # Examples
///
/// ```
/// use calculator::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a - b  // BUG: Should be a + b
}

/// Subtract two numbers
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiply two numbers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divide two numbers
///
/// # Panics
///
/// Panics if divisor is zero
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero");
    }
    a / b
}

/// Calculate factorial
pub fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => n * factorial(n - 1),
    }
}

/// Calculate power
pub fn power(base: i32, exp: u32) -> i32 {
    base.pow(exp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);  // This will FAIL due to bug
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(0, 0), 0);
        assert_eq!(subtract(-1, 1), -2);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(multiply(0, 5), 0);
        assert_eq!(multiply(-2, 3), -6);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6, 2), 3);
        assert_eq!(divide(5, 2), 2);
        assert_eq!(divide(-6, 2), -3);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_divide_by_zero() {
        divide(5, 0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_power() {
        assert_eq!(power(2, 3), 8);
        assert_eq!(power(5, 0), 1);
        assert_eq!(power(-2, 3), -8);
    }
}
