use std::{
    // ffi::OsString,
    // fs::{self, write, File, OpenOptions},
    fs::{self, File, OpenOptions},
    // io::{BufReader, Error, Read, Result},
    io::{BufReader, Read, Result, Write},
    path::{Path, PathBuf},
    process::Command,
};

extern crate rayon;
use crate::utils::*;
use noirc_frontend::token::SpannedToken;
use rayon::prelude::*;

pub fn parallel_process_mutated_tokens(
    mutated_tokens_with_paths: &mut Vec<(SpannedToken, PathBuf)>,
) {
    mutated_tokens_with_paths
        .par_iter_mut()
        .for_each(|(token, path)| {
            let mut contents = String::new();
            // Open the file at the given path in write mode
            let mut file = OpenOptions::new()
                .write(true)
                .read(true)
                .open(&path.as_path())
                .expect("File path doesn't seem to work...");
            // Read the file's contents into a String
            file.read_to_string(&mut contents).unwrap();
            let mut original_bytes = contents.into_bytes();
            // println!("Original Bytes: {:?}", original_bytes);
            // @todo fix unwrap here
            let replacement_bytes = get_bytes_from_token(token.clone().into_token());
            // println!("Replacement Bytes: {:?}", replacement_bytes.unwrap());

            // mutate original_bytes
            replace_bytes(
                &mut original_bytes,
                token.to_span().start() as usize,
                token.to_span().end() as usize,
                replacement_bytes.unwrap(),
            );

            contents = String::from_utf8_lossy(original_bytes.as_slice()).into_owned();

            // After modifying the contents, write it back to the file
            let mut file = OpenOptions::new().write(true).open(path).unwrap();

            // modify string of contents, then write back to temp file
            file.write_all(contents.as_bytes()).unwrap();

            // run_test_suite
            let output = Command::new("nargo test")
                // .arg("--workspace")
                .output()
                .expect("Failed to execute command");

            // Check the output
            if output.status.success() {
                // Command was successful
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("Command output: {}", stdout);
            } else {
                // Command failed
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Command failed with error: {}", stderr);
            }
        });
}
