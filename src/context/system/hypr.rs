//! Hyprland specific information
//!
//! This module contains the Hyprland specific information such as the
//! keybinds.

use serde::Deserialize;

use crate::utils::exec::run;

/// Store the parsed keybinds
///
/// This struct contains the keybinds for Hyprland.
#[derive(Debug)]
pub struct HyprKeybinds {
    pub modmask: String,
    pub submap: String,
    pub key: String,
    pub description: String,
}

/// Store the raw keybinds
///
/// This struct contains the raw keybinds from hyprctl.
#[derive(Debug, Deserialize)]
struct RawKeyBinds {
    modmask: u32,
    submap: String,
    key: String,
    description: String,
}

/// Initialize the keybinds from hyprctl
impl HyprKeybinds {
    /// Load and parse the keybinds
    ///
    /// This function will load the keybinds from hyprctl and parse them
    /// into a vector of `HyprKeybinds`.
    ///
    /// # Errors
    /// Returns an error if
    /// * hyprctl fails
    /// * output can not be parsed
    /// * hyprctl output is empty
    pub fn load_and_parse() -> anyhow::Result<Vec<Self>> {
        let (success, output) = run("hyprctl", ["binds", "-j"])?;
        if !success {
            anyhow::bail!("hyprctl failed: {}", output);
        }

        let raw_binds: Vec<RawKeyBinds> = serde_json::from_slice(output.as_bytes())?;
        let final_binds = raw_binds
            .into_iter()
            .map(|raw| HyprKeybinds {
                modmask: Self::parse_modmask(raw.modmask),
                submap: raw.submap,
                key: raw.key,
                description: raw.description,
            })
            .collect();

        Ok(final_binds)
    }

    /// Parse the modmask
    ///
    /// This function will parse the modmask and return a string
    /// containing the modifiers.
    ///
    /// # Arguments
    /// * `mask` - Modmask to parse
    ///
    /// # Returns
    /// Returns a string containing the modifiers or an empty string
    pub fn parse_modmask(mask: u32) -> String {
        let mut modifiers = Vec::new();

        if mask & 1 != 0 {
            modifiers.push("SHIFT");
        }

        if mask & 4 != 0 {
            modifiers.push("CTRL");
        }

        if mask & 8 != 0 {
            modifiers.push("ALT");
        }

        if mask & 64 != 0 {
            modifiers.push("SUPER");
        }

        if modifiers.is_empty() {
            "None".to_string()
        } else {
            modifiers.join(" + ")
        }
    }
}
