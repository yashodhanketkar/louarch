//! Handles shell completions
//!
//! This module contains the shell completions generator. Currently, only
//! bash and zsh completions are supported.
//!
//! User will need to install the completions manually.

use clap_complete::Shell;

mod _internal;
mod bash;
mod zsh;

/// Generate shell completions
///
/// This function generates shell completions for the user. Currently, only
/// bash and zsh completions are supported.
///
/// # Arguments
/// * `action` - Shell to generate completions for
///
/// # Requirements
/// * clap must be installed
/// * clap_complete must be installed
///
/// # Errors
/// Returns an error if
/// * clap is not installed
/// * clap_complete is not installed
/// * shell is not supported
pub fn handler(action: Shell) -> anyhow::Result<()> {
    let path = _internal::ensure_completions_dir(&shellexpand::full("~/.cache/louarch")?)?;
    match action {
        Shell::Bash => bash::handle(path)?,
        Shell::Zsh => zsh::handle(path)?,
        _ => anyhow::bail!("Unsupported shell"),
    };

    Ok(())
}
