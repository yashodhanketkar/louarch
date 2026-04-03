//! This is the main library crate for louarch
pub mod context;
pub mod tools;
pub mod utils;

use context::Context;
use tools::{audio, browser, completions, config, network, osmode, tmux, wallpaper};
use utils::cli::{Cli, Command};

/// Entry point for the application
///
/// Accepts following commands:
/// * wallpaper
/// * osmode
/// * audio
/// * browser
/// * network
/// * tmux
/// * completions
///
/// # Arguments
/// * `cli` - Command line arguments
///
/// # Errors
/// Returns an error if command fails
pub fn run(cli: Cli) -> anyhow::Result<()> {
    let ctx = Context::new()?;

    match cli.command {
        // Sub-action commands
        Command::Wallpaper { action } => wallpaper::handler(&ctx, action)?,
        Command::Osmode { action } => osmode::handler(&ctx, action)?,
        Command::Audio { action } => audio::handler(&ctx, action)?,
        Command::Browser { action } => browser::handler(&ctx, action)?,
        Command::Wifi { action } => network::wifi_handler(&ctx, action)?,
        Command::Bluetooth { action } => network::bluetooth_handler(&ctx, action)?,
        Command::Tmux { action } => tmux::handler(&ctx, action)?,
        Command::Config { action } => config::handler(&ctx, action)?,

        // Sub-action commands with flags
        Command::Completions { action, silent } => completions::handler(action, silent)?,
    }

    Ok(())
}
