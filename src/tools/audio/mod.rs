//! This module contains logic for switching audio devices.
use crate::{context::Context, utils::cli::Audio};

mod _internal;
mod sink;
mod source;

/// Handle the audio action
///
/// This function dispatches the correct action based on input from user
///
/// # Arguments
/// * `ctx` - Context containing the configuration
/// * `action` - Audio action to perform
///
/// # Actions
/// * `Audio::Sink` opens switch sinks dialog
/// * `Audio::Source` opens switch sources dialog
///
/// # Errors
/// Returns an error
/// * unsupported action is requested
/// * supported action fails
pub fn handler(ctx: &Context, action: Audio) -> anyhow::Result<()> {
    match action {
        Audio::Sink => sink::select(ctx),
        Audio::Source => source::select(ctx),
    }
}
