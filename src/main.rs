use std::{env, fs::File, io::{self, Read, Write}};

use scanner::Scanner;
use token::Token;

mod token;
mod scanner;
mod error;
use error::{Result, Error};

fn main() -> Result<()> {
    let mut args = env::args();

    match args.len() {
        1 => run_prompt()?,
        2 => run_file(&args.next().unwrap())?,
        _ => {
            println!("cargo run [script]");
            std::process::exit(64);
        }
    };
    
    Ok(())
}
//TODO: better error handling?
fn run_file(file_path: &str) -> Result<()> {
    let mut file = File::open(file_path).map_err(|_| Error::FileNotFound(file_path.to_owned()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|_| Error::FileNotUtf8(file_path.to_owned()))?;
    run(&contents)?;
    Ok(())
}

fn run_prompt() -> Result<()> {
    loop {
        let mut line = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        if line.is_empty() {
            break;
        }
        run(&line)?;
    }
    Ok(())
}
fn run(source: &str) -> Result<()> {
    let mut scanner = Scanner::new(source.to_owned());
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }
    
    Ok(())
}