use crate::config::LanguageConfig;
use crate::languages::common::Language;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    os::unix::process::ExitStatusExt,
    path::PathBuf,
    process::{self, Command, Stdio},
};

use std::sync::Arc;
use tempfile::Builder;
use tempfile::TempDir;

#[derive(Clone)]
pub struct NoirConfig;

impl LanguageConfig for NoirConfig {
    fn language(&self) -> Language {
        Language::Noir
    }

    fn name(&self) -> &'static str {
        "Noir"
    }

    fn ext(&self) -> &'static str {
        "nr"
    }

    fn test_runner(&self) -> &'static str {
        "nargo"
    }

    fn test_command(&self) -> &'static str {
        "test"
    }

    fn build_command(&self) -> &'static str {
        "build"
    }

    fn manifest_name(&self) -> &'static str {
        "Nargo.toml"
    }

    fn is_test_failed(&self, stderr: &str) -> bool {
        let stderr = stderr.to_lowercase();
        stderr.contains("test failed")
            || stderr.contains("failed")
            || stderr.contains("fail")
            || stderr.contains("failed constraint")
    }

    fn excluded_dirs(&self) -> Vec<&'static str> {
        vec!["temp", "target", "test", "tests"]
    }

    fn setup_test_infrastructure(&self) -> io::Result<(Arc<TempDir>, Arc<PathBuf>)> {
        // Create a temp directory with a specific prefix
        let temp_dir = Arc::new(
            Builder::new()
                .prefix("Hunter_temp_mutations_")
                .tempdir_in(std::env::temp_dir())?,
        );

        // Inside /temp, create a src/ directory
        let src_dir = Arc::new(temp_dir.path().join("src"));
        fs::create_dir_all(src_dir.as_ref())?;

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

        Ok((temp_dir, src_dir))
    }

    // fn test_mutant_project(&self) -> Box<process::Output> {
    //     Box::new(
    //         Command::new(self.test_runner())
    //             .arg(self.test_command())
    //             .output()
    //             .expect("Failed to execute command"),
    //     )
    // }
    fn test_mutant_project(&self) -> Box<process::Child> {
        let output = Command::new(self.test_runner())
            .arg(self.test_command())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");

        Box::new(output)
    }

    fn build_mutant_project(&self) -> Box<process::Output> {
        let output = Command::new(self.test_runner())
            .arg(self.build_command())
            .output()
            .expect("Failed to execute build command");

        let output_str = String::from_utf8_lossy(&output.stderr);
        if output_str
            .to_lowercase()
            .contains("cannot find a nargo.toml")
        {
            dbg!("No nargo.toml found !!!");
            return Box::new(process::Output {
                status: process::ExitStatus::from_raw(444),
                stdout: vec![],
                stderr: vec![],
            });
        }

        Box::new(output)
    }

    fn clone_box(&self) -> Box<dyn LanguageConfig + Send + Sync> {
        Box::new(self.clone())
    }
}
