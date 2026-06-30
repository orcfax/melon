mod config;
mod run;
use config::Config;

use melon_net::cli::Generate;
use run::SetRunStandalone as Run;

#[derive(clap::Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Command {
    /// Generate a set of configurations for nodes
    Generate(Generate),
    /// Run the node with the loaded config
    Run(Run),
}

impl Cli {
    pub async fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::Generate(args) => args.run()?,
            Command::Run(args) => args.run().await?,
        };
        Ok(())
    }
}
