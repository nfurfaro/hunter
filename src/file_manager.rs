use crate::{config::LanguageConfig, handlers::mutator::Mutant, languages::common::Language};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Result, Write},
    path::{Path, PathBuf},
};

pub struct Defer<T: FnOnce()>(pub Option<T>);
// a wrapper around a closure that is called when the Defer object is dropped.
impl<T: FnOnce()> Drop for Defer<T> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}

pub fn find_source_file_paths<'a>(
    dir_path: &'a Path,
    config: &'a dyn LanguageConfig,
) -> Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = vec![];

    if dir_path.is_dir() {
        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            let path_buf = entry.path();
            if path_buf.is_dir() {
                // Skipped directories are not included in the results
                if config
                    .excluded_dirs()
                    .iter()
                    .any(|&dir| path_buf.ends_with(dir) || path_buf.starts_with(dir))
                {
                    continue;
                }
                let path_result = find_source_file_paths(&path_buf, config);
                match path_result {
                    Ok(sub_results_paths) => {
                        paths.extend(sub_results_paths.clone());
                    }
                    Err(_) => continue,
                }
            } else if path_buf
                .extension()
                .map_or(false, |extension| extension == config.ext())
            {
                paths.push(path_buf);
            }
        }
    }
    if paths.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No files found",
        ));
    }

    Ok(paths)
}

pub fn setup_temp_dirs(language: Language) -> io::Result<(PathBuf, PathBuf)> {
    // Create a ./temp directory
    let temp_dir = PathBuf::from("./temp");
    fs::create_dir_all(&temp_dir)?;

    // Inside /temp, create a src/ directory
    let src_dir = temp_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    let mut manifest = match language {
        Language::Noir => File::create(temp_dir.join("Nargo.toml"))?,
    };

    match language {
        Language::Noir => {
            write!(
                manifest,
                r#"
                [package]
                name = "hunter_temp"
                type = "lib"
                authors = ["Hunter"]
                compiler_version = "0.22.0"
                "#
            )?;
            let _ = File::create(src_dir.join("lib.nr"))?;
        }
    }

    Ok((temp_dir, src_dir))
}

pub fn write_mutation_to_temp_file(
    mutant: &Mutant,
    src_dir: PathBuf,
    lang_ext: &'static str,
) -> io::Result<PathBuf> {
    let temp_file = src_dir.join(format!("mutation_{}.{}", mutant.id(), lang_ext));
    fs::copy(mutant.path(), &temp_file)?;

    let mut lib_file = OpenOptions::new()
        .append(true)
        .open(src_dir.join(format!("lib.{}", lang_ext)))?;
    writeln!(lib_file, "mod mutation_{};", mutant.id())?;

    Ok(temp_file)
}
