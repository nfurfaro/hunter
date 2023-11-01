#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;

use noirc_frontend::lexer::Lexer;
use noirc_frontend::token::Token;

/// Mutate Noir code and run tests against each mutation.
#[derive(Parser)]
struct Cli {
    /// The path to the Noir source file to read
    source: std::path::PathBuf,

    /// The path to the test directory
    tests: std::path::PathBuf,
}

fn lex(source: &str) {
    let (tokens, lexing_errors) = Lexer::lex(source);
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.source)
        .with_context(|| format!("could not read file `{}`", args.source.display()))?;
    println!("{}", content);

    Ok(())
}
