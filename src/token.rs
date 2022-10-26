
use std::fmt::Display;

#[derive(Debug, Default)]
pub struct Token {
    pub kind: TokenType,
    pub lexem: Option<String>,
    pub literal: Option<String>,
    pub line: u32,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)?;

        if let Some(lexem) = self.lexem.as_ref() {
            write!(f, " {}", lexem)?;
        }

        if let Some(literal) = self.literal.as_ref() {
            write!(f, " {}", literal)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,

    #[default]
    Unknown,
}
