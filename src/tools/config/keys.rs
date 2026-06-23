//! Handles hyprland keybinds printing
//!
//! This module provides functionality to print the keybinds for
//! Hyprland for quick lookup.
use crate::context::system::hypr::HyprKeybinds;
use crate::{context::Context, utils::exec::rofi_prompt};

/// Print the keybinds
///
/// This function will print the keybinds to the console.
/// The keybinds are displayed in a rofi prompt.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Errors
/// Returns an error
/// * failed to run rofi
pub(crate) fn print(ctx: &Context) -> anyhow::Result<()> {
    let keybinds = &ctx.system.hypr_keybinds;
    let keybinds_desc: Vec<String> = keybinds.iter().map(format_keybind).collect();

    rofi_prompt("Keybinds", keybinds_desc, true)?;
    Ok(())
}

/// Format the keybind
///
/// This function will format the keybind for display in the rofi
/// prompt.
///
/// # Arguments
/// * `k` - Keybind to format
///
/// # Returns
/// Returns a string containing the formatted keybind with submap if
/// present
fn format_keybind(k: &HyprKeybinds) -> String {
    if k.submap.is_empty() {
        format!("{}: {} + {}", k.description, k.modmask, k.key)
    } else {
        format!(
            "({}) {}: {} + {}",
            k.submap, k.description, k.modmask, k.key
        )
    }
}
