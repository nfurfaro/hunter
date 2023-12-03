use crate::cli::Args;
use crate::config::Config;
use crate::reporter::ScanResult;
use crate::token::{mutant_builder, Mutant};
use crate::utils::{collect_tokens, count_tests, find_source_file_paths};
use colored::*;

use std::path::Path;

pub fn analyze(_args: Args, config: &Config) -> ScanResult {
    let paths = find_source_file_paths(Path::new("."), config).unwrap_or_else(|_| {
        panic!(
            "No {} files found... Are you in the right directory?",
            config.language().name().red()
        )
    });

    let paths_clone = paths.clone();
    let test_count = count_tests(paths.clone(), config);
    let tokens_with_paths = collect_tokens(paths_clone, config).expect("No tokens found");

    let mut mutants: Vec<Mutant> = vec![];
    for entry in &tokens_with_paths {
        let path = entry.1.clone();
        let spanned_token = entry.0.clone();
        let maybe_mutant = mutant_builder(
            entry.2,
            spanned_token.token().clone(),
            spanned_token.span(),
            path,
        );
        match maybe_mutant {
            None => continue,
            Some(m) => mutants.push(m),
        }
    }

    ScanResult::new(paths, tokens_with_paths, test_count, mutants)
}
