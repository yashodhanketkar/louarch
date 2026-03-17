//! This module handles the audio output selection
use crate::{context::Context, tools::audio::_internal::select_device};

/// Select the audio sink
///
/// This function will prompt the user to select an audio sink and set
/// it as the default sink.
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
        &ctx.system.audio_sink,
        "sinks",
        "set-default-sink",
        "Select output device: ",
        "Sink selection cancelled.",
        None,
    )
}
