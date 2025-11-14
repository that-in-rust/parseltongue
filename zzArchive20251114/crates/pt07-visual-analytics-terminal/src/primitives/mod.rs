//! Terminal rendering primitives for visual analytics
//!
//! All rendering functions follow 4-word naming convention.
//! Each primitive is responsible for a single visual concern.

pub mod render_box_drawing_unicode;
pub mod render_progress_bar_horizontal;
pub mod render_color_emoji_terminal;

pub use render_box_drawing_unicode::*;
pub use render_progress_bar_horizontal::*;
pub use render_color_emoji_terminal::*;
