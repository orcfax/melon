use std::{fs, path::PathBuf};

use serde::Serialize;

use crate::{config, testnet::Testnet};

#[derive(Debug, Clone, clap::Args)]
pub struct CoreGenerate {
    /// Output directory of config files
    #[arg(long, default_value = config::DEFAULT_CONFIG_DIR)]
    pub output_dir: String,
    /// Total number of shares (nodes)
    #[arg(long, default_value_t = 3)]
    pub total: usize,
    /// Port from which testnet starts
    #[arg(long, default_value_t = 10000)]
    pub base_port: u16,
}

impl CoreGenerate {
    pub fn default_path(&self, index: usize) -> PathBuf {
        config::default_path(&self.output_dir, index)
    }

    pub fn create_dir(&self) -> anyhow::Result<()> {
        fs::create_dir_all(&self.output_dir)?;
        Ok(())
    }

    pub fn run(&self) -> anyhow::Result<()> {
        Testnet::generate(self.total, self.base_port).write(&self.output_dir)
    }

    pub fn write<C: Serialize>(&self, index: usize, c: &C) -> anyhow::Result<()> {
        config::write(c, self.default_path(index))
    }
}
