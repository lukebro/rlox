use crate::token::{Token, TokenType};
use crate::{error, DATA_EXIT_CODE};

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

    pub fn scan_token(&mut self) -> Option<Token> {
        if self.is_eof() {
            self.current += 1;
            return Some(Token {
                kind: TokenType::EOF,
                line: self.line,
                ..Default::default()
            });
        }

        if self.is_at_end() {
            return None;
        }

        self.start = self.current;

        let ch = self.advance();

        if let Some(kind) = self.match_token(ch) {
            match kind {
                TokenType::Unknown => {
                    error(self.line, "Unknown token".into());
                    // TODO(luke): this feels weird to exit, we need to just stop
                    // scanning and go into "error" state
                    std::process::exit(DATA_EXIT_CODE)
                }

                TokenType::String => {
                    if let Some(value) = self.scan_string() {
                        Some(Token {
                            kind,
                            line: self.line,
                            literal: Some(value.to_string()),
                            ..Default::default()
                        })
                    } else {
                        error(self.line, "Unterminated string".into());
                        std::process::exit(DATA_EXIT_CODE)
                    }
                }
                _ => Some(Token {
                    kind,
                    line: self.line as u32,
                    ..Default::default()
                }),
            }
        } else {
            // match_token returns None, we have
            // hit whitespace or something
            return self.scan_token();
        }
    }

    fn advance(&mut self) -> u8 {
        let ch = self.source[self.current];
        self.current += 1;

        ch
    }

    fn match_token(&mut self, ch: u8) -> Option<TokenType> {
        return match ch {
            b'(' => Some(TokenType::LeftParen),
            b')' => Some(TokenType::RightParen),
            b',' => Some(TokenType::Comma),
            b'.' => Some(TokenType::Dot),
            b'-' => Some(TokenType::Minus),
            b'+' => Some(TokenType::Plus),
            b';' => Some(TokenType::Semicolon),
            b'*' => Some(TokenType::Star),

            b'!' => Some(if self.check_next(b'=') {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            }),

            b'=' => Some(if self.check_next(b'=') {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            }),

            b'<' => Some(if self.check_next(b'=') {
                TokenType::LessEqual
            } else {
                TokenType::Less
            }),

            b'>' => Some(if self.check_next(b'=') {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            }),

            b'/' => {
                if self.check_next(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                    None
                } else {
                    Some(TokenType::Slash)
                }
            }

            b' ' | b'\r' | b'\t' => None,

            b'\n' => {
                self.line += 1;
                None
            }

            b'"' => Some(TokenType::String),
            _ => Some(TokenType::Unknown),
        };
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

    fn check_next(&mut self, expected: u8) -> bool {
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

    fn is_eof(&self) -> bool {
        self.current == self.source.len()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
