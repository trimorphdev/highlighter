//! The core of the Highlighter crate.

pub mod language;
mod lexer;

pub use regex::Error;

pub use lexer::{LexerContext, Token, TokenHandler, TokenContext};
use language::Language;
use lexer::Lexer;


/// Highlights the provided code block.
pub fn highlight<L: Language>(language: L, src: &str) -> Result<Vec<Token>, Error> {
    let lexer = Lexer::new(language)?;
    Ok(lexer.lex(src))
}