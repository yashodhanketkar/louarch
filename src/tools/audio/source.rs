//! This module handles the audio input selection
use crate::{context::Context, tools::audio::_internal::select_device};

/// Select the audio source
///
/// This function will prompt the user to select an audio source and set
/// it as the default source.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Requirements
/// * pactl must be installed
///
/// # Errors
/// Returns an error if
/// * pactl fails
/// * user cancels selection
pub(crate) fn select(ctx: &Context) -> anyhow::Result<()> {
    select_device(
        &ctx.system.audio_source,
        "sources",
        "set-default-source",
        "Select input device: ",
        "Source selection cancelled.",
        Some("input"),
    )
}
