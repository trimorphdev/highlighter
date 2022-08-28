use regex::Error;

use crate::lexer::LexerContext;

/// The scope of a token.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scope {
    /// A comment token, such as:
    /// 
    /// ```
    /// // Hello, world!
    /// /* Hello, world! */
    /// ```
    /// ```html
    /// <!-- Hello, world! -->
    /// ```
    Comment,

    /// A numeric constant token.
    /// 
    /// ```
    /// 1234;
    /// 1.3f32;
    /// 0x42;
    /// ```
    ConstantNumber,

    /// A character constant token.
    /// 
    /// ```
    /// 'A';
    /// ```
    ConstantChar,

    /// A language constant token.
    /// 
    /// ```
    /// true;
    /// false;
    /// ```
    /// 
    /// ```c
    /// nullptr
    /// ```
    /// 
    /// ```lua
    /// nil
    /// ```
    ConstantLanguage,

    /// Any other constant.
    ConstantOther,

    /// A function name token.
    /// 
    /// ```c
    /// main();
    /// ```
    NameFunction,

    /// A type name token.
    /// 
    /// ```c
    /// typedef struct Name {
    ///     int member;
    /// } Name;
    /// ```
    NameType,

    /// The name of a tag.
    /// 
    /// ```html
    /// <name></name>
    /// ```
    NameTag,

    /// The name of a tag.
    /// 
    /// ```markdown
    /// ## Header
    /// ```
    /// 
    /// ```latex
    /// \chapter{Chapter}
    /// ```
    NameSection,

    /// An invalid token.
    Invalid,

    /// A deprecated case.
    Deprecated,

    /// A storage type keyword, such as `class`, `function` or `var`.
    /// 
    /// ```c++
    /// class MyClass {
    /// public:
    ///     // ...
    /// }
    /// ```
    /// 
    /// ```
    /// fn main() {
    ///     println!("Hello, world!");
    /// }
    /// ```
    StorageType,

    /// A storage modifier keyword, such as `static`, `mut`, `final`, etc.
    StorageModifier,

    /// A quoted string token.
    StringQuoted,

    /// A string that is evaluated, such as JavaScript template strings.
    StringEvaluated,

    /// A regex string.
    /// 
    /// ```js
    /// /([a-zA-Z])+/g
    /// ```
    StringRegex,

    /// Any other kind of string.
    StringOther,

    /// A function provided by the language or standard library.
    SupportFunction,

    /// A type (class, struct, etc.) provided by the language or standard library.
    SupportType,

    /// A constant provided by the language or standard library.
    SupportConstant,

    /// A variable provided by the language or standard library.
    SupportVar,

    /// Any other supporting value.
    SupportOther,

    /// A variable declared as a function parameter.
    VariableParameter,

    /// A special variable such as `super` or `this`.
    VariableLanguage,

    /// Any other variable names.
    VariableOther,

    /// A controlling keyword, such as `if`, `break`, `return`, `while`, etc.
    KeywordControl,

    /// An other keyword.
    KeywordOther,

    /// No scope matches this token.
    None,
}

impl Scope {
    /// Converts the [`Scope`]'s name to a snake case string.
    pub fn snake_case(&self) -> &str {
        match self {
            Self::Comment => "comment",
            Self::ConstantNumber => "constant_number",
            Self::ConstantChar => "constant_char",
            Self::ConstantLanguage => "constant_language",
            Self::ConstantOther => "constant_other",
            Self::NameFunction => "name_function",
            Self::NameType => "name_type",
            Self::NameTag => "name_tag",
            Self::NameSection => "name_section",
            Self::Invalid => "invalid",
            Self::Deprecated => "deprecated",
            Self::StorageType => "storage_type",
            Self::StorageModifier => "storage_modifier",
            Self::StringQuoted => "string_quoted",
            Self::StringEvaluated => "string_evaluated",
            Self::StringRegex => "string_regex",
            Self::StringOther => "string_other",
            Self::SupportFunction => "support_function",
            Self::SupportType => "support_type",
            Self::SupportConstant => "support_constant",
            Self::SupportVar => "support_var",
            Self::SupportOther => "support_other",
            Self::VariableParameter => "variable_parameter",
            Self::VariableLanguage => "variable_language",
            Self::VariableOther => "variable_other",
            Self::KeywordControl => "keyword_control",
            Self::KeywordOther => "keyword_other",
            Self::None => "none",
        }
    }
}

/// A language implementation for Highlighter.
pub trait Language {

    /// Returns the name of the programming language.
    fn name(&self) -> String;

    /// Returns all of the aliases of the programming language.
    /// 
    /// For example:
    /// ```
    /// vec!["js".to_owned(), "javascript".to_owned(), "jscript".to_owned(), "es".to_owned(), "ecmascript".to_owned()];
    /// ```
    fn names(&self) -> Vec<String> {
        vec![self.name()]
    }

    /// Initializes the programming language.
    fn init(&self, x: &mut LexerContext) -> Result<(), Error>;
}