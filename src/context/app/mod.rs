//! Gather system information
//!
//! This module gather application configuration and provides a unified
//! interface for the context struct.
//!
//! The configuration file is located in the `$HOME/.config/louarch`. In
//! absence of a configuration file, the default values are used.
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

mod defaults;
mod deserializers;

use defaults::*;
use deserializers::*;

/// Store for the application configuration
///
/// This struct contains the configuration for the application.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// Browser to use
    #[serde(default = "default_browser")]
    pub browser: String,

    /// Editor to use
    #[serde(default = "default_editor")]
    pub editor: String,

    /// Search engine to use
    #[serde(default = "default_search_engine")]
    pub search_engine: String,

    /// Hyprpaper configuration path
    #[serde(
        deserialize_with = "deserialize_path",
        default = "default_hyprpaper_path"
    )]
    pub hyprpaper_path: PathBuf,

    /// Wallpapers directory path
    #[serde(
        deserialize_with = "deserialize_path",
        default = " default_wallpaper_dir"
    )]
    pub wallpaper_dir: PathBuf,

    /// Database path
    #[serde(deserialize_with = "deserialize_path", default = "default_db_path")]
    pub db_path: PathBuf,

    /// Tmux configuration path
    #[serde(
        deserialize_with = "deserialize_vec_path",
        default = "default_tmux_dirs"
    )]
    pub tmux_dirs: Vec<PathBuf>,
}

impl Default for Config {
    /// Default values for the application configuration
    fn default() -> Self {
        Self {
            browser: default_browser(),
            editor: default_editor(),
            search_engine: default_search_engine(),
            hyprpaper_path: default_hyprpaper_path(),
            wallpaper_dir: default_wallpaper_dir(),
            db_path: default_db_path(),
            tmux_dirs: default_tmux_dirs(),
        }
    }
}

impl Config {
    /// Load the application configuration
    ///
    /// This will load the application configuration from the
    /// `$HOME/.config/louarch/config.json` directory.
    ///
    /// # Errors
    /// Returns an error if
    /// * the configuration file cannot be read
    /// * the configuration file is not valid JSON
    pub fn load() -> anyhow::Result<Self> {
        let config_path = shellexpand::full("~/.config/louarch/config.json")?;

        let config_file = match fs::read_to_string(config_path.as_ref()) {
            Ok(content) => content,
            // If the config file does not exist, return the default config
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(e) => return Err(e.into()),
        };

        let config: Config = serde_json::from_str(&config_file)?;
        Ok(config)
    }
}
