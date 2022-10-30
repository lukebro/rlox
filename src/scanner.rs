use crate::report_error;
use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    line: u32,
    start: usize,
    current: usize,
    source: &'a [u8],
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            line: 1,
            start: 0,
            current: 0,
            source: source.as_bytes(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();

        while !self.is_at_end() {
            self.start = self.current;

            if let Some(token) = self.scan_token() {
                tokens.push(token);
            }
        }

        tokens.push(Token {
            kind: TokenType::EOF,
            line: self.line,
            ..Default::default()
        });

        tokens
    }

    fn scan_token(&mut self) -> Option<Token> {
        let ch = self.advance();

        let token = match ch {
            b'(' => self.create_token(TokenType::LeftParen),
            b')' => self.create_token(TokenType::RightParen),
            b',' => self.create_token(TokenType::Comma),
            b'.' => self.create_token(TokenType::Dot),
            b'-' => self.create_token(TokenType::Minus),
            b'+' => self.create_token(TokenType::Plus),
            b';' => self.create_token(TokenType::Semicolon),
            b'*' => self.create_token(TokenType::Star),

            b'!' => self.match_or(b'=', TokenType::BangEqual, TokenType::Bang),
            b'=' => self.match_or(b'=', TokenType::EqualEqual, TokenType::Equal),
            b'<' => self.match_or(b'=', TokenType::LessEqual, TokenType::Less),
            b'>' => self.match_or(b'=', TokenType::GreaterEqual, TokenType::Greater),

            // Comments or forward slash
            b'/' => {
                if self.match_next(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                    return None;
                } else {
                    self.create_token(TokenType::Slash)
                }
            }

            b' ' | b'\r' | b'\t' => return None,

            b'\n' => {
                self.line += 1;
                return None;
            }

            b'"' => {
                if let Some(value) = self.scan_string() {
                    Token {
                        kind: TokenType::String,
                        line: self.line,
                        literal: Some(value.to_string()),
                        ..Default::default()
                    }
                } else {
                    report_error(self.line, "Unterminated string".into());
                    return None;
                }
            }

            _ => Token {
                kind: TokenType::Unknown,
                line: self.line,
                ..Default::default()
            },
        };

        return Some(token);
    }

    fn match_or(&mut self, next: u8, one: TokenType, two: TokenType) -> Token {
        let kind = if self.match_next(next) { one } else { two };

        Token {
            kind,
            line: self.line,
            ..Default::default()
        }
    }

    fn create_token(&self, kind: TokenType) -> Token {
        Token {
            kind,
            line: self.line,
            ..Default::default()
        }
    }

    fn advance(&mut self) -> u8 {
        let ch = self.source[self.current];
        self.current += 1;

        ch
    }

    fn scan_string(&mut self) -> Option<String> {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return None;
        }

        // The closing ".
        self.advance();

        let start = self.start + 1;
        let end = self.current - 1;

        let value = &self.source[start..end];
        let literal = std::str::from_utf8(value).unwrap().to_string();

        Some(literal)
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source[self.current]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
