//! Handles game mode state.
use crate::context::Context;
use crate::utils::exec::run;
use crate::utils::notify::{
    NotifyColor::{Green, Red},
    NotifyType::Hint,
    send,
};

/// Toggle the game mode
///
/// This function will toggle game mode on and off based on current
/// state. This state is retrieved from the application context.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Requirement
/// * must be inside a hyprland session
///
/// # Errors
/// Returns an error if hyprctl results in an error
pub(crate) fn toggle(ctx: &Context) -> anyhow::Result<()> {
    match ctx.system.game_mode_status {
        true => off()?,
        false => on()?,
    }

    Ok(())
}

/// Turn on game mode
///
/// Helper function to turn on game mode
/// This function will run hyprctl commands to turn disabled animations
/// and decorations to improve performance.
///
/// Sends a notification to the user.
///
/// # Errors
/// Returns an error if hyprctl results in an error
fn on() -> anyhow::Result<()> {
    let commands = r#"
        hl.config({
            animations = { enabled = false, },
            animation = { "borderangle, 0", },
            decoration = {
                shadow = { enabled = false },
                blur = { enabled = false },
                fullscreen_opacity = 1.0,
                rounding = 0,
            },
            general = { gaps_in = 0, gaps_out = 0, border_size = 0, }
        })
    "#;

    run("hyprctl", ["--quiet", "eval", commands])?;
    send(Hint, Green, "Turned on game mode")?;
    Ok(())
}

/// Turn off game mode
///
/// Helper function to turn off game mode
/// This function will run hyprctl commands reload default config.
///
/// Sends a notification to the user.
///
/// # Errors
/// Returns an error if hyprctl results in an error
fn off() -> anyhow::Result<()> {
    run("hyprctl", ["--quiet", "reload"])?;
    send(Hint, Red, "Turned off game mode")?;
    Ok(())
}
