//! Path deserializers
//!
//! Deserializes paths from JSON strings. These paths are resolved and
//! used in application configuration
use std::path::PathBuf;

use serde::Deserialize;

/// Deserialize a path
///
/// This function deserializes a path from a JSON string. The path is
/// resolved to an absolute path using the `resolve_path` function. If
/// the path is relative, it is expanded relative to the user's home
/// directory.
///
/// # Arguments
/// * `deserializer` - Deserializer to deserialize the path from
///
/// # Errors
/// Returns an error if
/// * path can not be resolved
pub(crate) fn deserialize_path<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: String = Deserialize::deserialize(deserializer)?;
    resolve_path(&raw).map_err(serde::de::Error::custom)
}

/// Deserialize a path vector
///
/// Simialr to `deserialize_path` but for a vector of paths.
pub(crate) fn deserialize_vec_path<'de, D>(deserializer: D) -> Result<Vec<PathBuf>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raws: Vec<String> = Deserialize::deserialize(deserializer)?;

    raws.into_iter()
        .map(|raw| resolve_path(&raw).map_err(serde::de::Error::custom))
        .collect()
}

/// Get the absolute path
///
/// This function queries the `shellexpand` crate to expand the relative
/// path to the actual path. If the path is relative, it is expanded
/// relative to the user's home directory.
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
