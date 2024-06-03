use dirs;
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize)]
pub struct MpdConfig {
  pub music_dir: String,
  pub ip: String,
  pub port: String,
  pub use_cover: bool,
}

impl MpdConfig {
  pub fn from_file() -> Result<Self, Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    let config_contents = fs::read_to_string(&config_path)?;
    let config: MpdConfig = toml::from_str(&config_contents)?;

    Ok(config)
  }
}

fn get_config_path() -> Result<PathBuf, std::io::Error> {
  let home_dir = dirs::home_dir().ok_or(std::io::Error::new(
    std::io::ErrorKind::NotFound,
    "Home directory not found",
  ))?;
  let config_path = home_dir.join(".config/mpd-notification/config.toml");

  Ok(config_path)
}

