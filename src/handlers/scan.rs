use crate::cli::Args;
use crate::config::Config;
use crate::mutant::{mutant_builder, Mutant};
use crate::utils::{collect_tokens, find_source_files};
use colored::*;

use std::{io::Result, path::Path};

pub fn analyze(_args: Args, config: Config) -> Result<()> {
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

    let (tokens_with_paths, test_count) = collect_tokens(&files, &config).expect("No tokens found");

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
        format!("Runs of test suite required: {}", num_mutants * test_count).magenta()
    );

    Ok(())
}
