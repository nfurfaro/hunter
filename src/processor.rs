use std::{
    fs,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use std::process;

use crate::{
    cli::Args,
    config::LanguageConfig,
    file_manager::{copy_src_to_temp_file, mutate_temp_file, Defer},
    handlers::mutator::{calculate_mutation_score, Mutant, MutationStatus},
    reporter::{mutants_progress_bar, mutation_test_summary_table, print_table},
};

use colored::*;
use rayon::prelude::*;

pub fn process_mutants(
    mutants: &mut Vec<Mutant>,
    args: Args,
    config: Box<dyn LanguageConfig + Send + Sync>,
) {
    let original_dir = std::env::current_dir().unwrap();
    let total_mutants = mutants.len();

    // @todo fix prog bar, currently prints once per thread!
    let bar = mutants_progress_bar(total_mutants);

    let destroyed = Arc::new(AtomicUsize::new(0));
    let survived = Arc::new(AtomicUsize::new(0));
    let pending = Arc::new(AtomicUsize::new(total_mutants));
    let (temp_dir, temp_src_dir) = config.setup_test_infrastructure().unwrap();

    // @note handles cleanup of the temp directories automatically after this function returns.
    let _cleanup = Defer(Some(|| {
        let _ = fs::remove_dir_all(&temp_dir);
    }));

    let extension = config.ext();

    mutants.par_iter_mut().for_each(|m| {
        let temp_file = copy_src_to_temp_file(m, temp_src_dir.clone(), extension)
            .expect("Failed to setup test infrastructure");

        mutate_temp_file(&temp_file, m);

        // Build the project
        let build_output = config.build_mutant_project();

        // run_test_suite
        let test_output = config.test_mutant_project();

        match build_output.status.code() {
            Some(0) => match test_output.status.code() {
                Some(0) => {
                    println!("Build was successful");
                    println!("Test suite passed");
                    m.set_status(MutationStatus::Survived);
                    survived.fetch_add(1, Ordering::SeqCst);
                    pending.fetch_sub(1, Ordering::SeqCst);
                }
                Some(_) => {
                    println!("Test suite failed");
                    let stderr = String::from_utf8_lossy(&test_output.stderr);
                    println!("stderr: {}", stderr);
                    if config.is_test_failed(&stderr) {
                        destroyed.fetch_add(1, Ordering::SeqCst);
                        pending.fetch_sub(1, Ordering::SeqCst);
                        m.set_status(MutationStatus::Killed);
                    }
                }
                None => {
                    eprintln!("Test suite was killed by a signal or crashed");
                    process::exit(1);
                }
            },
            Some(_) => {
                destroyed.fetch_add(1, Ordering::SeqCst);
                pending.fetch_sub(1, Ordering::SeqCst);
                m.set_status(MutationStatus::Killed);
            }
            None => {
                println!("Build was killed by a signal or crashed");
                process::exit(1);
            }
        }

        // Change back to the original directory at the end
        if let Err(e) = std::env::set_current_dir(&original_dir) {
            eprintln!("Failed to change back to the original directory: {}", e);
        }

        // @note the /temp dir and its contents will be deleted automatically,
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
    // returns & the Defer object from the top of the function is dropped.
    println!("{}", "Cleaning up temp files".cyan());
    print_table(args.output_path, summary_table).unwrap();
}
