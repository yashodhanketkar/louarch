//! Application configuration management.
//!
//! Provides functionality to view the currently loaded state and
//! open the configuration file in the system editor.
use crate::context::Context;
use crate::utils::cli::Config;
use crate::utils::exec::cmd_run;

/// Handle the config action
///
/// This function dispatches the correct action based on input from user
///
/// # Arguments
/// * `ctx` - Context containing the configuration
/// * `action` - Action to be performed
///
/// # Actions
/// * `Config::View` view the current configuration
/// * `Config::Edit` edit the configuration file
///
/// # Errors
/// Returns an error
/// * unsupported action is requested
/// * supported action fails
pub fn handler(ctx: &Context, action: Config) -> anyhow::Result<()> {
    match action {
        Config::View => view(ctx),
        Config::Edit => edit(ctx),
    }
}

/// View configuration
///
/// Prints the current application configuration to the console.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Errors
/// Returns an error
/// * failed to run the editor
fn view(ctx: &Context) -> anyhow::Result<()> {
    println!("Current Application configuration\n");
    println!("{:#?}", ctx.app);
    Ok(())
}

/// Edit configuration
///
/// Opens the configuration file in the editor.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Errors
/// Returns an error
/// * failed to run the editor
fn edit(ctx: &Context) -> anyhow::Result<()> {
    shellexpand::full("~/.config/louarch/config.json")
        .map_err(anyhow::Error::msg)
        .and_then(|p| cmd_run(&ctx.app.editor, [p.as_ref()]))?;
    Ok(())
}
