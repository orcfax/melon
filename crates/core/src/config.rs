use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::path::{Path, PathBuf};

use crate::{Manifest, node};

pub static DEFAULT_CONFIG_DIR: &str = "./configs";

/// Read any type T from a TOML file.
pub fn read<T>(path: impl AsRef<Path>) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let content = std::fs::read_to_string(path.as_ref())?;
    let config: T = toml::from_str(&content)?;
    Ok(config)
}

/// Writes any serializable type T to a TOML file.
pub fn write<T>(data: &T, path: impl AsRef<Path>) -> anyhow::Result<()>
where
    T: Serialize,
{
    let content = toml::to_string_pretty(data)?;
    std::fs::write(path.as_ref(), content)?;
    Ok(())
}

pub fn default_filename(index: usize) -> String {
    format!("node-{:02}.toml", index)
}

pub fn default_path(output_dir: &str, index: usize) -> PathBuf {
    Path::new(output_dir).join(default_filename(index))
}

/// This is roughly a default config.
/// Particular configurations of components will need
/// more or less than this.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Config {
    #[n(0)]
    pub secrets: node::Secrets,
    #[n(1)]
    pub manifest: Manifest,
    #[n(2)]
    pub addr: String,
    #[n(3)]
    pub bootstrap: Vec<(String, node::Id)>,
}
