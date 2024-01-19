use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    process::Command,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use crate::{
    cli::Args,
    config::{is_test_failed, Config},
    file_manager::{setup_temp_dirs, write_mutation_to_temp_file, Defer},
    handlers::mutator::{calculate_mutation_score, Mutant, MutationStatus},
    reporter::{mutants_progress_bar, mutation_test_summary_table, print_table},
    utils::*,
};

use colored::*;
use rayon::prelude::*;

pub fn process_mutants(mutants: &mut Vec<Mutant>, args: Args, config: Config) {
    let original_dir = std::env::current_dir().unwrap();
    let total_mutants = mutants.len();
    let bar = mutants_progress_bar(total_mutants);

    let destroyed = Arc::new(AtomicUsize::new(0));
    let survived = Arc::new(AtomicUsize::new(0));
    let pending = Arc::new(AtomicUsize::new(total_mutants));

    let (temp_dir, temp_src_dir) = setup_temp_dirs(config.language()).unwrap();

    // this handles cleanup of the temp directorys after this function returns.
    let _cleanup = Defer(Some(|| {
        let _ = fs::remove_dir_all(&temp_dir);
    }));

    mutants.par_iter_mut().for_each(|m| {
        let temp_file = write_mutation_to_temp_file(m, temp_src_dir.clone(), config.clone())
            .expect("Failed to setup test infrastructure");

        let mut contents = String::new();
        let mut file = File::open(&temp_file).expect("File path doesn't seem to work...");
        file.read_to_string(&mut contents).unwrap();

        let mut original_bytes = contents.into_bytes();
        replace_bytes(&mut original_bytes, m.span_start() as usize, &m.bytes());
        contents = String::from_utf8_lossy(original_bytes.as_slice()).into_owned();

        // After modifying the contents, write it back to the temp file
        let mut file = OpenOptions::new().write(true).open(&temp_file).unwrap();

        // modify string of contents, then write back to temp file
        file.write_all(contents.as_bytes()).unwrap();

        // Build the project
        let build_output = Command::new(config.test_runner())
            .arg(config.build_command())
            .output()
            .expect("Failed to execute build command");

        // run_test_suite
        let output = Command::new(config.test_runner())
            .arg(config.test_command())
            .output()
            .expect("Failed to execute command");

        match build_output.status.code() {
            Some(0) => {
                match output.status.code() {
                    Some(0) => {
                        println!("Build was successful");
                        println!("Test suite passed");
                        m.set_status(MutationStatus::Survived);
                        survived.fetch_add(1, Ordering::SeqCst);
                        pending.fetch_sub(1, Ordering::SeqCst);
                    }
                    Some(_) => {
                        println!("Test suite failed");
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("stderr: {}", stderr);
                        if is_test_failed(&stderr, &config.language()) {
                            destroyed.fetch_add(1, Ordering::SeqCst);
                            pending.fetch_sub(1, Ordering::SeqCst);
                            m.set_status(MutationStatus::Killed);
                        }
                    }
                    None => {
                        println!("Test suite was killed by a signal or crashed");
                        // Handle this case
                    }
                }
            }
            Some(_) => {
                // println!("Build failed");
                destroyed.fetch_add(1, Ordering::SeqCst);
                pending.fetch_sub(1, Ordering::SeqCst);
                m.set_status(MutationStatus::Killed);
            }
            None => {
                println!("Build was killed by a signal or crashed");
                // Handle this case
            }
        }

        // Change back to the original directory at the end
        if let Err(e) = std::env::set_current_dir(&original_dir) {
            eprintln!("Failed to change back to the original directory: {}", e);
        }

        // Note: the /temp dir and its contents will be deleted automatically,
        // so this might seem redundant. However, Hunter deletes the file
        // as soon as possible to help prevent running out of space when testing very large projects.
        if let Err(e) = std::fs::remove_file(&temp_file) {
            eprintln!("Failed to delete temporary file: {}", e);
        }

        bar.inc(1);
    });

    bar.finish_with_message("All mutants processed!");
    let score = calculate_mutation_score(
        destroyed.load(Ordering::SeqCst) as f64,
        total_mutants as f64,
    );
    let summary_table = mutation_test_summary_table(
        total_mutants,
        pending.load(Ordering::SeqCst).to_string(),
        destroyed.load(Ordering::SeqCst).to_string(),
        survived.load(Ordering::SeqCst).to_string(),
        score,
    );

    // Note: cleanup is handled automatically when this function
    // returns & the Defer object is dropped.
    println!("{}", "Cleaning up temp files".cyan());

    print_table(args.output_path, summary_table).unwrap();
}
