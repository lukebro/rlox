use std::error::Error;
use std::fmt::Display;
use std::io::{self, BufRead, Write};
use std::{env, fs, path::PathBuf};

const DATA_EXIT_CODE: i32 = 65;
const USAGE_EXIT_CODE: i32 = 64;

#[derive(Debug, Default)]
enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,

    #[default]
    UNKNOWN,
}

#[derive(Debug, Default)]
struct Token {
    kind: TokenType,
    lexem: String,
    literal: Option<String>,
    line: u32,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.kind, self.lexem)?;

        if let Some(literal) = self.literal.as_ref() {
            write!(f, " {}", literal)?;
        }

        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(USAGE_EXIT_CODE);
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        run_prompt();
    }
}

fn run_file(file: &str) {
    let file_path = fs::canonicalize(PathBuf::from(file)).unwrap();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    match run(&contents) {
        _ => (),
        Err(_) => std::process::exit(DATA_EXIT_CODE),
    }
}

fn run_prompt() {
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let line = stdin.lock().lines().next().unwrap().unwrap();

        if line.is_empty() {
            break;
        }

        match run(&line) {
            Err(_) => {
                error(10, "something happened".to_string());
                std::process::exit(DATA_EXIT_CODE)
            }
            _ => (),
        }
    }
}

fn error(line: u32, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: u32, location: String, message: String) {
    println!("[line {}] Error{}: {}", line, location, message);
}

fn run(source: &str) -> Result<(), Box<dyn Error>> {
    let mut scanner = Scanner::new(&source);
    let mut tokens = Vec::new();

    while let Some(token) = scanner.scan_token() {
        tokens.push(token);
    }

    println!("{:?}", tokens);

    Ok(())
}

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

    fn advance(&mut self) -> u8 {
        let ch = self.source[self.current];
        self.current += 1;

        ch
    }

    fn scan_token(&mut self) -> Option<Token> {
        if self.is_at_end() {
            return None;
        }

        let ch = self.advance();

        if let Some(kind) = self.match_token(ch) {
            match kind {
                TokenType::UNKNOWN => {
                    error(self.line, "Unknown token".to_string());
                    // TODO(luke): this feels weird to exit, we need to just stop
                    // scanning and go into "error" state
                    std::process::exit(DATA_EXIT_CODE)
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

    fn match_token(&mut self, ch: u8) -> Option<TokenType> {
        return match ch {
            b'(' => Some(TokenType::LEFT_PAREN),
            b')' => Some(TokenType::RIGHT_PAREN),
            b',' => Some(TokenType::COMMA),
            b'.' => Some(TokenType::DOT),
            b'-' => Some(TokenType::MINUS),
            b'+' => Some(TokenType::PLUS),
            b';' => Some(TokenType::SEMICOLON),
            b'*' => Some(TokenType::STAR),
            b'!' => Some(if self.check_next(b'=') {
                TokenType::BANG_EQUAL
            } else {
                TokenType::BANG
            }),
            b'=' => Some(if self.check_next(b'=') {
                TokenType::EQUAL_EQUAL
            } else {
                TokenType::EQUAL
            }),
            b'<' => Some(if self.check_next(b'=') {
                TokenType::LESS_EQUAL
            } else {
                TokenType::LESS
            }),
            b'>' => Some(if self.check_next(b'=') {
                TokenType::GREATER_EQUAL
            } else {
                TokenType::GREATER
            }),
            b'/' => {
                if self.check_next(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                    None
                } else {
                    Some(TokenType::SLASH)
                }
            }
            _ => Some(TokenType::UNKNOWN),
        };
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

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
