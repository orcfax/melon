use clap::Parser;

use melon_app::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    melon_core::tracing::default();
    Cli::parse().run().await?;
    Ok(())
}
