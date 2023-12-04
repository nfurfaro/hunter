use crate::cli::Args;
use crate::config::Config;
use crate::file_manager::find_source_file_paths;
use crate::reporter::ScanResult;
use crate::token::{mutant_builder, Mutant};
use crate::utils::{collect_tokens, count_tests};
use colored::*;
use std::io::{Error, ErrorKind, Result};

use std::path::Path;

pub fn analyze(_args: Args, config: &Config) -> Result<ScanResult> {
    let paths = find_source_file_paths(Path::new("."), config).map_err(|_| {
        let err_msg = format!("No {} files found... Are you in the right directory?",
            config.language().name().red());
        Error::new(ErrorKind::Other, err_msg)
    })?;

    let test_count = count_tests(paths.clone(), config);
    let meta_tokens = collect_tokens(paths.clone(), config).expect("No tokens found");

    let mut mutants: Vec<Mutant> = vec![];
    for entry in &meta_tokens {
        let path = entry.src().clone();
        let maybe_mutant = mutant_builder(entry.id(), entry.token().clone(), entry.span(), path);
        match maybe_mutant {
            None => continue,
            Some(m) => mutants.push(m),
        }
    }

    Ok(ScanResult::new(paths, meta_tokens, test_count, mutants))
}
