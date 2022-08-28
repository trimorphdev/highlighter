//! Official language extensions for Highlighter.

use highlighter_core::{Token, Error, highlight, language::Language};

#[cfg(feature = "brainheck")]
pub mod brainheck;

/// Returns the language with the specified name, if any.
pub fn language(name: &str) -> Option<Box<dyn Language>> {
    match name.to_lowercase().as_str() {
        #[cfg(feature = "brainheck")]
        "brainheck" => Some(Box::new(brainheck::Brainheck)),
        _ => None
    }
}

/// Highlights a language extended from the `highlighter-languages` crate.
pub fn highlight_language(language: &str, src: &str) -> Option<Result<Vec<Token>, Error>> {
    match language.to_lowercase().as_str() {
        #[cfg(feature = "brainheck")]
        "brainheck" => Some(highlight(brainheck::Brainheck, src)),
        _ => None
    }
}