//! System information probes
//!
//! This module interacts with system APIs to determine system state
//! and devices.
use crate::utils::exec::run;
use std::process::Command;

/// Store for the audio devices
///
/// Helper struct for storing the currently default audio devices
pub struct AudioDevices {
    pub sink: String,
    pub source: String,
}

/// Lists the names connected monitors
///
/// This function queries the `hyprctl` API to gather information of
/// available monitors. The output is parsed into a vector of strings
/// containing the names of the monitors.
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
pub(crate) fn list_monitors() -> anyhow::Result<Vec<String>> {
    let output = Command::new("hyprctl").args(["monitors", "-j"]).output()?;

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
/// In this context, "Game Mode" is defined as the animations state
/// being disabled via `hyprctl`. State is determined by querying
/// `hyprctl` for the animations state.
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
pub(crate) fn check_game_mode() -> anyhow::Result<bool> {
    let output = Command::new("hyprctl")
        .args(["getoption", "animations:enabled"])
        .output()?;
    let status = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(!status.contains("int: 1"))
}

/// Check if the Night mode is enabled
///
/// In this context, "Night Mode" is defined blue light reduction is
/// turned on. This is determined by querying if hyprsunset process is
/// running.
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
pub(crate) fn check_night_mode() -> anyhow::Result<bool> {
    Ok(run("pidof", ["hyprsunset"])?.0)
}

/// Get the default audio devices
///
/// This function will query the `pactl` API to determine the default
/// audio devices. The output is parsed into a struct containing the
/// names of the default audio devices.
///
/// # Requirements
/// * pactl must be installed
/// * must be inside a hyprland session
///
/// # Errors
/// Returns an error if
/// * pactl is not installed
/// * `pactl` fails to run
/// * output can not be parsed
pub(crate) fn audio_default_devices() -> anyhow::Result<AudioDevices> {
    let get_pactl = |arg: &str| -> anyhow::Result<String> {
        let (success, output) = run("pactl", [arg])?;
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

/// Get the current running tmux sessions
///
/// This function will query the `tmux` API to determine the current
/// running tmux sessions. The output is parsed into a vector of strings
/// containing the names of the sessions.
///
/// # Requirements
/// * tmux must be installed
///
/// # Errors
/// Returns an error if
/// * tmux is not installed
/// * `tmux` fails to run
/// * output can not be parsed
pub(crate) fn tmux_sessions() -> anyhow::Result<Vec<String>> {
    let (success, output) = run("tmux", ["list-sessions", "-F", "#S"])?;
    if !success || output.is_empty() {
        return Ok(Vec::new());
    }
    let sessions = output.split("\n").map(|s| s.to_string()).collect();
    Ok(sessions)
}
