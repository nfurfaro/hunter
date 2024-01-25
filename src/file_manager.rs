use crate::{
    config::LanguageConfig, handlers::mutator::Mutant, token::token_as_bytes, utils::replace_bytes,
};
use colored::*;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Result, Write},
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

    // Check if the current directory is in the list of excluded directories
    let current_dir = std::env::current_dir()?;

    if let Some(current_dir_name) = current_dir.file_name() {
        let current_dir_name = current_dir_name.to_string_lossy();
        if config
            .excluded_dirs()
            .iter()
            .any(|dir| dir.trim_end_matches('/') == &*current_dir_name)
        {
            eprintln!(
                "{}",
                format!(
                    "Warning: You are attempting to use Hunter in an excluded directory: {}",
                    current_dir.display()
                )
                .red()
            );
            eprintln!(
                "{}",
                format!(
                    "Excluded directories are set in the languages/{}.rs file",
                    config.name().to_lowercase()
                )
                .yellow()
            );
            std::process::exit(1);
        }
    }

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

pub fn copy_src_to_temp_file(
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

pub fn mutate_temp_file(temp_file: &std::path::PathBuf, m: &mut Mutant) {
    let mut contents = String::new();
    let mut file = File::open(temp_file).expect("File path doesn't seem to work...");
    file.read_to_string(&mut contents).unwrap();

    let mut original_bytes = contents.into_bytes();
    replace_bytes(
        &mut original_bytes,
        m.span_start() as usize,
        &m.bytes(),
        token_as_bytes(&m.token()).unwrap(),
    );
    contents = String::from_utf8_lossy(original_bytes.as_slice()).into_owned();

    // After modifying the contents, write it back to the temp file
    let mut file = OpenOptions::new().write(true).open(temp_file).unwrap();

    // modify string of contents, then write back to temp file
    file.write_all(contents.as_bytes()).unwrap();
}
