// use colored::*;
// use colored::*;
use std::{
    fs::{copy, create_dir_all, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

use indicatif::{ProgressBar, ProgressStyle};
use prettytable::{row, Cell, Row, Table};
use rayon::iter::ParallelIterator;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::mutant::{Mutant, MutationStatus};
use crate::utils::*;

extern crate rayon;
use rayon::prelude::*;

pub fn parallel_process_mutated_tokens(mutants: &mut Vec<Mutant>) {
    let total_mutants = mutants.len();
    let destroyed = Arc::new(AtomicUsize::new(0));
    let surviving = Arc::new(AtomicUsize::new(total_mutants));

    let bar = ProgressBar::new(total_mutants as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let temp_dir = PathBuf::from("./temp");
    if !temp_dir.exists() {
        create_dir_all(&temp_dir).expect("Failed to create directory");
    }

    std::env::set_current_dir(&temp_dir).expect("Failed to change directory");

    mutants.par_iter_mut().for_each(|m| {
        let thread_index = rayon::current_thread_index().unwrap_or(0);
        let mut contents = String::new();

        let original_path = Path::new(m.path());
        let parent_dir = Path::new("../src");
        let file_name = original_path.file_name().expect("Failed to get file name");
        let source_path = parent_dir.join(file_name);

        // Open the file at the given path in write mode
        let mut file = File::open(source_path.clone()).expect("File path doesn't seem to work...");
        // Read the file's contents into a String
        file.read_to_string(&mut contents).unwrap();

        // Include the thread's index in the file name
        let temp_file_path = format!("./src/main_{}.nr", thread_index);
        copy(source_path, &temp_file_path).expect("Failed to copy file");

        let mut original_bytes = contents.into_bytes();
        replace_bytes(&mut original_bytes, m.start() as usize, &m.bytes());
        contents = String::from_utf8_lossy(original_bytes.as_slice()).into_owned();

        // After modifying the contents, write it back to the file
        let mut file = OpenOptions::new()
            .write(true)
            .create(true) // Create the file if it doesn't exist
            .open(temp_file_path.clone())
            .unwrap();

        // modify string of contents, then write back to temp file
        file.write_all(contents.as_bytes()).unwrap();

        // run_test_suite
        let output = Command::new("nargo")
            .arg("test")
            // .arg("--package")
            // .arg("hunter")
            .output()
            .expect("Failed to execute command");

        // Check the output
        if output.status.success() {
            // tests passed indicating mutant survived !
            m.set_status(MutationStatus::Survived);
        } else {
            // test failed indicating mutant was killed !
            let stderr = String::from_utf8_lossy(&output.stderr);
            // if stderr.contains("test failed") || !output.status.success() {
            if stderr.contains("test failed") {
                eprint!("{}", stderr);
                destroyed.fetch_add(1, Ordering::SeqCst);
                surviving.fetch_sub(1, Ordering::SeqCst);
                m.set_status(MutationStatus::Survived);
            }
        }

        // Clean up the temporary file
        // std::fs::remove_file(&temp_file_path).expect("Failed to remove temporary file");

        bar.inc(1);
    });

    let parent_dir = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    std::env::set_current_dir(&parent_dir).expect("Failed to change directory");

    bar.finish_with_message("All mutants processed!");

    let mutation_score = (destroyed.load(Ordering::SeqCst) as f64 / total_mutants as f64) * 100.0;
    let mutation_score_string = format!("{:.2}%", mutation_score);

    let mut table = Table::new();
    table.add_row(row!["Metric", "Value"]);
    table.add_row(Row::new(vec![
        Cell::new("Total mutants").style_spec("Fr"),
        Cell::new(&total_mutants.to_string()).style_spec("Frb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants destroyed").style_spec("Fc"),
        Cell::new(&destroyed.load(Ordering::SeqCst).to_string()).style_spec("Fcb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutation score").style_spec("Fb"),
        Cell::new(&mutation_score_string).style_spec("Fbb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Surviving mutants").style_spec("Fm"),
        Cell::new(&surviving.load(Ordering::SeqCst).to_string()).style_spec("Fmb"),
    ]));
    // Print the table to stdout
    table.printstd();
}
