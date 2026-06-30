use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub socket: String,
    pub name: String,
}

impl Config {
    const DEFAULT_DIR: &str = "/tmp";

    pub fn from_index(index: usize) -> Self {
        Self::from_name(format!("node-{}", index))
    }

    pub fn from_name(name: String) -> Self {
        Self {
            socket: Path::new(Self::DEFAULT_DIR)
                .join(format!("{}.sock", name.clone()))
                .to_str()
                .unwrap()
                .to_owned(),
            name,
        }
    }

    pub fn from_config_path(fp: &str) -> Self {
        let basename = Path::new(fp)
            .file_stem()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("melon");
        Self::from_name(basename.to_string())
    }
}
