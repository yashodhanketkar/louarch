//! Logic loading system information
//!
//! This module interacts with system APIs to determine system state and
//! devices.
use std::process::Command;

use crate::utils::exec::run;

/// Store for the audio devices
///
/// Helper struct for storing the currently default audio devices
pub struct AudioDevices {
    pub sink: String,
    pub source: String,
}

/// Store for the system
///
/// This struct contains the information about the system state and connected
/// devices.
#[derive(Debug)]
pub struct SystemConfig {
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
}

impl SystemConfig {
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
        let audio_devices = audio_default_devices()?;

        Ok(Self {
            monitors: list_monitors()?,
            game_mode_status: check_game_mode()?,
            night_mode_status: check_night_mode()?,
            audio_sink: audio_devices.sink,
            audio_source: audio_devices.source,
        })
    }
}

/// Lists the names connected monitors
///
/// This function queries the `hyprctl` API to gather information of available
/// monitors. The output is parsed into a vector of strings containing the
/// names of the monitors.
///
/// # Requirements
/// * hyprctl must be installed
/// * must be inside a hyprland session
///
/// # Errors
/// Returns an error if
/// * hyprctl is not installed
/// * `hyprctl` fails to run
/// * output can not be parsed
fn list_monitors() -> anyhow::Result<Vec<String>> {
    let output = Command::new("hyprctl").args(&["monitors", "-j"]).output()?;

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let names = json
        .as_array()
        .map(|l| {
            l.iter()
                .filter_map(|m| m["name"].as_str())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default();

    Ok(names)
}

/// Check if the Game mode is enabled
///
/// In this context, "Game Mode" is defined as the animations state being
/// disabled via `hyprctl`. State is determined by querying `hyprctl` for the
/// animations state.
///
/// # Requirements
/// * hyprctl must be installed
/// * must be inside a hyprland session
///
/// # Errors
/// Returns an error if
/// * hyprctl is not installed
/// * `hyprctl` fails to run
/// * output can not be parsed
fn check_game_mode() -> anyhow::Result<bool> {
    let output = Command::new("hyprctl")
        .args(&[&"getoption", &"animations:enabled"])
        .output()?;
    let status = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(!status.contains("int: 1"))
}

/// Check if the Night mode is enabled
///
/// In this context, "Night Mode" is defined blue light reduction is turned
/// on. This is determined by querying if hyprsunset process is running.
///
/// Currently, reduction levels are not monitored.
///
/// # Note
/// This function relies on `pidof` to detect process state.
///
/// # Requirements
/// * hyprsunset must be installed
/// * must be inside a hyprland session
///
/// # Errors
/// Returns an error if
/// * hyprsunset is not installed
/// * output can not be parsed
fn check_night_mode() -> anyhow::Result<bool> {
    Ok(run("pidof", &[&"hyprsunset"])?.0)
}

fn audio_default_devices() -> anyhow::Result<AudioDevices> {
    let get_pactl = |arg: &str| -> anyhow::Result<String> {
        let (success, output) = run("pactl", &[arg])?;
        if !success {
            anyhow::bail!("pactl failed: {}", output);
        }
        Ok(output)
    };

    Ok(AudioDevices {
        source: get_pactl("get-default-source")?,
        sink: get_pactl("get-default-sink")?,
    })
}
