//! Web browsing and searching.
//!
//! Provides an interface for searching queries or opening URLs
//! using the system's configured web browser.
use crate::{
    context::Context,
    utils::{cli::Browser, exec::is_installed},
};

mod _internal;
mod browse;
mod search;

/// Handles browser operations
///
/// This function dispatches the correct action based on input from user
///
/// # Arguments
/// * `ctx` - Context containing the configuration
/// * `action` - Audio action to perform
///
/// # Actions
/// * `Browser::Search` search for a query
/// * `Browser::Browse` browse a url
///
/// # Errors
/// Returns an error
/// * unsupported action is requested
/// * supported action fails
pub fn handler(ctx: &Context, action: Browser) -> anyhow::Result<()> {
    if !is_installed(&ctx.app.browser) {
        anyhow::bail!("Browser {} not found", &ctx.app.browser);
    }

    match action {
        Browser::Search => search::handle(ctx),
        Browser::Browse => browse::handle(ctx),
    }
}
