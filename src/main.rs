use std::env;
use std::fs;
use std::process::exit;
use std::io::{self, BufRead, Write};

mod scanner;
mod expression;
use crate::scanner::*;
use crate::expression::*;

fn run_file(path: &str) ->Result<(), String> {
    // Reads program file
    match fs::read_to_string(path) {
        Err(msg) => Err(msg.to_string()),
        Ok(contents) => return run(&contents),
    } 
}

fn run(contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

// Basic REPL without the evaluation
fn run_prompt() -> Result<(), String> {
    loop {
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Could not flush Stdout".to_string()),
        }

        let mut input = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut input) {
            Ok(n) => {
                if n <= 1 {
                    return Ok(());
                }
            },
            Err(_) => return Err("Couldn't read the line".to_string()),
        }
            
        println!("{}", input);
        match run(&input) {
            Ok(_) => Ok(()),
            Err(msg) => Err(msg),
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: Symphony [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error:\n{}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error:\n{}", msg);
                exit(1);
            }
        }
    }
}
