use crate::languages::common::Language;
use crate::{handlers::mutator::Mutant, languages};
use fs_extra::error::Error;
use regex::Regex;
use std::sync::Mutex;
use std::{io, path::PathBuf, process};
use tempfile::TempDir;

pub trait LanguageConfig {
    fn language(&self) -> languages::common::Language;
    fn name(&self) -> &'static str;
    fn ext(&self) -> &'static str;
    fn test_runner(&self) -> &'static str;
    fn test_command(&self) -> &'static str;
    fn build_command(&self) -> &'static str;
    fn manifest_name(&self) -> &'static str;
    fn excluded_dirs(&self) -> Vec<&'static str>;
    fn filter_tests(&self) -> bool;
    fn test_regex(&self) -> Option<Regex>;
    fn comment_regex(&self) -> Regex;
    fn literal_regex(&self) -> Regex;
    fn setup_test_infrastructure(&self) -> Result<TempDir, Error>;
    fn copy_src_file(
        &self,
        temp_dir: &TempDir,
        mutant: &Mutant,
        mutex: Option<&Mutex<()>>,
    ) -> io::Result<PathBuf>;
    fn test_mutant_project(&self) -> Box<process::Output>;
    fn build_mutant_project(&self) -> Box<process::Output>;
    fn clone_box(&self) -> Box<dyn LanguageConfig + Send + Sync>;
}

// @extendable: add a new match arm here to support a new language
pub fn config(language: Language) -> Box<dyn LanguageConfig> {
    match language {
        Language::Noir => Box::new(languages::noir::NoirConfig),
    }
}
