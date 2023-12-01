use crate::cli::Args;
use crate::config::Config;
use crate::parallel::parallel_process_mutated_tokens;
use crate::token::{mutant_builder, Mutant, MutationStatus};
use crate::utils::{collect_tokens, find_source_files, print_line_in_span};
use colored::*;
use prettytable::{Cell, Row, Table};
use std::{io::Result, path::Path};

pub fn mutate(args: Args, config: Config) -> Result<()> {
    // add a [workspace] to the project manifest
    // modify_toml(config);

    println!("{}", "Initiating source file analysis...".green());
    println!(
        "{}",
        format!("Searching for {} files", config.language().name()).green()
    );
    let files = find_source_files(Path::new("."), &config).unwrap_or_else(|_| {
        panic!(
            "No {} files found... Are you in the right directory?",
            config.language().name().red()
        )
    });

    println!("{}", "Files found:".cyan());
    for file in &files {
        println!("{}", format!("{}", file.1.as_path().display()).red());
    }

    println!("{}", "Collecting tokens from files".green());
    let config_clone = config.clone();

    let (tokens_with_paths, test_count) =
        collect_tokens(&files, &config_clone).expect("No tokens found");

    println!(
        "{}",
        format!("Analysing {} tokens", tokens_with_paths.len()).green()
    );

    let mut mutants: Vec<Mutant> = vec![];
    for entry in tokens_with_paths {
        let path = entry.1.as_path();
        let spanned_token = entry.0.clone();
        let maybe_mutant = mutant_builder(
            entry.2,
            spanned_token.token().clone(),
            spanned_token.span(),
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
        format!("Test runs required: {}", num_mutants * test_count).magenta()
    );

    println!("{}", "Running tests...".green());
    parallel_process_mutated_tokens(&mut mutants, config);

    if args.verbose {
        // Create a new table
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Surviving Mutants").style_spec("Fmb")
        ]));
        table.add_row(Row::new(vec![
            Cell::new("Source file:").style_spec("Fyb"),
            Cell::new("Line #:").style_spec("Fyb"),
            Cell::new("Mutant context:").style_spec("Fyb"),
            Cell::new("Original:").style_spec("Fyb"),
        ]));

        for mutant in &mutants {
            if mutant.status() == MutationStatus::Survived
                || mutant.status() == MutationStatus::Pending
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
    }

    println!("{}", "Cleaning up temp files".cyan());

    let _current_dir = std::env::current_dir().unwrap();

    // @fix
    // Remove the ./temp directory
    // let temp_dir = Path::new("./temp");
    // if temp_dir.exists() {
    //     std::fs::remove_dir_all(&temp_dir).expect("Failed to remove ./temp directory");
    // }

    Ok(())
}
