use logos::Logos;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum Token {
    /*
        +----------+
        | Keywords |
        +----------+
    */
    
    ///
    /// ```
    /// let name = "regretti"
    /// ```
    /// 
    #[token("let")]
    Variable,

    ///
    /// ```
    /// main:
    ///     ...
    /// ```
    ///
    #[token("main:")]
    MainFunction,

    ///
    /// ```
    /// funcname:
    ///     ...
    /// ```
    ///
    #[regex("[a-zA-Z_]+:")]
    FunctionStart,

    #[token(":end")]
    FunctionEnd,

    #[token("/*")]
    CommandStart,

    #[regex(r"\*/")]
    CommandEnd,

    #[regex(r"\|(.*)\|")]
    Statement,

    #[token("\n")]
    Newline,

    #[regex(r"[a-zA-Z_]+\((.*)\)\?")]
    LibFunctionNoExecute,

    #[regex(r"[a-zA-Z_]+\((.*)\)")]
    LibFunction,

    #[regex("[a-zA-Z_]+")]
    Keyword,

    #[regex(r"\#(.*)", logos::skip)]
    Comments,

    /*
        +-----------+
        | Datatypes |
        +-----------+
    */
    
    #[regex("[0-9]+")]
    Number,

    Math,

    #[regex("\"(.*)\"")]
    String,

    /*
        +-----------+
        | Operators |
        +-----------+
    */
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,
    
    #[token("/")]
    Divide,

    #[token("=")]
    Assign,

    #[token("==")]
    Equals,   

    #[token("!=")]
    NotEquals,

    #[token(">")]
    GreaterThan,
    
    #[token("<")]
    LesserThan,

    #[token(">=")]
    GreaterThanOrEquals,

    #[token("<=")]
    LesserThanOrEquals,

    /*
        +------------+
        | Error/Skip |
        +------------+
    */
    // whitespace
    #[regex(r"[ \t\f]+", logos::skip)]
    // syntatic sugar
    #[regex(r"\+(.*)\+", logos::skip)]    
    Skipped,

    #[error]
    PassLex,
}

use once_cell::sync::Lazy;
use regex::Regex;

// parser grammar
pub static STATEMENT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\|(.*)\|").unwrap());
pub static STRING: Lazy<Regex> = Lazy::new(|| Regex::new("\"(.*)\"").unwrap());

// debug grammar:
// pub static MATH: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d+(\s*[+\-*/]\s*\d+)+$").unwrap());
// pub static KEYWORD: Lazy<Regex> = Lazy::new(|| Regex::new("[a-zA-Z_]+").unwrap());

/// Returns the token type from grammar matches
pub fn token_grammar(slice: &str) -> Token {
    if STATEMENT.is_match(slice) {
        Token::Statement
    } else if STRING.is_match(slice) {
        Token::String
    } else {
        Token::Skipped
    }
}