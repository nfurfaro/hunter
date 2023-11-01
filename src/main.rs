// #![allow(unused)]
use anyhow::Result;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    cli::run_cli().await
}
