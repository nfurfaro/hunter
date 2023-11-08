// #![allow(unused)]
use anyhow::Result;
pub mod mutant;
pub mod parallel;
pub mod utils;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(cli::run_cli().await?)
}
