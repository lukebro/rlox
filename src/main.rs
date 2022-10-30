mod scanner;
mod token;

use std::io::{self, BufRead, Write};
use std::{env, fs, path::PathBuf};

use anyhow::{Result};

use crate::scanner::Scanner;

const USAGE_EXIT_CODE: i32 = 64;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(USAGE_EXIT_CODE);
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt()?;
    }

    Ok(())
}

fn run_file(file: &str) -> Result<()> {
    let file_path = fs::canonicalize(PathBuf::from(file)).unwrap();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    run(&contents)?;

    Ok(())
}

fn run_prompt() -> Result<()> {
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let line = stdin.lock().lines().next().unwrap().unwrap();

        if line.is_empty() {
            break;
        }

        run(&line)?;
    }

    Ok(())
}

fn report_error(line: u32, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: u32, location: String, message: String) {
    println!("[line {}] Error{}: {}", line, location, message);
}

fn run(source: &str) -> Result<()> {
    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan_tokens();

    for token in &tokens {
        println!("{:?}", token);
    }

    Ok(())
}
