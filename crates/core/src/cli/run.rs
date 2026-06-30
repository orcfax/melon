use std::path::{Path, PathBuf};

use crate::config;

#[derive(Debug, Clone, clap::Args)]
pub struct CoreRun {
    /// Path to config file
    #[arg(long)]
    config: Option<String>,
    /// If using the default filepaths and names, simply specify the index.
    #[arg(long)]
    index: Option<usize>,
}

impl CoreRun {
    pub fn config(&self) -> anyhow::Result<PathBuf> {
        if let Some(path) = &self.config {
            Ok(Path::new(path).into())
        } else if let Some(index) = self.index {
            Ok(config::default_path(config::DEFAULT_CONFIG_DIR, index))
        } else {
            Err(anyhow::anyhow!("Either config or index must be specified"))
        }
    }
}
