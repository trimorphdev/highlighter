//! An extendable syntax highlighter written in Rust.

pub use highlighter_core as core;
pub use highlighter_core::{highlight, Token};

#[cfg(feature = "js")]
pub use highlighter_js::JavaScript;

#[cfg(feature = "js")]
#[test]
fn test_js() {
    let tokens = highlight(JavaScript, "function main()").unwrap();
    dbg!(tokens);
}