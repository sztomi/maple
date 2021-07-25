use std::fs::{self, File};
use std::path::PathBuf;

use anyhow::Result;
use configparser;
use directories::ProjectDirs;
use log;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
  #[error("Config file doesn't exist or empty.")]
  NoConfig,
  #[error("Could not load config file: {ini_error:?}")]
  CouldNotLoadConfig { ini_error: String },
  #[error("No valid home location could be determined.")]
  NoValidHome,
  #[error("Could not write config file: {ini_error:?}")]
  CouldNotWriteConfig { ini_error: String },
}

pub fn get(section: &str, key: &str) -> Result<Option<String>, ConfigError> {
  if let Some(proj_dirs) = ProjectDirs::from("com", "Maple for Plex", "Maple") {
    let cfg_file = PathBuf::from(format!("{}/maplecfg.ini", proj_dirs.config_dir().display()));
    if !cfg_file.is_file() {
      if let Ok(_) = File::create(&cfg_file) {
        return Err(ConfigError::NoConfig);
      }
      else {
        log::error!("Could not create config file at {}", cfg_file.display());
        std::process::exit(-1);
      }
    }
    let mut config = configparser::ini::Ini::new();
    if let Err(e) = config.load(&cfg_file) {
      return Err(ConfigError::CouldNotLoadConfig {
        ini_error: e.to_string(),
      });
    }
    match config.get(&section, &key) {
      Some(val) => return Ok(Some(val.clone())),
      None => return Ok(None),
    }
  }
  Err(ConfigError::NoValidHome)
}

pub fn set(section: &str, key: &str, value: &str) -> Result<(), ConfigError> {
  if let Some(proj_dirs) = ProjectDirs::from("com", "Maple for Plex", "Maple") {
    let cfg_dir = PathBuf::from(proj_dirs.config_dir());
    if !cfg_dir.is_dir() {
      if let Err(e) = fs::create_dir_all(&cfg_dir) {
        log::error!(
          "Could not create config dir at {}. Error: {}",
          cfg_dir.display(),
          e
        );
        std::process::exit(-1);
      }
    }
    let cfg_file = PathBuf::from(format!("{}/maplecfg.ini", proj_dirs.config_dir().display()));
    if !cfg_file.is_file() {
      if let Err(creation_err) = File::create(&cfg_file) {
        log::error!(
          "Could not create config file at {}. Error: {}",
          cfg_file.display(),
          creation_err
        );
        std::process::exit(-1);
      }
    }
    log::trace!(
      "Writing {}.{} = {} to {}",
      section,
      key,
      value,
      cfg_file.display()
    );
    let mut config = configparser::ini::Ini::new();
    config.set(&section, &key, Some((&value).to_string()));
    if let Err(e) = config.write(cfg_file) {
      return Err(ConfigError::CouldNotWriteConfig {
        ini_error: e.to_string(),
      });
    }
    return Ok(());
  }
  Err(ConfigError::NoValidHome)
}
