//! Internal completions helper
//!
//! This module contains completion generator and completion manager.
use std::{fs, io::Write};

use crate::utils::cli::Cli;

use anyhow::Context;
use clap::CommandFactory;
use clap_complete::{Shell, generate};

/// Generate actual completions script
///
/// # Arguments
/// * `shell` - Shell to generate completions for
///
/// # Errors
/// Returns an error if output is invalid.
pub(crate) fn generator(shell: Shell) -> anyhow::Result<String> {
    let mut cmd = Cli::command();
    let mut buffer: Vec<u8> = Vec::new();
    let name = cmd.get_name().to_string();

    generate(shell, &mut cmd, name, &mut buffer);

    let generated = String::from_utf8(buffer)?;
    Ok(generated)
}

/// This function writes the completions script to disk.
///
/// # Arguments
/// * `path` - Path to write completions to
/// * `content` - Content of the completions script
///
/// # Errors
/// Returns an error if the file cannot be written to disk.
pub(crate) fn atomic_writer(path: &str, content: &str) -> anyhow::Result<()> {
    let tmp_path = format!("{}.tmp", path);
    let mut file =
        fs::File::create(&tmp_path).with_context(|| format!("Failed to create {}", tmp_path))?;

    file.write_all(content.as_bytes())
        .context("Failed to write to temp file")?;

    file.sync_all().context("Failed to sync to disk")?;

    fs::rename(&tmp_path, path)
        .with_context(|| format!("Failed to rename {} to {}", tmp_path, path))?;

    Ok(())
}

/// Ensure completions directory exists
///
/// # Arguments
/// * `path` - Path to completions directory
///
/// # Errors
/// Returns an error if the directory cannot be created.
pub(crate) fn ensure_completions_dir(path: &str) -> anyhow::Result<String> {
    fs::create_dir_all(path)
        .with_context(|| format!("Failed to create completions directory at {}", path))?;

    Ok(path.to_string())
}
