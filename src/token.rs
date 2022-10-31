use std::fmt::Display;

#[derive(Debug, Default)]
pub struct Token {
    pub kind: TokenType,
    pub lexem: Option<String>,
    pub line: usize,
}

impl Display for Token {
    // KIND LEXEM? LITERAL?
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO(luke): impl Display for TokenType
        write!(f, "{:?}", self.kind)?;

        if let Some(lexem) = self.lexem.as_ref() {
            write!(f, " {}", lexem)?;
        }

        match &self.kind {
            TokenType::String(text) => {
                write!(f, " {}", text)?;
            }
            TokenType::Number(num) => {
                write!(f, " {:.2}", num)?;
            }
            _ => (),
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
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
    Identifier(String),
    String(String),
    Number(f64),

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

    Eof,

    #[default]
    Unknown,
}
