//! This module contains logic for selecting network devices.
//!
//! Currently, supports random wifi and bluetooth operations.
use crate::context::Context;
use crate::utils::cli::Network;

mod bluetooth;
mod wifi;

/// Handle the network action
///
/// This function dispatches the correct action based on input from user
///
/// # Arguments
/// * `ctx` - Context containing the configuration
/// * `action` - Action to be performed
///
/// # Actions
/// * `Network::WiFi` Select a WiFi device
/// * `Network::Bluetooth` Select a bluetooth device
///
/// # Errors
/// Returns an error
/// * unsupported action is requested
/// * supported action fails
pub fn handler(_ctx: &Context, action: Network) -> anyhow::Result<()> {
    match action {
        Network::Wifi { action } => wifi::handler(action),
        Network::Bluetooth { action } => bluetooth::handler(action),
    }
}
