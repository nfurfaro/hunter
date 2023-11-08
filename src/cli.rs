use crate::mutant::{mutant_builder, Mutant};
use crate::parallel::parallel_process_mutated_tokens;
use crate::utils::*;
use clap::Parser;
use colored::*;

use std::{io::Result, path::Path};

/// Mutate Noir code and run tests against each mutation.
#[derive(Parser)]
struct Cli {
    /// The path to the Noir source files directory, defaults to ./src
    #[clap(short, long)]
    source_dir: Option<std::path::PathBuf>,
    /// The path to the test directory, defaults to ./tests
    #[clap(short, long)]
    test_dir: Option<std::path::PathBuf>,
}

pub async fn run_cli() -> Result<()> {
    let _args = Cli::parse();

    // collect all noir files in the current directory recursively
    println!("{}", "Searching for Noir files".green());
    let noir_files = find_noir_files(Path::new("."))?;
    println!("{}", "Found:".green());
    for file in &noir_files {
        println!("{}", format!("{}", file.1.as_path().display()).green());
    }

    // @todo handle unwrap
    // get all the tokens from the collected noir files, along with the path to their origin file
    println!("{}", "Collecting tokens from files".green());
    let tokens_with_paths = collect_tokens(&noir_files)
        .expect("No Noir files found... Are you in the right directory?");

    let mut mutants: Vec<Mutant> = vec![];
    println!("{}", "Building mutants".green());
    for entry in tokens_with_paths {
        let path = entry.1.as_path();
        let spanned_token = entry.0.clone();
        let span = spanned_token.to_span();
        let maybe_mutant = mutant_builder(
            entry.2,
            spanned_token.token().clone(),
            (span.start(), span.end()),
            Path::new(path),
        );
        match maybe_mutant {
            None => continue,
            Some(m) => mutants.push(m),
        }
    }

    parallel_process_mutated_tokens(&mut mutants);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "No Noir files found... Are you in the right directory?")]
    async fn test_run_cli() {
        run_cli().await.unwrap();
    }
}
