//! This module contains logic for switching wallpapers.
//!
//! Currently, supports random wallpaper and selecting wallpapers.
use crate::context::Context;
use crate::utils::cli::Wallpaper;

mod _internal;
mod apply;
mod random_wallpaper;
mod select_wallpaper;

/// Handle the wallpaper action
///
/// This function dispatches the correct action based on input from user
///
/// # Arguments
/// * `ctx` - Context containing the configuration
/// * `action` - Action to be performed
///
/// # Actions
/// * `Wallpaper::Random` apply one random the wallpaper
/// * `Wallpaper::Select` apply the wallpaper/s selected by the user
///
/// # Errors
/// Returns an error
/// * unsupported action is requested
/// * supported action fails
pub fn handler(ctx: &Context, action: Wallpaper) -> anyhow::Result<()> {
    match action {
        Wallpaper::Random => random_wallpaper::switch(ctx),
        Wallpaper::Select => select_wallpaper::switch(ctx),
    }
}
