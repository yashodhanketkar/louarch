//! Logic loading application configuration
//!
//! This module loads the application configuration from a JSON file.
//! The configuration file is located in the `$HOME/.config/louarch`
//! directory.
use std::fs;
use std::path::PathBuf;

use anyhow::Ok;
use rusqlite::Connection;
use serde::Deserialize;

/// Store for the application configuration
///
/// This struct contains the configuration for the application.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    /// Browser to use
    pub browser: String,

    /// Search engine to use
    pub search_engine: String,

    /// bookmarks file path
    #[serde(deserialize_with = "deserialize_path")]
    pub bookmarks_file: PathBuf,

    /// hyprpaper configuration path
    #[serde(deserialize_with = "deserialize_path")]
    pub hyprpaper_path: PathBuf,

    /// wallpapers directory path
    #[serde(deserialize_with = "deserialize_path")]
    pub wallpaper_dir: PathBuf,

    /// database path
    #[serde(deserialize_with = "deserialize_path")]
    pub db_path: PathBuf,
}

impl AppConfig {
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
        let config_file = fs::read_to_string(config_path.as_ref())?;
        let config: AppConfig = serde_json::from_str(&config_file)?;
        Ok(config)
    }

    /// Load the database connection
    ///
    /// # Arguments
    /// * `self` - Application configuration
    ///
    /// # Errors
    /// Returns an error if
    /// * the database cannot be opened
    /// * initialization fails
    pub fn open_db(&self) -> anyhow::Result<Connection> {
        let conn = Connection::open(&self.db_path)?;
        init_db(&conn)?;
        Ok(conn)
    }
}

/// Deserialize a path
///
/// This function deserializes a path from a JSON string. The path is
/// resolved to an absolute path using the `resolve_path` function. If the
/// path is relative, it is expanded relative to the user's home directory.
///
/// # Arguments
/// * `deserializer` - Deserializer to deserialize the path from
///
/// # Errors
/// Returns an error if
/// * path can not be resolved
fn deserialize_path<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: String = Deserialize::deserialize(deserializer)?;
    resolve_path(&raw).map_err(serde::de::Error::custom)
}

/// Get the absolute path
///
/// This function queries the `shellexpand` crate to expand the relative path
/// to the actual path. If the path is relative, it is it is expanded relative
/// to the user's home directory.
///
/// # Arguments
/// * `input` - Path to expand
///
/// # Requirements
/// * shellexpand must be installed
///
/// # Errors
/// Returns an error if
/// * shellexpand is not installed
/// * path can not be expanded
fn resolve_path(input: &str) -> anyhow::Result<PathBuf> {
    let expanded = shellexpand::full(input)?;
    let mut path = PathBuf::from(expanded.as_ref());

    if path.is_relative() {
        let home = std::env::var("HOME")?;
        path = PathBuf::from(home).join(path);
    }

    Ok(path)
}

/// Initialize the database
///
/// This function creates the database tables if they do not exist.
///
/// # Arguments
/// * `conn` - Database connection
///
/// # Errors
/// Returns an error if table fails to be created
pub(crate) fn init_db(conn: &Connection) -> anyhow::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS bookmarks (
        id INTEGER PRIMARY KEY,
        url TEXT NOT NULL UNIQUE
        );",
        (),
    )?;

    Ok(())
}
