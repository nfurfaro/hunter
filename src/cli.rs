use crate::mutant::{mutant_builder, Mutant};
use crate::parallel::parallel_process_mutated_tokens;
use crate::utils::*;
use clap::Parser;

use std::path::Path;

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

pub async fn run_cli() -> std::io::Result<()> {
    let _args = Cli::parse();

    // collect all noir files in the current directory recursively
    println!("Searching for Noir files...");
    let copied_noir_files = find_and_copy_noir_files(Path::new("."))?;

    // @todo handle unwrap
    // get all the tokens from the collected noir files, along with the path to their origin file
    let tokens_with_paths = collect_tokens(&copied_noir_files).unwrap();
    let mut mutants: Vec<Mutant> = vec![];
    for entry in tokens_with_paths {
        let path = entry.1.as_path();
        let maybe_mutant = mutant_builder(entry.0.clone(), Path::new(path));
        match maybe_mutant {
            None => continue,
            Some(m) => mutants.push(m),
        }
    }

    parallel_process_mutated_tokens(&mut mutants);

    Ok(())
}
