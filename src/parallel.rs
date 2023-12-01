use std::{
    fs::{copy, create_dir_all, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use crate::config::Config;
use crate::token::{Mutant, MutationStatus};
use crate::utils::*;
use indicatif::{ProgressBar, ProgressStyle};
use prettytable::{Cell, Row, Table};
use rayon::iter::ParallelIterator;
extern crate rayon;
use rayon::prelude::*;

pub fn parallel_process_mutated_tokens(mutants: &mut Vec<Mutant>, config: Config) {
    let total_mutants = mutants.len();
    let destroyed = Arc::new(AtomicUsize::new(0));
    let survived = Arc::new(AtomicUsize::new(0));
    let pending = Arc::new(AtomicUsize::new(total_mutants));

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
        let temp_file_path = format!("./src/main_{}.{}", thread_index, config.language().ext());
        // @fix use a temp file path that is unique to the mutant
        // currently Nargo demands that there is a main.nr file or a lib.nr in the directory
        // let temp_file_path = format!("./src/main.nr");
        copy(source_path, &temp_file_path).expect("Failed to copy file");

        let mut original_bytes = contents.into_bytes();
        replace_bytes(&mut original_bytes, m.span_start() as usize, &m.bytes());
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
        let output = Command::new(config.test_runner())
            .arg(config.test_command())
            .output()
            .expect("Failed to execute command");

        // Check the output
        if output.status.success() {
            // tests passed indicating mutant survived !
            m.set_status(MutationStatus::Survived);
            survived.fetch_add(1, Ordering::SeqCst);
            pending.fetch_sub(1, Ordering::SeqCst);
        } else {
            // test failed indicating mutant was killed !
            let stderr = String::from_utf8_lossy(&output.stderr);
            // @fix this is a brittle hack to get around the fact that
            // the compiler fails to compile the mutated tests because the
            // mutated code contains contraints that are always false !
            // let re = Regex::new(r"aborting due to \d+ previous errors").unwrap();
            // if re.is_match(&stderr) {
            //     panic!("test aborted due to previous errors");
            // }

            if (stderr.contains("test failed") || stderr.contains("FAILED"))
                && !stderr.contains("aborting due to 1 previous errors")
            {
                println!("test failed and contains test failed");
                eprint!("{}", stderr);
                destroyed.fetch_add(1, Ordering::SeqCst);
                pending.fetch_sub(1, Ordering::SeqCst);
                m.set_status(MutationStatus::Killed);
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
    std::env::set_current_dir(parent_dir).expect("Failed to change directory");

    bar.finish_with_message("All mutants processed!");

    let mutation_score = (destroyed.load(Ordering::SeqCst) as f64 / total_mutants as f64) * 100.0;
    let mutation_score_string = format!("{:.2}%", mutation_score);

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Mutation Test Breakdown").style_spec("Fyb"),
        Cell::new("Value").style_spec("Fyb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants Total:").style_spec("Fbb"),
        Cell::new(&total_mutants.to_string()).style_spec("Fbb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants Pending...").style_spec("Fyb"),
        Cell::new(&pending.load(Ordering::SeqCst).to_string()).style_spec("Fyb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants Destroyed:").style_spec("Fgb"),
        Cell::new(&destroyed.load(Ordering::SeqCst).to_string()).style_spec("Fgb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutants Survived:").style_spec("Fmb"),
        Cell::new(&survived.load(Ordering::SeqCst).to_string()).style_spec("Fmb"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Mutation score:").style_spec("Fcb"),
        Cell::new(&mutation_score_string).style_spec("Fcb"),
    ]));

    table.printstd();
}
