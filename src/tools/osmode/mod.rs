//! OS mode management.
//!
//! Provides toggles for system-level configurations such as
//! game mode and night light settings.
mod game;
mod night;

use crate::context::Context;
use crate::utils::cli::Osmode;

/// Handle the OS mode action
///
/// This function dispatches the correct action based on input from user
///
/// # Arguments
/// * `ctx` - Context containing the configuration
/// * `action` - OS mode action to perform
///
/// # Actions
/// * `Osmode::Game` toggle game mode
/// * `Osmode::Night` toggle night mode
///
/// # Errors
/// Returns an error
/// * unsupported action is requested
/// * supported action fails
pub fn handler(ctx: &Context, action: Osmode) -> anyhow::Result<()> {
    match action {
        Osmode::Game => game::toggle(ctx),
        Osmode::Night => night::toggle(ctx),
    }
}
