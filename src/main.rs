// #![allow(unused)]
use anyhow::Result;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(cli::run_cli().await?)
}
