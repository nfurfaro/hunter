use crate::mutant::{mutant_builder, Mutant, MutationStatus};
use crate::parallel::parallel_process_mutated_tokens;
use crate::utils::{collect_tokens, find_noir_files, print_line_in_span};
use clap::Parser;
use colored::*;
use prettytable::{Cell, Row, Table};
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
    // Display information about the program
    #[clap(short, long)]
    info: bool,
}

pub async fn run_cli() -> Result<()> {
    let args = Cli::parse();

    if args.info {
        println!(
            "{}",
            "Welcome to Hunter, a tool for mutation-testing Noir source code.".cyan()
        );
        return Ok(());
    }
    // add a [workspace] to the project manifest
    // modify_toml();

    println!("{}", "Initiating mutant hunter...".cyan());
    // collect all noir files in the current directory recursively
    println!("{}", "Searching for Noir files".cyan());
    let noir_files = find_noir_files(Path::new("."))?;
    println!("{}", "Found:".cyan());
    for file in &noir_files {
        println!("{}", format!("{}", file.1.as_path().display()).red());
    }

    // @todo handle unwrap
    // get all the tokens from the collected noir files, along with the path to their origin file
    println!("{}", "Collecting tokens from files".cyan());
    let tokens_with_paths = collect_tokens(&noir_files)
        .expect("No Noir files found... Are you in the right directory?");

    let mut mutants: Vec<Mutant> = vec![];
    println!("{}", "Building mutants".cyan());
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

    // println!("mutants: {:#?}", mutants);

    // Create a new table
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Surviving Mutants").style_spec("Fmb")
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Source file:").style_spec("Fyb"),
        Cell::new("Line #:").style_spec("Fyb"),
        Cell::new("    Mutant context:").style_spec("Fyb"),
        Cell::new("Original:").style_spec("Fyb"),
    ]));

    for mutant in &mutants {
        if mutant.status() == MutationStatus::Survived || mutant.status() == MutationStatus::Pending
        {
            let span = mutant.span();
            let span_usize = (span.0 as usize, span.1 as usize);

            print_line_in_span(
                &mut table,
                Path::new(mutant.path()),
                span_usize,
                &mutant.token(),
            )
            .unwrap();
        }
    }

    table.printstd();

    println!("{}", "Cleaning up temp files".cyan());

    let current_dir = std::env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dir);

    // @fix
    // Remove the ./temp directory
    // let temp_dir = Path::new("./temp");
    // if temp_dir.exists() {
    //     std::fs::remove_dir_all(&temp_dir).expect("Failed to remove ./temp directory");
    // }

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
