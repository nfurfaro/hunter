use crate::cli::{Args, LangConfig};
use crate::mutant::{mutant_builder, Mutant, MutationStatus};
use crate::parallel::parallel_process_mutated_tokens;
use crate::utils::{collect_tokens, find_source_files, print_line_in_span};
use colored::*;
use prettytable::{Cell, Row, Table};
use std::{io::Result, path::Path};

pub fn mutate(_args: Args, config: LangConfig) -> Result<()> {
    // add a [workspace] to the project manifest
    // modify_toml();

    println!("{}", "Initiating source file analysis...".green());
    println!("{}", format!("Searching for {} files", config.name).green());
    let files = find_source_files(config.ext, Path::new(".")).expect(&format!(
        "No {} files found... Are you in the right directory?",
        config.name.red()
    ));

    println!("{}", "Files found:".cyan());
    for file in &files {
        println!("{}", format!("{}", file.1.as_path().display()).red());
    }

    println!("{}", "Collecting tokens from files".green());

    let (tokens_with_paths, test_count) = collect_tokens(&files).expect(
        "No tokens found");

    println!(
        "{}",
        format!("Analysing {} tokens", tokens_with_paths.len()).green()
    );

    let mut mutants: Vec<Mutant> = vec![];
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

    let num_mutants: usize = mutants.len();

    println!(
        "{}",
        format!("Mutable tokens found: {}", num_mutants).cyan()
    );
    println!(
        "{}",
        format!("Runs of test suite required: {}", num_mutants * test_count).magenta()
    );

    println!("{}", "Running tests...".green());
    parallel_process_mutated_tokens(&mut mutants);

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
