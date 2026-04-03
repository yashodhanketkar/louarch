//! Network device management.
//!
//! Provides controls for interacting with and switching between
//! WiFi and Bluetooth devices.
use crate::context::Context;
use crate::utils::cli::{Bluetooth, Wifi};

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
///
/// # Errors
/// Returns an error
/// * unsupported action is requested
/// * supported action fails
pub fn wifi_handler(_ctx: &Context, action: Wifi) -> anyhow::Result<()> {
    match action {
        Wifi::Connect => wifi::connect(),
        Wifi::Disconnect => wifi::disconnect(),
    }
}

/// Handle the network action
///
/// This function dispatches the correct action based on input from user
///
/// # Arguments
/// * `ctx` - Context containing the configuration
/// * `action` - Action to be performed
///
/// # Actions
/// * `Network::Bluetooth` Select a bluetooth device
///
/// # Errors
/// Returns an error
/// * unsupported action is requested
/// * supported action fails
pub fn bluetooth_handler(_ctx: &Context, action: Bluetooth) -> anyhow::Result<()> {
    match action {
        Bluetooth::Connect => bluetooth::connect(),
        Bluetooth::Disconnect => bluetooth::disconnect(),
    }
}
