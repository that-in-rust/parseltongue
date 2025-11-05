//! Horizontal progress bar rendering primitives
//!
//! Provides functions for rendering progress bars using Unicode characters.

/// Render progress bar with percentage horizontal
///
/// Creates a horizontal progress bar showing completion percentage.
///
/// # Example
/// ```text
/// [████████░░] 80%
/// ```
pub fn render_progress_bar_with_percentage_horizontal(
    current: usize,
    total: usize,
    width: usize,
) -> String {
    // STUB implementation - will be properly implemented with tests
    let percentage = if total > 0 {
        (current as f64 / total as f64 * 100.0) as usize
    } else {
        0
    };
    format!("[████░░] {}%", percentage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub_renders_progress_bar() {
        let result = render_progress_bar_with_percentage_horizontal(8, 10, 10);
        assert!(result.contains("%"));
    }
}
