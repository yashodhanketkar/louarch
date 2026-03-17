//! Handles night mode state.
use std::{thread::sleep, time::Duration};

use crate::context::Context;
use crate::utils::exec::run;

/// Toggle the night mode
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// This function will toggle game mode on and off based on current
/// state. This state is retrieved from the application context.
///
/// This function uses sleep to prevent race condition.
///
/// # Requirements
/// * hyprsunset must be installed
/// * must be inside a hyprland session
///
/// # Errors
/// Returns an error if
/// * hyprctl fails to run
/// * hyprctl fails to parse output
/// * hyprsunset fails to run
pub(crate) fn toggle(ctx: &Context) -> anyhow::Result<()> {
    if ctx.system.night_mode_status {
        run("killall", ["hyprsunset"])?;
        return Ok(());
    }

    let (success, _) = run("hyprctl", ["dispatch", "exec", "hyprsunset"])?;

    if !success {
        anyhow::bail!("hyprsunset not found");
    }

    sleep(Duration::from_millis(100));
    run("hyprctl", ["hyprsunset", "temperature", "4500"])?;
    Ok(())
}
