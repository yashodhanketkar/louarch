//! Handles tmux kill action.
use crate::{
    context::Context,
    utils::exec::{cmd_run, rofi_prompt},
};

/// Handles the tmux kill action
///
/// This function will prompt the user to select a tmux session and kill it.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Errors
/// Returns an error if
/// * user cancels selection
/// * no sessions found
/// * tmux fails
pub(crate) fn handle(ctx: &Context) -> anyhow::Result<()> {
    let selected = selector(ctx)?;
    killer(&selected)?;

    Ok(())
}

/// Select a tmux session
///
/// This function will select a tmux session from the list of available
/// sessions.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Errors
/// Returns an error if
/// * user cancels selection
/// * no sessions found
fn selector(ctx: &Context) -> anyhow::Result<String> {
    let selected = rofi_prompt("Select", &ctx.system.tmux_sessions, true)?;
    if let Some(s) = selected {
        Ok(s)
    } else {
        anyhow::bail!("No option selected");
    }
}

/// Kill a tmux session
///
/// This function will kill a tmux session.
///
/// # Arguments
/// * `name` - Name of the tmux session to kill
///
/// # Errors
/// Returns an error if
/// * tmux fails
fn killer(name: &str) -> anyhow::Result<()> {
    cmd_run("tmux", ["kill-session", "-t", name])?;
    Ok(())
}
