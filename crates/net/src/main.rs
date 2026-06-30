use clap::Parser;

use melon_net::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    melon_core::tracing::default();
    let _ = Cli::parse().run().await?;
    Ok(())
}
