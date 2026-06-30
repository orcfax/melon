use melon_core::{Testnet, cli};

#[derive(Debug, Clone, clap::Args)]
pub struct NetGenerate {
    #[clap(flatten)]
    core: cli::Generate,
}

impl NetGenerate {
    pub fn run(&self) -> anyhow::Result<()> {
        Testnet::generate(self.core.total, self.core.base_port).write(&self.core.output_dir)
    }
}
