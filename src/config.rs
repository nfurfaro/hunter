use crate::languages;
use crate::languages::common::Language;
use std::sync::Arc;
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
    fn is_test_failed(&self, stderr: &str) -> bool;
    fn excluded_dirs(&self) -> Vec<&'static str>;
    fn setup_test_infrastructure(&self) -> io::Result<(Arc<TempDir>, Arc<PathBuf>)>;
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
