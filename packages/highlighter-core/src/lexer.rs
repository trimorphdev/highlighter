use regex::{Regex, Error, CaptureLocations};

use crate::language::{Language, Scope};

/// A token in a token context.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// The scope of this token.
    pub scope: Scope,

    /// The raw value of this token.
    pub value: String,
}

/// A context for processing tokens.
pub struct TokenContext {
    /// The tokens in the context.
    tokens: Vec<Token>,
}

impl TokenContext {
    /// Inserts a token into the token context.
    pub fn token<Str: Into<String>>(&mut self, scope: Scope, value: Str) {
        self.tokens.push(Token { scope, value: value.into() });
    }
}

/// A function which handles a match, and then outputs the corresponding tokens.
pub type TokenHandler = fn(CaptureLocations, &String, &mut TokenContext);

/// A pattern in the lexer.
struct HandledPattern {
    /// The pattern to match.
    regex: Regex,

    /// The handler for the pattern.
    handler: TokenHandler,
}

/// A pattern in the Highlighter.
enum Pattern {
    /// A plain pattern with no handler.
    Plain(Scope, Regex),

    /// A pattern with a handler.
    Handled(HandledPattern),
}

/// A context used by the lexer to match tokens.
pub struct LexerContext {
    /// A list of patterns that this lexer context can match.
    patterns: Vec<Pattern>,
}

impl LexerContext {
    /// Creates a new lexer context initialized with a catchall token.
    #[inline]
    fn new() -> Result<Self, Error> {
        Ok(Self { patterns: Vec::new() })
    }

    pub fn token<Str: Into<String>>(&mut self, scope: Scope, pattern: Str) -> Result<(), Error> {
        // insert before `__other` token.
        self.patterns.push(Pattern::Plain(scope, Regex::new(&pattern.into())?));
        Ok(())
    }

    /// Registers a pattern in the lexer context.
    pub fn advanced_token<Str: Into<String>>(&mut self, pattern: Str, handler: TokenHandler) -> Result<(), Error> {
        // insert before `__other` token.
        self.patterns.push(Pattern::Handled(HandledPattern { regex: Regex::new(&pattern.into())?, handler }));
        Ok(())
    }
}

/// A lexer for the selected language.
pub struct Lexer {
    /// The context used by the language.
    ctx: LexerContext,
}

impl Lexer {
    /// Creates a lexer, initialized for the selected language.
    pub fn new<L: Language>(language: L) -> Result<Self, Error> {
        let mut ctx = LexerContext::new()?;

        language.init(&mut ctx)?;

        Ok(Self { ctx })
    }

    /// Tokenizes a string.
    pub fn lex(&self, str: &str) -> Vec<Token> {
        let mut i = 0;
        let mut tokens = TokenContext { tokens: Vec::new() };

        'str_iter: while i < str.len() {
            for pattern in &self.ctx.patterns {
                match pattern {
                    Pattern::Plain(scope, regex) => {
                        let mut captures = regex.capture_locations();

                        if let Some(m) = regex.captures_read_at(&mut captures, str, i) {
                            if m.start() != i {
                                continue;
                            }

                            i = m.end();
                            tokens.token(*scope, m.as_str());
                            continue 'str_iter;
                        }
                    },
                    Pattern::Handled(pattern) => {
                        let regex = &pattern.regex;
                        let mut captures = pattern.regex.capture_locations();

                        if let Some(m) = regex.captures_read_at(&mut captures, str, i) {
                            if m.start() != i {
                                continue;
                            }

                            i = m.end();
                            (pattern.handler)(captures, &str.to_owned(), &mut tokens);
                            continue 'str_iter;
                        }
                    },
                }
            }

            tokens.token(Scope::None, str.chars().nth(i).unwrap());
            i += 1;
        }

        tokens.tokens
    }
}