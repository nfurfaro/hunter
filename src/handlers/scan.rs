use crate::cli::{Args, Config};
use crate::mutant::{mutant_builder, Mutant};
use crate::utils::{collect_tokens, find_source_files};
use colored::*;

use std::{io::Result, path::Path};

pub fn analyze(_args: Args, config: Config) -> Result<()> {
    println!("{}", "Initiating source file analysis...".green());
    println!(
        "{}",
        format!("Searching for {} files", config.language().to_str()).green()
    );
    let files = find_source_files(Path::new("."), &config).unwrap_or_else(|_| {
        panic!(
            "No {} files found... Are you in the right directory?",
            config.language().to_str().red()
        )
    });

    println!("{}", "Files found:".cyan());
    for file in &files {
        println!("{}", format!("{}", file.1.as_path().display()).red());
    }

    println!("{}", "Collecting tokens from files".green());

    let (tokens_with_paths, test_count) = collect_tokens(&files).expect("No tokens found");

    println!(
        "{}",
        format!("Analysing {} tokens", tokens_with_paths.len()).green()
    );

    // let bar = ProgressBar::new(tokens_with_paths.len() as u64);
    // bar.set_style(
    //     ProgressStyle::default_bar()
    //         .template(
    //             "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
    //         )
    //         .unwrap()
    //         .progress_chars("#>-"),
    // );

    let mut mutants: Vec<Mutant> = vec![];
    for entry in tokens_with_paths {
        let path = entry.1.as_path();
        let spanned_token = entry.0.clone();
        // let span = spanned_token.to_span();
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
        // bar.inc(1);
    }
    // bar.finish_with_message("Done processing tokens.");

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
