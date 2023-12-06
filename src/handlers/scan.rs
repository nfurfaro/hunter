use crate::cli::Args;
use crate::config::Config;
use crate::file_manager::find_source_file_paths;
use crate::reporter::ScanResult;
use crate::token::{mutant_builder, Mutant};
use crate::utils::{collect_tokens, count_tests, test_regex};
use colored::*;
use std::io::{Error, ErrorKind, Result};

use std::path::Path;

pub fn scan(args: Args, config: &Config) -> Result<ScanResult> {
    let source_path = args
        .source_path
        .clone()
        .unwrap_or(Path::new(".").to_path_buf());
    let paths = if source_path.is_file() {
        vec![source_path]
    } else {
        find_source_file_paths(source_path.as_path(), config).map_err(|_| {
            let err_msg = format!(
                "No {} files found... Are you in the right directory?",
                config.language().name().red()
            );
            Error::new(ErrorKind::Other, err_msg)
        })?
    };

    let test_count = count_tests(paths.clone(), test_regex(&config.language()), config);
    let meta_tokens = collect_tokens(paths.clone(), config).expect("No tokens found");

    let mut mutants: Vec<Mutant> = vec![];
    for entry in &meta_tokens {
        let path = entry.src().clone();
        dbg!(&path);
        let maybe_mutant = mutant_builder(entry.id(), entry.token().clone(), entry.span(), path);
        match maybe_mutant {
            None => continue,
            Some(m) => mutants.push(m),
        }
    }

    Ok(ScanResult::new(paths, meta_tokens, test_count, mutants))
}
