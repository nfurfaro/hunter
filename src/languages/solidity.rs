use crate::config::LanguageConfig;
use crate::languages::common::Language;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process::{self, Command},
};

use tempfile::Builder;
use tempfile::TempDir;

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
        vec!["temp", "target", "test", "tests", "lib"]
    }

    fn filter_tests(&self) -> bool {
        FILTER_TESTS
    }

    fn setup_test_infrastructure(&self, debug_mode: bool) -> io::Result<(TempDir, PathBuf)> {
        // Create a temp directory with a specific prefix
        let temp_dir = Builder::new()
            .prefix("Hunter_temp_mutations_")
            .tempdir_in(std::env::temp_dir())?;

            // let temp_dir = Builder::new()
            // .prefix("Hunter_temp_mutations_")
            // .tempdir_in(std::env::current_dir()?)?;

        // Inside /temp, create a src/ directory
        let src_dir = temp_dir.path().join("src");
        fs::create_dir_all(src_dir.clone())?;
        Ok((temp_dir, src_dir))
    }

    // fn setup_test_infrastructure(&self, project_path: &PathBuf) -> io::Result<PathBuf> {
    //     // Create a new directory named _hunter_temp_ inside project_path
    //     let temp_dir_path = project_path.join("_hunter_temp_");
    //     fs::create_dir_all(&temp_dir_path)?;

    //     // Copy the contents of the project path into _hunter_temp_
    //     for entry in fs::read_dir(project_path)? {
    //         let entry = entry?;
    //         let path = entry.path();
    //         if path.is_file() {
    //             fs::copy(&path, &temp_dir_path.join(path.file_name().unwrap()))?;
    //         }
    //     }

    //     Ok(temp_dir_path)
    // }


    fn test_mutant_project(&self) -> Box<process::Output> {
        let child = Command::new(self.test_runner())
            .arg(self.test_command())
            .stderr(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .spawn()
            .expect("Failed to execute command");

        Box::new(child.wait_with_output().expect("Failed to wait on child"))
    }

    fn build_mutant_project(&self) -> Box<process::Output> {
        let child = Command::new(self.test_runner())
            .arg(self.build_command())
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
