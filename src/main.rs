use std::{
    env,
    fs::File,
    io::{self, Read, Write},
};

mod ast_printer;
mod error;
mod expr;
mod parser;
mod scanner;
mod token;

use parser::Parser;
// use ast_printer::AstPrinter;
// use expr::{BinaryExpr, Expr, UnaryExpr};
use error::{Error, Result};
use scanner::Scanner;
// use token::Token;

use crate::{
    ast_printer::AstPrinter,
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    token::{Literal, Token, TokenType},
};

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
    file.read_to_string(&mut contents)
        .map_err(|_| Error::FileNotUtf8(file_path.to_owned()))?;
    run(&contents)?;
    Ok(())
}

fn run_prompt() -> Result<()> {
    loop {
        let mut line = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
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
    let mut parser = Parser::new(tokens.to_vec());
    let expression = parser.parse()?;

    let printer = AstPrinter {};
    println!("{}", printer.print(&expression)?);

    Ok(())
}

fn report(msg: String) {
    eprintln!("{msg}");
}
