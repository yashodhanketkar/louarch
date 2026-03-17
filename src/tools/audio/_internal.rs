//! This module contains the internal logic for selecting audio devices
use crate::utils::{
    exec::{self, rofi_prompt},
    notify,
};

/// Select an audio device
///
/// This function will prompt the user to select an audio device and set
/// it as the default device.
///
/// # Arguments
/// * `ctx_value` - Value of the current default device
/// * `list_arg` - Argument to pass to pactl to list correct devices
/// * `set_arg` - Argument to pass to pactl to set the default device
/// * `prompt` - Prompt to display to the user
/// * `cancel_msg` - Message to display if user cancels selection
/// * `extra_filter` - Optional filter to apply to the list of devices
///
/// # Requirements
/// * pactl must be installed
///
/// # Errors
/// Returns an error if
/// * pactl fails
/// * user cancels selection
pub(crate) fn select_device(
    ctx_value: &str,
    list_arg: &str,
    set_arg: &str,
    prompt: &str,
    cancel_msg: &str,
    extra_filter: Option<&str>,
) -> anyhow::Result<()> {
    let available = get_available(list_arg, extra_filter)?;
    let output = rofi_prompt(prompt, available, true)?;

    let selected = match output {
        Some(s) => s,
        None => {
            let _ = notify::send(
                notify::NotifyType::Error,
                notify::NotifyColor::Red,
                cancel_msg,
            );
            anyhow::bail!(cancel_msg.to_string());
        }
    };

    if selected == ctx_value {
        notify::send(
            notify::NotifyType::Confused,
            notify::NotifyColor::Yellow,
            "Source is already selected",
        )?;

        return Ok(());
    }

    exec::run("pactl", [set_arg, selected.as_str()])?;

    notify::send(
        notify::NotifyType::Ok,
        notify::NotifyColor::Green,
        &format!("Selected {}", selected),
    )?;

    Ok(())
}

/// Lists the available audio devices (sinks or sources) depending on
/// the user provided context
///
/// # Arguments
/// * `list_arg` - Argument to pass to pactl to list correct devices
/// * `extra_filter` - Optional filter to apply to the list of devices
///
/// # Requirements
/// * pactl must be installed
///
/// # Errors
/// Returns an error if pactl fails
fn get_available(list_arg: &str, extra_filter: Option<&str>) -> anyhow::Result<Vec<String>> {
    let (success, output) = exec::run("pactl", ["list", list_arg])?;
    if !success {
        anyhow::bail!("pactl failed: {}", output);
    }

    let available_sources = output
        .lines()
        .filter(|line| line.contains("Name:"))
        .filter(|line| {
            if let Some(f) = extra_filter {
                line.contains(f)
            } else {
                true
            }
        })
        .filter_map(|line| line.split_whitespace().nth(1))
        .map(|source| source.to_string())
        .collect();

    Ok(available_sources)
}
