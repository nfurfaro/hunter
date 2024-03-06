use std::{
    fs, io,
    path::PathBuf,
    process::{self, Command},
    sync::Mutex,
};

use fs_extra::{
    dir::CopyOptions as DirCopyOptions,
    error::Error,
    file::{self, CopyOptions as FileCopyOptions},
};
use regex::Regex;
use tempfile::{Builder, TempDir};
use walkdir::WalkDir;

use crate::{config::LanguageConfig, handlers::mutator::Mutant, languages::common::Language};

const NAME: &str = "Solidity";
const EXT: &str = "sol";
const TEST_RUNNER: &str = "forge";
const TEST_COMMAND: &str = "test";
const BUILD_COMMAND: &str = "build";
const MANIFEST_NAME: &str = "Foundry.toml";
const FILTER_TESTS: bool = false;

#[derive(Clone)]
pub struct SolidityConfig;

impl LanguageConfig for SolidityConfig {
    fn language(&self) -> Language {
        Language::Solidity
    }

    fn name(&self) -> &'static str {
        NAME
    }

    fn ext(&self) -> &'static str {
        EXT
    }

    fn test_runner(&self) -> &'static str {
        TEST_RUNNER
    }

    fn test_command(&self) -> &'static str {
        TEST_COMMAND
    }

    fn build_command(&self) -> &'static str {
        BUILD_COMMAND
    }

    fn manifest_name(&self) -> &'static str {
        MANIFEST_NAME
    }

    fn excluded_dirs(&self) -> Vec<&'static str> {
        vec!["temp", "target", "test", "tests", "lib", "node_modules"]
    }

    fn filter_tests(&self) -> bool {
        FILTER_TESTS
    }

    fn test_regex(&self) -> Option<Regex> {
        None
    }

    fn comment_regex(&self) -> Regex {
        Regex::new(r"//.*|/\*(?s:.*?)\*/").unwrap()
    }

    fn literal_regex(&self) -> Regex {
        Regex::new(r#""([^"\\]|\\.)*""#).unwrap()
    }

    fn setup_test_infrastructure(&self) -> Result<TempDir, Error> {
        // Create a temp directory with a specific prefix
        let temp_dir = Builder::new()
            .prefix("Hunter_temp_mutations_")
            .tempdir_in(std::env::temp_dir())?;

        // Get the current directory
        let current_dir = std::env::current_dir()?;

        // Define copy options for directories
        let mut dir_options = DirCopyOptions::new();
        dir_options.overwrite = true;

        // Define copy options for files
        let mut file_options = FileCopyOptions::new();
        file_options.overwrite = true;

        // Walk through the current directory and copy files/directories to the temp directory
        for entry in WalkDir::new(&current_dir) {
            let entry = entry.expect("Failed to get entry");
            let path = entry.path();

            // Skip certain files and directories
            if path.to_string_lossy().contains("cache")
                || path.to_string_lossy().contains("cache")
                || path.to_string_lossy().contains("broadcast")
                || path.to_string_lossy().contains("README.md")
            {
                continue;
            }

            // Copy the file or directory to the temp directory
            if path.is_file() {
                file::copy(
                    path,
                    temp_dir.path().join(path.strip_prefix(&current_dir)?),
                    &file_options,
                )?;
            } else if path.is_dir() {
                fs_extra::dir::create_all(
                    temp_dir.path().join(path.strip_prefix(&current_dir)?),
                    true,
                )?;
            }
        }

        Ok(temp_dir)
    }

    fn copy_src_file(
        &self,
        temp_dir: &TempDir,
        mutant: &Mutant,
        mutex: Option<&Mutex<()>>,
    ) -> io::Result<PathBuf> {
        // For Solidity, mutex should always be Some
        if mutex.is_some() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Mutex is not needed for Solidity",
            ));
        }

        // Join temp_dir with mutant.path()
        let temp_file = temp_dir
            .path()
            .join(mutant.path().strip_prefix("./").unwrap());

        // Create directories in temp_file path if they do not exist
        if let Some(parent) = temp_file.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(mutant.path(), &temp_file)?;

        Ok(temp_file)
    }

    fn test_mutant_project(&self) -> Box<process::Output> {
        let child = Command::new(self.test_runner())
            .arg(self.test_command())
            .arg("--no-auto-detect")
            .stderr(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .spawn()
            .expect("Failed to execute command");

        Box::new(child.wait_with_output().expect("Failed to wait on child"))
    }

    fn build_mutant_project(&self) -> Box<process::Output> {
        let child = Command::new(self.test_runner())
            .arg(self.build_command())
            .arg("--no-auto-detect")
            .stderr(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .spawn()
            .expect("Failed to execute build command");

        Box::new(child.wait_with_output().expect("Failed to wait on child"))
    }

    fn clone_box(&self) -> Box<dyn LanguageConfig + Send + Sync> {
        Box::new(self.clone())
    }
}
