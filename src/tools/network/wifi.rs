//! Handle wifi actions
use crate::utils::cli::WifiAction;
use crate::utils::exec::{self, rofi_prompt};
use crate::utils::notify::{
    NotifyColor::{Green, Red, Yellow},
    NotifyType::{Hint, Info, Warn},
    send,
};

/// Handler for wifi sub-actions
///
/// This function dispatches the correct sub-action based on input from user
pub(crate) fn handler(action: WifiAction) -> anyhow::Result<()> {
    match action {
        WifiAction::Connect => connect(),
        WifiAction::Disconnect => disconnect(),
    }
}

/// Disconnect from wifi network
///
/// This function will disconnect from the currently connected wifi network.
/// On disconnection, the user will be notified.
///
/// # Errors
/// Returns an error if
/// * nmcli fails
/// * user cancels disconnection
fn disconnect() -> anyhow::Result<()> {
    let (success, _) = exec::run("nmcli", ["device", "disconnect", "wlan0"])?;
    match success {
        true => send(Warn, Red, "Disconnected from WiFi network")?,
        false => anyhow::bail!("Failed to disconnect from WiFi network"),
    }
    Ok(())
}

/// Connect to wifi network
///
/// This function will connect to the currently selected wifi network.
///
/// # Errors
/// Returns an error if
/// * nmcli fails
/// * user cancels connection
fn connect() -> anyhow::Result<()> {
    let devices = list()?;
    let selected = select(&devices)?;
    apply(selected)?;
    Ok(())
}

/// Handles the wifi connection
///
/// This function will apply the selected wifi network.
/// After connecting to network, the user will be notified.
///
/// # Arguments
/// * `device` - The wifi network to apply
///
/// # Errors
/// Returns an error if
/// * nmcli fails
/// * user cancels connection
fn apply(device: String) -> anyhow::Result<()> {
    let (success, _) = exec::run("nmcli", ["connection", "up", &device])?;
    match success {
        true => send(Info, Green, &format!("Connected to {}", device))?,
        false => anyhow::bail!("Failed to connect to WiFi network"),
    }
    Ok(())
}

/// Select a wifi network
///
/// This function will select a wifi network from the list of available
/// networks.
///
/// # Arguments
/// * `devices` - List of available wifi networks
///
/// # Errors
/// Returns an error if
/// * user cancels selection
/// * no networks found
fn select(devices: &[String]) -> anyhow::Result<String> {
    // let device_refs: Vec<&str> = devices.iter().map(String::as_str).collect();
    // match rofi_prompt("Select wifi device", &device_refs, true)? {
    match rofi_prompt("Select wifi device", devices, true)? {
        Some(s) => Ok(s),
        None => anyhow::bail!("Selection cancelled"),
    }
}

/// List available wifi networks
/// This function will list all available wifi networks.
///
/// # Errors
/// Returns an error if
/// * nmcli fails
/// * no networks found
fn list() -> anyhow::Result<Vec<String>> {
    send(Hint, Yellow, "Scanning for WiFi networks")?;
    let (success, output) = exec::run(
        "nmcli",
        [
            "-t", "-f", "SSID", "device", "wifi", "list", "--rescan", "yes",
        ],
    )?;
    if !success {
        anyhow::bail!("Failed to get available wifi devices");
    }

    let mut ssids: Vec<String> = output
        .lines()
        .map(|l| l.to_string())
        .filter(|s| !s.is_empty() && s != "--")
        .collect();

    ssids.sort();
    ssids.dedup();

    if ssids.is_empty() {
        anyhow::bail!("No WiFi networks found");
    }

    Ok(ssids)
}
