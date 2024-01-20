use color_eyre::eyre::{Context, OptionExt, Result};

use crate::config::{Button, Config};
use std::{
    path::{Path, PathBuf},
    process::exit,
};

impl Config {
    fn get_dir() -> Result<xdg::BaseDirectories> {
        Ok(xdg::BaseDirectories::with_prefix("apm")?)
    }

    pub fn get_file_path(file_name: &str) -> Result<PathBuf> {
        if let Ok(dir) = Self::get_dir() {
            if let Some(path) = dir.find_config_file(file_name) {
                return Ok(path);
            }
        }
        let path = if cfg!(debug_assertions) {
            format!("{}/examples/{}", env!("CARGO_MANIFEST_DIR"), file_name)
        } else {
            let exe_path =
                std::env::current_exe().wrap_err("Unable to get path of current executable")?;
            let path = exe_path
                .parent()
                .ok_or_eyre("Unable to get path of parent directory of current executable")?;
            format!("{}/{}", path.display(), file_name)
        };

        Ok(Path::new(&path).to_path_buf())
    }

    fn read_from_path(path: impl AsRef<Path>) -> Result<Self> {
        let file = std::fs::read_to_string(path)?;
        let data = toml::from_str::<Self>(&file)?;

        Ok(data)
    }

    pub fn open() -> Result<Self> {
        let path = &(Self::get_file_path("config.toml")?);

        let context = || format!("Unable to get config file from {}", path.display());
        let data = Self::read_from_path(path).wrap_err_with(context)?;

        Ok(data)
    }

    pub fn get_styles_path() -> PathBuf {
        if let Ok(path) = Self::get_file_path("styles.css") {
            return path;
        }
        println!("Icon is missing. Exiting...");
        exit(1);
    }
}

impl Button {
    pub fn get_icon_path(self) -> PathBuf {
        if let Ok(path) = Config::get_file_path(&self.icon) {
            return path;
        }
        println!("Icon is missing. Exiting...");
        exit(1);
    }
}
