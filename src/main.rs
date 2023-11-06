// #![allow(unused)]
use anyhow::Result;
pub mod filter;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(cli::run_cli().await?)
}
