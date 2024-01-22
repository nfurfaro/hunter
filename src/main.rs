use std::io::Result;
use std::process;
pub mod config;
pub mod file_manager;
pub mod filters;
pub mod handlers;
pub mod languages;
pub mod processor;
pub mod reporter;
pub mod token;
pub mod utils;

mod cli;

#[tokio::main]
async fn main() {
    let result: Result<()> = cli::run_cli().await;
    if let Err(e) = result {
        eprintln!("{}", e);
        process::exit(1);
    } else {
        process::exit(0);
    }
}
