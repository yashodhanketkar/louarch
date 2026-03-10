//! This is the main library crate for louarch
pub mod context;
pub mod tools;
pub mod utils;

use crate::context::Context;
use tools::{audio, browser, completions, network, osmode, wallpaper};
use utils::cli::{Cli, Command};

/// Entry point for the application
///
/// Accepts following commands:
/// * wallpaper
/// * osmode
///
/// # Arguments
/// * `cli` - Command line arguments
///
/// # Errors
/// Returns an error if command fails
pub fn run(cli: Cli) -> anyhow::Result<()> {
    let ctx = Context::new()?;

    match cli.command {
        Command::Wallpaper { action } => wallpaper::handler(&ctx, action)?,
        Command::Osmode { action } => osmode::handler(&ctx, action)?,
        Command::Audio { action } => audio::handler(&ctx, action)?,
        Command::Browser { action } => browser::handler(&ctx, action)?,
        Command::Network { action } => network::handler(&ctx, action)?,
        Command::Completions { action, silent } => completions::handler(action, silent)?,
    }

    Ok(())
}
