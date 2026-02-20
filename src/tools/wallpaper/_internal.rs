//! Handles loading images and formatting paths.
use crate::context::Context;
use std::fs;
use std::path::{Path, PathBuf};

/// List all the images in the wallpapers directory
///
/// # Arguments
/// * `dir` - Path to the wallpapers directory
///
/// This function will return a vector of paths to the images in the
/// wallpapers directory.
///
/// # Errors
/// Returns an error if the wallpapers directory cannot be found
pub(crate) fn list_images(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}

/// Return a vector of strings containing the paths to the images
///
/// # Arguments
/// * `ctx` - Context containing the configuration
/// * `imgs` - Vector of strings containing the names of the images
///
/// This function will return a vector of strings containing the paths
/// to the images in the wallpapers directory. The paths are formatted
/// to include the wallpapers directory.
///
/// # Errors
/// Returns an error if the wallpapers directory cannot be found
pub(crate) fn format_image(ctx: &Context, imgs: &Vec<String>) -> anyhow::Result<Vec<String>> {
    let mut paths = vec![];

    for img in imgs {
        paths.push(format!(
            "{}{}",
            &ctx.config.wallpaper_dir.to_string_lossy(),
            img
        ));
    }

    Ok(paths)
}

/// Get the names of the wallpapers
///
/// # Arguments
/// * `paths` - Vector of paths to the wallpapers
///
/// This function will return a vector of strings containing the names of the
/// wallpapers in the wallpapers directory.
pub(crate) fn pathbuf_to_string(paths: Vec<PathBuf>) -> Vec<String> {
    paths
        .into_iter()
        .filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(String::from))
        .collect()
}
