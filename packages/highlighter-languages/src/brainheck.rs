use highlighter_core::{language::{Language, Scope}};

/// Rust language extension for Highlighter.
pub struct Brainheck;

impl Language for Brainheck {
    fn name(&self) -> String {
        "brainheck".to_string()
    }

    fn names(&self) -> Vec<String> {
        vec!["brainheck".to_string(), "bf".to_string()]
    }

    fn init(&self, cx: &mut highlighter_core::LexerContext) -> Result<(), highlighter_core::Error> {
        cx.token(Scope::KeywordOperator, "[+\\-<>\\.,\\[\\]]+")?;
        cx.token(Scope::Comment, "[^+\\-<>\\.,\\[\\]]+")?;
        
        Ok(())
    }
}