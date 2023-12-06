use crate::cli::Args;
use crate::config::Config;
use crate::parallel::process_mutants;
use crate::reporter::{print_line_in_span, ScanResult};
use crate::token::MutationStatus;
use colored::*;
use prettytable::{Cell, Row, Table};
use std::{io::Result, path::Path};

fn mutants_table() -> Table {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Surviving Mutants").style_spec("Fmb")
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Source file:").style_spec("Fcb"),
        Cell::new("Line #:").style_spec("Fcb"),
        Cell::new("Original context:").style_spec("Fcb"),
        Cell::new("Mutation:").style_spec("Fmb"),
    ]));
    table
}

pub fn mutate(args: Args, config: Config, results: &mut ScanResult) -> Result<()> {
    let mutants = results.mutants();
    println!("{}", "Running tests...".green());
    process_mutants(mutants, config);

    if args.verbose {
        // Create a new table
        let mut table = mutants_table();

        for mutant in mutants.clone() {
            if mutant.status() == MutationStatus::Survived
                || mutant.status() == MutationStatus::Pending
            {
                let span = mutant.span();
                let span_usize = (span.0 as usize, span.1 as usize);
                print_line_in_span(
                    &mut table,
                    // @fix path here is non existant
                    Path::new(mutant.path()),
                    span_usize,
                    &mutant.token(),
                )
                .unwrap();
            }
        }
        table.printstd();
    }

    println!("{}", "Cleaning up temp files".cyan());

    let _current_dir = std::env::current_dir().unwrap();

    // @fix
    // Remove the ./temp directory
    // let temp_dir = Path::new("./temp");
    // if temp_dir.exists() {
    //     std::fs::remove_dir_all(&temp_dir).expect("Failed to remove ./temp directory");
    // }

    Ok(())
}
