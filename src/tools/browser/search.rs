//! Handles search functionality
use crate::{
    context::Context,
    utils::exec::{rofi_prompt, run},
};

/// Search for a term
///
/// Searches for the term using the configured search engine and and opens the
/// search result in the browser
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Errors
/// Returns an error
/// * user cancels search
/// * browser fails
pub(crate) fn handle(ctx: &Context) -> anyhow::Result<()> {
    let output = rofi_prompt("Search: ", &[], false)?;
    if output.is_none() {
        anyhow::bail!("Search term not found. Cancelling search.");
    }

    let search_term = output.unwrap();

    let (success, _) = run(
        &ctx.config.browser,
        &[format!("{}{}", &ctx.config.search_engine, search_term)],
    )?;

    if !success {
        anyhow::bail!("Failed to open browser");
    };

    Ok(())
}
