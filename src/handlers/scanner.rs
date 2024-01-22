use crate::{
    cli::Args,
    config::Config,
    file_manager::find_source_file_paths,
    filters::test_regex,
    handlers::mutator::{mutants, Mutant},
    reporter::count_tests,
    token::MetaToken,
    utils::collect_tokens,
};
use colored::*;
use std::{
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct ScanResult {
    paths: Vec<PathBuf>,
    contains_unit_tests: Vec<PathBuf>,
    meta_tokens: Vec<MetaToken>,
    test_count: usize,
    mutants: Vec<Mutant>,
}

impl ScanResult {
    pub fn new(
        paths: Vec<PathBuf>,
        contains_unit_tests: Vec<PathBuf>,
        meta_tokens: Vec<MetaToken>,
        test_count: usize,
        mutants: Vec<Mutant>,
    ) -> ScanResult {
        ScanResult {
            paths,
            contains_unit_tests,
            meta_tokens,
            test_count,
            mutants,
        }
    }

    pub fn paths(&self) -> &Vec<PathBuf> {
        &self.paths
    }

    pub fn contains_unit_tests(&self) -> &Vec<PathBuf> {
        &self.contains_unit_tests
    }

    pub fn meta_tokens(&self) -> &Vec<MetaToken> {
        &self.meta_tokens
    }

    pub fn test_count(&self) -> usize {
        self.test_count
    }

    pub fn mutants(&mut self) -> &mut Vec<Mutant> {
        &mut self.mutants
    }
}

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

    let mut test_count = 0;
    let mut contains_unit_tests: Vec<PathBuf> = vec![];

    for path in &paths {
        let num_tests = count_tests(path, test_regex(&config.language()));
        if num_tests > 0 {
            contains_unit_tests.push(path.clone());
            test_count += num_tests;
        }
    }

    // @todo consider adding a switch here to mutate all tokens in source files, or only those in files with unit tests
    let meta_tokens = collect_tokens(contains_unit_tests.clone(), config).expect("No tokens found");

    // @todo consider moving this to mutator.rs
    let mutants = mutants(&meta_tokens, args.random);

    Ok(ScanResult::new(
        paths,
        contains_unit_tests,
        meta_tokens,
        test_count,
        mutants,
    ))
}
