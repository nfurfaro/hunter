use anyhow::Result;
pub mod config;
pub mod file_manager;
pub mod filters;
pub mod handlers;
pub mod processor;
pub mod reporter;
pub mod token;
pub mod utils;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(cli::run_cli().await?)
}
