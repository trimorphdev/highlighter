use highlighter_core::Token;

fn html_escape(str: String) -> String {
    let mut new_str = String::new();

    for char in str.chars() {
        match char {
            '>' => new_str.push_str("&gt;"),
            '<' => new_str.push_str("&lt;"),
            '&' => new_str.push_str("&amp;"),
            '\'' => new_str.push_str("&#39;"),
            '"' => new_str.push_str("&quot;"),
            _ => new_str.push(char),
        }
    }

    new_str
}

/// An HTML target for Highlighter.
/// 
/// TODO: add a beautify option.
#[derive(Clone, Debug, PartialEq)]
pub struct HighlighterTargetHtml {
    /// The prefix to use for outputted HTML.  Defaults to "".
    html_prefix: String,

    /// The prefix to use for outputted HTML.  Defaults to "".
    html_suffix: String,

    /// The prefix used for CSS class names.
    /// 
    /// Defaults to `scope-`.
    class_prefix: String,
}

impl HighlighterTargetHtml {
    /// Creates a new, default HTML target.
    pub fn new() -> Self {
        Self { html_prefix: String::new(), html_suffix: String::new(), class_prefix: "scope-".to_owned() }
    }

    /// Returns this HTML target, with the provided HTML prefix.
    pub fn with_html_prefix<Str: Into<String>>(mut self, html_prefix: Str) -> Self {
        self.html_prefix = html_prefix.into();
        self
    }

    /// Returns this HTML target, with the provided HTML suffix.
    pub fn with_html_suffix<Str: Into<String>>(mut self, html_suffix: Str) -> Self {
        self.html_suffix = html_suffix.into();
        self
    }

    /// Returns this HTML target, with the provided class prefix.
    pub fn with_class_prefix<Str: Into<String>>(mut self, class_prefix: Str) -> Self {
        self.class_prefix = class_prefix.into();
        self
    }

    /// Builds the given tokens into a string of HTML.
    pub fn build(&self, tokens: Vec<Token>) -> String {
        let mut str = String::new();

        for token in tokens {
            str.push_str(&format!("<span class=\"{}{}\">{}</span>", self.class_prefix, token.scope.kebab_case(), html_escape(token.value)));
        }

        str
    }
}