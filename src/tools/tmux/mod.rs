//! This module contains logic for tmux actions.
//!
//! Currently, supports attaching to a tmux session and killing a session.
//! actions.

mod attach;
mod kill;

use crate::context::Context;
use crate::utils::cli::Tmux;

/// Handle the tmux action
///
/// This function dispatches the correct action based on input from user
///
/// # Arguments
/// * `ctx` - Context containing the configuration
/// * `action` - Tmux action to perform
///
/// # Actions
/// * `Tmux::Attach` attach to a tmux session
/// * `Tmux::Kill` kill a tmux session
///
/// # Errors
/// Returns an error
/// * unsupported action is requested
/// * supported action fails
pub fn handler(ctx: &Context, action: Tmux) -> anyhow::Result<()> {
    match action {
        Tmux::Attach => attach::handle(ctx),
        Tmux::Kill => kill::handle(ctx),
    }
}
