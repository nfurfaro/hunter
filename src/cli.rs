use crate::mutant::{mutant_builder, Mutant, MutationStatus};
use crate::parallel::parallel_process_mutated_tokens;
use crate::utils::*;
use clap::Parser;
use colored::*;
use prettytable::{row, Cell, Row, Table};

use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

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

fn print_line_in_span(file_path: &Path, span: (usize, usize)) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut byte_index = 0;

    // for (index, line) in reader.lines().enumerate() {
    //     let line = line?;
    //     let line_length = line.len();

    //     if byte_index <= span.0 && byte_index + line_length >= span.1 {
    //         println!("File: {:?}, Line {}: {}", file_path, index + 1, line);
    //         break;
    //     }

    //     byte_index += line_length + 1; // +1 for the newline character
    // }

    let mut table = Table::new();
    table.add_row(row!["Surviving Mutants"]);
    table.add_row(row!["Source File", "Line #", "Operator"]);
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let line_length = line.len();

        if byte_index <= span.0 && byte_index + line_length >= span.1 {
            table.add_row(Row::new(vec![
                Cell::new(file_path.to_str().unwrap()).style_spec("Fr"),
                Cell::new(&(index + 1).to_string()).style_spec("Fr"),
                Cell::new(&line).style_spec("Fr"),
            ]));
            break;
        }

        byte_index += line_length + 1; // +1 for the newline character
    }

    table.printstd();

    Ok(())
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

    let parent_dir = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    std::env::set_current_dir(&parent_dir).expect("Failed to change directory");

    for mutant in &mutants {
        if mutant.status() == MutationStatus::Survived {
            // println!("{}", mutant);
            // println!("Current directory: {:?}", std::env::current_dir().unwrap());
            let span = mutant.span();
            let span_usize = (span.0 as usize, span.1 as usize);
            print_line_in_span(Path::new(mutant.path()), span_usize).unwrap();
        }
    }

    // Change to the parent directory
    let parent_dir = Path::new("..");
    std::env::set_current_dir(&parent_dir).expect("Failed to change directory");
    println!("{}", "Cleaning up temp files".green());
    // Remove the ./temp directory
    let temp_dir = Path::new("./temp");
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir).expect("Failed to remove ./temp directory");
    }

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
