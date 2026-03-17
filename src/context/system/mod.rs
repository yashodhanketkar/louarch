//! Gather system information
//!
//! This module gather system information and provides a unified
//! interface for the context struct.
mod probes;

/// Store for the system
///
/// This struct contains the information about the system state and
/// connected devices.
#[derive(Debug)]
pub struct Config {
    /// List of connected monitors
    pub monitors: Vec<String>,
    /// Game mode status
    pub game_mode_status: bool,
    /// Night (blue-light) mode status
    pub night_mode_status: bool,
    /// Current audio sink
    pub audio_sink: String,
    /// Current audio source
    pub audio_source: String,
    /// Current active tmux sessions
    pub tmux_sessions: Vec<String>,
}

impl Config {
    /// Load the system configuration
    ///
    /// This will load the system configuration from the system.
    ///
    /// # Errors
    /// Returns an error if
    /// * the monitors cannot be listed
    /// * game mode status cannot be checked
    /// * night mode status cannot be checked
    pub fn load() -> anyhow::Result<Self> {
        let audio_devices = probes::audio_default_devices()?;

        Ok(Self {
            monitors: probes::list_monitors()?,
            game_mode_status: probes::check_game_mode()?,
            night_mode_status: probes::check_night_mode()?,
            audio_sink: audio_devices.sink,
            audio_source: audio_devices.source,
            tmux_sessions: probes::tmux_sessions()?,
        })
    }
}
