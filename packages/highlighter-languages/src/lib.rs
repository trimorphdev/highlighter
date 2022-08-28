//! Official language extensions for Highlighter.

use highlighter_core::{Token, Error, highlight};

#[cfg(feature = "brainheck")]
pub mod brainheck;

/// Highlights a language extended from the `highlighter-languages` crate.
pub fn highlight_language(language: &str, src: &str) -> Option<Result<Vec<Token>, Error>> {
    match language.to_lowercase().as_str() {
        #[cfg(feature = "brainheck")]
        "brainheck" => Some(highlight(brainheck::Brainheck, src)),
        _ => None
    }
}