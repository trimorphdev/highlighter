//! An extendable syntax highlighter written in Rust.

pub use highlighter_core as core;
pub use highlighter_core::{highlight, Token};

pub use highlighter_languages as languages;
pub use highlighter_languages::{highlight_language, language};

#[cfg(feature = "brainheck")]
pub use highlighter_languages::brainheck::Brainheck;

#[cfg(feature = "target-html")]
pub use highlighter_target_html::HighlighterTargetHtml;

