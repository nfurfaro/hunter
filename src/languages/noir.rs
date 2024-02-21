use crate::config::LanguageConfig;
use crate::handlers::mutator::Mutant;
use crate::languages::common::Language;
use std::sync::Mutex;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::PathBuf,
    process::{self, Command},
};

// use lazy_static::lazy_static;

use tempfile::Builder;
use tempfile::TempDir;

const NAME: &str = "Noir";
const EXT: &str = "nr";
const TEST_RUNNER: &str = "nargo";
const TEST_COMMAND: &str = "test";
const BUILD_COMMAND: &str = "build";
const MANIFEST_NAME: &str = "Nargo.toml";
const FILTER_TESTS: bool = true;

#[derive(Clone)]
pub struct NoirConfig;

impl LanguageConfig for NoirConfig {
    fn language(&self) -> Language {
        Language::Noir
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
        vec!["temp", "target", "test", "tests"]
    }

    fn filter_tests(&self) -> bool {
        FILTER_TESTS
    }

    fn setup_test_infrastructure(&self) -> io::Result<TempDir> {
        // Create a temp directory with a specific prefix
        let temp_dir = Builder::new()
            .prefix("Hunter_temp_mutations_")
            .tempdir_in(std::env::temp_dir())?;

        // Inside /temp, create a src/ directory
        let src_dir = temp_dir.path().join("src");
        fs::create_dir_all(src_dir.clone())?;

        let mut manifest = File::create(temp_dir.path().join(self.manifest_name()))?;

        write!(
            manifest,
            r#"[package]
name = "hunter_temp"
type = "lib"
authors = ["Hunter"]
compiler_version = ">=0.22.0"

[dependencies]
        "#
        )?;
        let _ = File::create(src_dir.join("lib.nr"))?;

        Ok(temp_dir)
    }

    fn copy_src_file(
        &self,
        temp_dir: &TempDir,
        mutant: &Mutant,
        mutex: Option<&Mutex<()>>,
    ) -> io::Result<PathBuf> {
        // For Noir, mutex should always be Some
        if mutex.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Mutex is not initialized",
            ));
        }

        let src_dir = temp_dir.path().join("src");

        let temp_file = src_dir.join(format!("mutation_{}.{}", mutant.id(), self.ext()));
        fs::copy(mutant.path(), &temp_file)?;

        // Lock the mutex before writing to the file
        let _guard = mutex.unwrap().lock().unwrap();

        let mut lib_file = OpenOptions::new()
            .write(true)
            .open(src_dir.join(format!("lib.{}", self.ext())))?;
        writeln!(lib_file, "mod mutation_{};", mutant.id())?;

        Ok(temp_file)
    }

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
