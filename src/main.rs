// #![allow(unused)]
use anyhow::Result;
pub mod config;
pub mod handlers;
pub mod parallel;
pub mod token;
pub mod utils;
pub mod reporter;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(cli::run_cli().await?)
}
