use clap::Parser;
use anyhow::Result;

/// Mutate Noir code and run tests against each mutation.
#[derive(Parser)]
struct Cli {
    /// The path to the Noir source files directory, defaults to ./src
    source: std::path::PathBuf,
    /// The path to the test directory, defaults to ./tests
    tests: std::path::PathBuf,

    // Files matching this regex will be excluded from testing
    // exclude: // regex ?,
    // Path to file defining custom mutation rules
    // mutations: std::path::PathBuf,
    // The percentage of mutations to run
    // sample_ratio: uint,
    // The optional path to the file for writing output. By default, output will by written to stdout
    // output: std::path::PathBuf,
}


pub async fn run_cli() -> Result<()> {
    let _args = Cli::parse();

    // core::load_src_files(args.source); // use args.exclude
    // core::mutate(args.mutations);
    // core::run_tests(args.tests);       // use args.sample_ratio
    // core::report(args.output);

    Ok(())
}
