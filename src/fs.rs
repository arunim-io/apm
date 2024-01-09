use crate::config::{Button, Config};
use std::path::{Path, PathBuf};

impl Config {
    fn get_dir() -> xdg::BaseDirectories {
        return xdg::BaseDirectories::with_prefix("acw").unwrap();
    }
    fn read_from_path(path: impl AsRef<Path>) -> Self {
        let file = std::fs::read_to_string(path).expect("Unable to read config file");
        return toml::from_str::<Self>(&file).expect("Unable to parse config file");
    }
    pub fn get_file_path(file_name: &str) -> PathBuf {
        let config_dir = Self::get_dir();
        return config_dir.get_config_file(file_name);
    }
    pub fn open() -> Self {
        return Self::read_from_path(Self::get_file_path("config.toml"));
    }
    pub fn get_styles_path() -> PathBuf {
        return Self::get_file_path("styles.css");
    }
}
impl Button {
    pub fn get_icon_path(self) -> PathBuf {
        let path = Config::get_file_path(&self.icon);

        if !path.try_exists().unwrap() {
            return shellexpand::path::tilde(&self.icon).to_path_buf();
        }
        return path;
    }
}
