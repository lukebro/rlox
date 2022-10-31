use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::report_error;
use crate::token::{Token, TokenType};

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();

        map.insert("and", TokenType::And);
        map.insert("class", TokenType::Class);
        map.insert("else", TokenType::Else);
        map.insert("false", TokenType::False);
        map.insert("for", TokenType::For);
        map.insert("fun", TokenType::Fun);
        map.insert("if", TokenType::If);
        map.insert("nil", TokenType::Nil);
        map.insert("or", TokenType::Or);
        map.insert("print", TokenType::Print);
        map.insert("return", TokenType::Return);
        map.insert("super", TokenType::Super);
        map.insert("this", TokenType::This);
        map.insert("true", TokenType::True);
        map.insert("var", TokenType::Var);
        map.insert("while", TokenType::While);

        map
    };
}

fn keyword(identifier: &str) -> Option<TokenType> {
    match KEYWORDS.get(identifier) {
        None => None,
        Some(token) => Some(token.clone()),
    }
}

pub struct Scanner<'a> {
    line: usize,
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
            kind: TokenType::Eof,
            line: self.line,
            ..Default::default()
        });

        tokens
    }

    fn scan_token(&mut self) -> Option<Token> {
        let ch = self.advance();

        let token = match ch {
            b'(' => self.new_token(TokenType::LeftParen),
            b')' => self.new_token(TokenType::RightParen),
            b'{' => self.new_token(TokenType::LeftParen),
            b'}' => self.new_token(TokenType::RightBrace),
            b',' => self.new_token(TokenType::Comma),
            b'.' => self.new_token(TokenType::Dot),
            b'-' => self.new_token(TokenType::Minus),
            b'+' => self.new_token(TokenType::Plus),
            b';' => self.new_token(TokenType::Semicolon),
            b'*' => self.new_token(TokenType::Star),

            b'!' => self.match_or(b'=', TokenType::BangEqual, TokenType::Bang),
            b'=' => self.match_or(b'=', TokenType::EqualEqual, TokenType::Equal),
            b'<' => self.match_or(b'=', TokenType::LessEqual, TokenType::Less),
            b'>' => self.match_or(b'=', TokenType::GreaterEqual, TokenType::Greater),

            // Comments or forward slash
            b'/' => {
                if self.match_next(b'/') {
                    while let Some(next) = self.peek() {
                        if next == b'\n' {
                            break;
                        }

                        self.advance();
                    }

                    return None;
                } else {
                    self.new_token(TokenType::Slash)
                }
            }

            b' ' | b'\r' | b'\t' => return None,

            b'\n' => {
                self.line += 1;
                return None;
            }

            b'"' => {
                if let Some(value) = self.scan_string() {
                    self.new_token(TokenType::String(value))
                } else {
                    report_error(self.line, "Unterminated string".into());
                    return None;
                }
            }

            c => {
                if is_digit(c) {
                    if let Some(value) = self.scan_number() {
                        return Some(self.new_token(TokenType::Number(value)));
                    } else {
                        report_error(self.line, "Unexpected character".into());
                        return None;
                    }
                } else if is_alpha(c) {
                    if let Some(value) = self.scan_identifier() {
                        if let Some(reserved) = keyword(&value) {
                            return Some(self.new_token(reserved));
                        } else {
                            return Some(self.new_token(TokenType::Identifier(value)));
                        }
                    } else {
                        report_error(self.line, "Unexpected character".into());
                        return None;
                    }
                } else {
                    self.new_token(TokenType::Unknown)
                }
            }
        };

        return Some(token);
    }

    fn match_or(&mut self, next: u8, one: TokenType, two: TokenType) -> Token {
        let kind = if self.match_next(next) { one } else { two };

        self.new_token(kind)
    }

    fn new_token(&self, kind: TokenType) -> Token {
        let lexem = match std::str::from_utf8(&self.source[self.start..self.current]).ok() {
            Some(lexem) => Some(lexem.to_string()),
            None => None,
        };

        Token {
            kind,
            line: self.line,
            lexem: lexem,
        }
    }

    fn advance(&mut self) -> u8 {
        let ch = self.source[self.current];
        self.current += 1;

        ch
    }

    fn scan_string(&mut self) -> Option<String> {
        while let Some(ch) = self.peek() {
            // at end
            if ch == b'"' {
                break;
            }

            match self.peek() {
                Some(b'\n') => self.line += 1,
                _ => (),
            }

            self.advance();
        }

        if self.is_at_end() {
            // TODO should be result with error on unterminated string
            return None;
        }

        // The closing ".
        self.advance();

        self.offset_as_string(1)
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> Option<u8> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.current])
        }
    }

    fn peek_next(&self) -> Option<u8> {
        if self.current + 1 >= self.source.len() {
            None
        } else {
            Some(self.source[self.current + 1])
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_number(&mut self) -> Option<f64> {
        while let Some(value) = self.peek() {
            if is_digit(value) {
                self.advance();
            } else {
                break;
            }
        }

        if let (Some(value), Some(next)) = (self.peek(), self.peek_next()) {
            if value == b'.' && is_digit(next) {
                self.advance();

                while let Some(value) = self.peek() {
                    if is_digit(value) {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }

        let lexem = &self.source[self.start..self.current];

        match std::str::from_utf8(lexem).ok() {
            Some(s) => s.parse::<f64>().ok(),
            None => None,
        }
    }

    fn offset_as_string(&self, offset: usize) -> Option<String> {
        let slice = &self.source[self.start + offset..self.current - offset];

        match std::str::from_utf8(slice).ok() {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }

    fn scan_identifier(&mut self) -> Option<String> {
        while let Some(next) = self.peek() {
            if is_alphanumeric(next) {
                self.advance();
            } else {
                break;
            }
        }

        self.offset_as_string(0)
    }
}

fn is_digit(digit: u8) -> bool {
    digit >= b'0' && digit <= b'9'
}

fn is_alpha(alpha: u8) -> bool {
    (alpha >= b'a' && alpha <= b'z') || (alpha >= b'A' && alpha <= b'Z') || (alpha == b'_')
}

fn is_alphanumeric(c: u8) -> bool {
    is_digit(c) || is_alpha(c)
}
