use color_eyre::eyre::{Context, Result};

use crate::config::Config;
use std::path::{Path, PathBuf};

impl Config {
    fn get_dir() -> Result<xdg::BaseDirectories> {
        return Ok(xdg::BaseDirectories::with_prefix("apm")?);
    }

    pub fn get_file_path(file_name: &str) -> PathBuf {
        if let Ok(path) = Self::get_dir() {
            if let Some(file) = path.find_config_file(file_name) {
                return file;
            }
        }
        return Path::new(&format!("examples/{}", file_name)).to_path_buf();
    }

    fn read_from_path(path: impl AsRef<Path>) -> Result<Self> {
        let file = std::fs::read_to_string(path)?;
        return Ok(toml::from_str::<Self>(&file)?);
    }

    pub fn open() -> Result<Self> {
        let ref path = Self::get_file_path("config.toml");
        let context = || format!("Unable to get config file from {}", path.display());

        Ok(Self::read_from_path(path).with_context(context)?)
    }

    pub fn get_styles_path() -> PathBuf {
        return Self::get_file_path("styles.css");
    }
}
