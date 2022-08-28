extern crate highlighter;

use highlighter::{core::language::{Language, Scope}, highlight, HighlighterTargetHtml};

/// My example programming language
pub struct MyLanguage;

impl Language for MyLanguage {
    fn name(&self) -> String {
        "my-language".to_string()
    }
    
    fn init(&self, cx: &mut highlighter::core::LexerContext) -> Result<(), highlighter::core::Error> {
        // Initialize the tokens in your language.
        cx.token(Scope::KeywordControl, "\\b(if|else|while|continue|break|return)\\b")?;
        cx.token(Scope::StorageType, "\\b(var|function)\\b")?;
        cx.token(Scope::ConstantNumber, "\\b([0-9][0-9_]*)\\b")?;
        cx.token(Scope::ConstantLanguage, "\\b(true|false)\\b")?;
        Ok(())
    }
}

fn main() {
    let tokens = highlight(MyLanguage, "var i = 0;").unwrap();
    let html = HighlighterTargetHtml::new()
        .build(tokens);
    
    println!("{}", html);
}