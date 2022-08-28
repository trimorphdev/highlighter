# Highlighter
An extendable syntax highlighter written in Rust.

## Installation
Simply add highlighter to your dependencies:
```toml
[dependencies]
highlighter = "0.1.1-alpha"
```

## Basic Usage
```rust
use highlighter::{highlight_language, HighlightTargetHtml};

fn main() {
    let result = highlight_language("brainheck", "++++++++ set current cell to 8").unwrap().unwrap();
    let html = HighlightTargetHtml::new()
        .build(result);
    println!(html);
}
```

## Implementing Languages
To extend Highlighter, make a structure which implements the `Language` trait.

```rust
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
```