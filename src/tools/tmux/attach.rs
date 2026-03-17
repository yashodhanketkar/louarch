//! Handles tmux attach action.
use std::fs;

use crate::{
    context::Context,
    utils::exec::{cmd_run, rofi_prompt},
};

/// Represents a tmux session
#[derive(Debug, Clone)]
struct Session {
    name: String,
    path: String,
}

/// Handles the tmux attach action
///
/// This function will prompt the user to select a tmux session and
/// attach to it.
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
    let available_dirs = list(ctx)?;
    let selected = selector(available_dirs)?;
    let formatted = formatter(selected)?;
    start(ctx, &formatted)?;

    Ok(())
}

/// List available directories for tmux sessions
///
/// This function will list all first level directories configured in
/// the tmux_dirs field of the context.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Errors
/// Returns an error if
/// * no directories found
fn list(ctx: &Context) -> anyhow::Result<Vec<String>> {
    let mut available_dirs: Vec<String> = Vec::new();
    for dir in &ctx.app.tmux_dirs {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            if let Some(p) = path.to_str() {
                available_dirs.push(p.to_string());
            }
        }
    }

    Ok(available_dirs)
}

/// Select a base directory
///
/// This function will select a base directory from the list of
/// available to spawn/attach to a tmux session.
fn selector(options: Vec<String>) -> anyhow::Result<String> {
    let selected = rofi_prompt("Select", options.iter().map(|s| s.as_str()), true)?;
    match selected {
        Some(s) => Ok(s),
        None => anyhow::bail!("No option selected"),
    }
}

/// Formats the selected base directory
///
/// This function will format the selected base directory to a session
/// object. This object will be used to spawn/attach to a tmux session.
///
/// # Arguments
/// * `selected` - Selected base directory
///
/// # Errors
/// Returns an error if
/// * no base directory found
fn formatter(selected: String) -> anyhow::Result<Session> {
    let name = selected.split("/").last().unwrap().to_string();
    Ok(Session {
        name,
        path: selected,
    })
}

/// Start a tmux session
///
/// This function will start a tmux session if it doesn't exist or
/// attach to it if it does.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
/// * `session` - Session to start/attach to
///
/// # Errors
/// Returns an error if
/// * tmux fails
fn start(ctx: &Context, session: &Session) -> anyhow::Result<()> {
    if ctx.system.tmux_sessions.contains(&session.name) {
        cmd_run("tmux", ["attach", "-t", &session.name])?;
    } else {
        cmd_run(
            "tmux",
            ["new-session", "-s", &session.name, "-c", &session.path],
        )?;
    }

    Ok(())
}
