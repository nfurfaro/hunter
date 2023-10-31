#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};

use noirc_frontend::lexer::Lexer;
use noirc_frontend::token::Token;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the Noir source file to read
    source: std::path::PathBuf,
}

fn lex(source: &str) {
    let (tokens, lexing_errors) = Lexer::lex(source);
    // find a way to iterate through tokens
    // take note of specific types of tokens(=, !=, <, >, etc... depending on config. Start with a default set of tokens to search for, allow user to override.)
    // may need to assign id's to tokens that match. Would AST be helpful for this?
    // @todo create rules for how each token is to be mutated
    // mutate token matches 1 by 1, keeping track of:
    //   - total # of mutants (# of mutated tokens)
    //   - where we are in the list of mutatable tokens
    //   - how many mutants were killed
    //   - how many (& which) mutants survived
    // user's test suite must be run once for each mutant created! This can become huge and slow, need to optimize for performance.
    // given n operators to mutate, we get n mutants.
    // given t tests in the suite, we must run n * t tests.
    // If n = 5, t = 10, total_test_runs = 50 ?
}


fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.source)
        .with_context(|| format!("could not read file `{}`", args.source.display()))?;
    println!("{}", content);

    Ok(())
}


