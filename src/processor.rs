use std::{
    collections::HashSet,
    fs,
    path::PathBuf,
    process,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use ctrlc;
use lazy_static::lazy_static;
use rayon::prelude::*;

use crate::{
    cli::Args,
    config::LanguageConfig,
    file_manager::mutate_temp_file,
    handlers::mutator::{calculate_mutation_score, Mutant, MutationStatus},
    languages::common::Language,
    reporter::{mutants_progress_bar, mutation_test_summary_table, print_table},
};

pub fn process_mutants(
    mutants: &mut Vec<Mutant>,
    args: Args,
    config: Box<dyn LanguageConfig + Send + Sync>,
) {
    // Handle the Ctrl+C interrupt signal
    ctrlc::set_handler(move || {
        let temp_dirs = TEMP_DIRS.lock().unwrap();

        // Delete all temporary directories
        for path in temp_dirs.iter() {
            let _ = fs::remove_dir_all(path);
        }

        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let original_dir = std::env::current_dir().unwrap();
    let total_mutants = mutants.len();
    let bar = mutants_progress_bar(total_mutants);

    // mutatant status counters
    let destroyed = Arc::new(AtomicUsize::new(0));
    let survived = Arc::new(AtomicUsize::new(0));
    let unbuildable = Arc::new(AtomicUsize::new(0));
    let pending = Arc::new(AtomicUsize::new(total_mutants));

    lazy_static! {
        static ref TEMP_DIRS: Mutex<HashSet<PathBuf>> = Mutex::new(HashSet::new());
    }

    lazy_static! {
        static ref LIB_FILE_MUTEX: Mutex<()> = Mutex::new(());
    }

    let temp_dir = config
        .setup_test_infrastructure()
        .expect("Failed to setup test infrastructure");

    // Add the paths of the temporary directories to the global variable
    TEMP_DIRS
        .lock()
        .unwrap()
        .insert(temp_dir.path().to_path_buf());

    let config = Arc::new(Mutex::new(config));

    mutants.par_iter_mut().for_each(|m| {
        let config = Arc::clone(&config);
        let config_guard = config.lock().unwrap();

        // Check if the source file exists
        if !m.path().exists() {
            eprint!("Source File does not exist. Shutting down...");
            std::process::exit(1);
        }

        let lib_mutex = match config_guard.language() {
            Language::Noir => Some(&LIB_FILE_MUTEX as &Mutex<()>),
        };

        let temp_file = config_guard.copy_src_file(&temp_dir, m, lib_mutex)
            .expect("Failed to copy src to temp file");

        mutate_temp_file(&temp_file, m);

        // set current dir to "./temp"
        if let Err(e) = std::env::set_current_dir(temp_dir.as_ref()) {
            eprintln!("Failed to change to the temporary directory: {}", e);
        }

        let build_output = config_guard.build_mutant_project();
        let test_output = config_guard.test_mutant_project();
        let build_status = build_output.status.code();
        let test_status = test_output.status.code();


        match build_status {
            Some(0) => {
                match test_status {
                    Some(0) => {
                        m.set_status(MutationStatus::Survived);
                        survived.fetch_add(1, Ordering::SeqCst);
                        pending.fetch_sub(1, Ordering::SeqCst);
                    }
                    Some(_) => {
                        destroyed.fetch_add(1, Ordering::SeqCst);
                        pending.fetch_sub(1, Ordering::SeqCst);
                        m.set_status(MutationStatus::Killed);
                    }
                    None => {
                        eprintln!("Test suite was killed by a signal or crashed");
                        process::exit(1);
                    }
                }
            }
            Some(_) => {
                unbuildable.fetch_add(1, Ordering::SeqCst);
                pending.fetch_sub(1, Ordering::SeqCst);
                m.set_status(MutationStatus::Killed);
            }
            None => {
                    eprintln!("Build was killed by a signal or crashed");
                    eprint!("To see what the problem might be, try running the build command manually.i.e: `nargo build`");
                process::exit(1);
            }
        }

        bar.inc(1);
        if let Err(e) = std::env::set_current_dir(&original_dir) {
            eprintln!("Failed to change back to the original directory: {}", e);
        }
    });

    bar.finish_with_message("All mutants processed!");

    let score = calculate_mutation_score(
        destroyed.load(Ordering::SeqCst) as f64,
        unbuildable.load(Ordering::SeqCst) as f64,
        total_mutants as f64,
    );
    let summary_table = mutation_test_summary_table(
        total_mutants as f64,
        pending.load(Ordering::SeqCst) as f64,
        unbuildable.load(Ordering::SeqCst) as f64,
        destroyed.load(Ordering::SeqCst) as f64,
        survived.load(Ordering::SeqCst) as f64,
        score,
    );

    print_table(args.output_path, summary_table).unwrap();
}
