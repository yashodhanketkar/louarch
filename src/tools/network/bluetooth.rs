//! Handle bluetooth actions
use crate::utils::cli::BluetoothAction;
use crate::utils::exec::{self, rofi_prompt};
use crate::utils::notify::{
    NotifyColor::{Green, Red},
    NotifyType::{Info, Warn},
    send,
};

/// Represents a bluetooth device
#[derive(Debug, Clone)]
pub(crate) struct BluetoothDevice {
    name: String,
    mac: String,
}

/// Implements the bluetooth device methods
impl BluetoothDevice {
    pub(crate) fn new(name: String, mac: String) -> Self {
        Self { name, mac }
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn get_mac(&self) -> &str {
        &self.mac
    }
}

/// Handler for bluetooth sub-actions
///
/// This function dispatches the correct sub-action based on input from
/// user
pub(crate) fn handler(action: BluetoothAction) -> anyhow::Result<()> {
    match action {
        BluetoothAction::Connect => connect(),
        BluetoothAction::Disconnect => disconnect(),
    }
}

/// Disconnect from bluetooth device
///
/// This function will disconnect from the currently connected bluetooth
/// device. On disconnection, the user will be notified.
///
/// # Errors
/// Returns an error if
/// * bluetoothctl fails
/// * user cancels disconnection
fn disconnect() -> anyhow::Result<()> {
    let (success, _) = exec::run("bluetoothctl", ["disconnect"])?;
    match success {
        true => send(Warn, Red, "Disconnected bluetooth device")?,
        false => anyhow::bail!("Failed to disconnect from bluetooth device"),
    }
    Ok(())
}

/// Connect to bluetooth device
///
/// This function will connect to the currently selected bluetooth
/// device.
///
/// # Errors
/// Returns an error if
/// * bluetoothctl fails
/// * user cancels connection
fn connect() -> anyhow::Result<()> {
    let devices = list()?;
    let selected = select(&devices)?;
    apply(selected)?;
    Ok(())
}

/// Handles the bluetooth connection
///
/// This function will apply the selected bluetooth device.
/// After connecting to device, the user will be notified.
///
/// # Arguments
/// * `device` - The bluetooth device to apply
///
/// # Errors
/// Returns an error if
/// * bluetoothctl fails
/// * user cancels connection
fn apply(device: &BluetoothDevice) -> anyhow::Result<()> {
    let (success, _) = exec::run("bluetoothctl", ["connect", device.get_mac()])?;
    match success {
        true => send(Info, Green, &format!("Connected to {}", device.get_name()))?,
        false => anyhow::bail!("Failed to connect to bluetooth device"),
    }
    Ok(())
}

/// Select a bluetooth device
///
/// This function will select a bluetooth device from the list of
/// available devices.
///
/// # Arguments
/// * `devices` - List of available bluetooth devices
///
/// # Errors
/// Returns an error if
/// * user cancels selection
/// * no devices found
fn select(devices: &[BluetoothDevice]) -> anyhow::Result<&BluetoothDevice> {
    let device_refs: Vec<&str> = devices.iter().map(|s| s.get_name()).collect();
    let output = rofi_prompt("Select bluetooth device", device_refs, true)?;
    let name = output.ok_or_else(|| anyhow::anyhow!("Selection cancelled"))?;

    let device = devices
        .iter()
        .find(|d| d.get_name() == name)
        .ok_or_else(|| anyhow::anyhow!("Selected device not found"))?;

    Ok(device)
}

/// List available bluetooth devices
///
/// This function will list all available bluetooth devices.
///
/// # Errors
/// Returns an error if
/// * bluetoothctl fails
fn list() -> anyhow::Result<Vec<BluetoothDevice>> {
    let (success, output) = exec::run("bluetoothctl", ["devices"])?;
    if !success {
        anyhow::bail!("Failed to list bluetooth devices");
    }

    let devices = output
        .lines()
        .filter_map(|l| {
            let mut parts = l.splitn(3, ' ');

            let _ = parts.next()?;
            let mac = parts.next()?.to_string();
            let name = parts.next()?.trim().to_string();

            Some(BluetoothDevice::new(name, mac))
        })
        .collect();

    Ok(devices)
}
