use std::fs::{self, File};
use std::path::PathBuf;

use anyhow::{bail, Result};
use configparser;
use directories::ProjectDirs;
use log;
use thiserror::Error;

// TODO(sztomi): read this from the environment
const CONFIG_FILE_NAME: &str = "mapleconfig.ini";

#[derive(Error, Debug)]
pub enum ConfigError {
  #[error("Could not load config file: {ini_error:?}")]
  CouldNotLoadConfig { ini_error: String },
  #[error("No valid home location could be determined.")]
  NoValidHome,
  #[error("Could not write config file: {ini_error:?}")]
  CouldNotWriteConfig { ini_error: String },
}

fn get_config_dir() -> Option<ProjectDirs> {
  ProjectDirs::from("com", "Maple for Plex", "Maple")
}

pub fn get_config_file() -> Result<PathBuf> {
  if let Some(proj_dirs) = get_config_dir() {
    return Ok(PathBuf::from(format!(
      "{}/{}",
      proj_dirs.config_dir().display(),
      CONFIG_FILE_NAME
    )));
  }
  bail!(ConfigError::NoValidHome)
}

pub fn ensure_config_file() -> Result<PathBuf, ConfigError> {
  if let Some(proj_dirs) = get_config_dir() {
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
    let cfg_file = PathBuf::from(format!(
      "{}/{}",
      proj_dirs.config_dir().display(),
      CONFIG_FILE_NAME
    ));
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
    return Ok(cfg_file);
  }
  Err(ConfigError::NoValidHome)
}

pub fn get(section: &str, key: &str) -> Result<Option<String>, ConfigError> {
  let cfg_file = ensure_config_file()?;
  let mut config = configparser::ini::Ini::new();
  if let Err(e) = config.load(&cfg_file) {
    return Err(ConfigError::CouldNotLoadConfig {
      ini_error: e,
    });
  }
  match config.get(section, key) {
    Some(val) => Ok(Some(val)),
    None => Ok(None),
  }
}

pub fn set(section: &str, key: &str, value: &str) -> Result<(), ConfigError> {
  let cfg_file = ensure_config_file()?;
  log::trace!(
    "Writing {}.{} = {} to {}",
    section,
    key,
    value,
    cfg_file.display()
  );
  let mut config = configparser::ini::Ini::new();
  config.set(section, key, Some((&value).to_string()));
  if let Err(e) = config.write(cfg_file) {
    return Err(ConfigError::CouldNotWriteConfig {
      ini_error: e.to_string(),
    });
  }
  Ok(())
}
