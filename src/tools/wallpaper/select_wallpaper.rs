//! Handles select wallpaper action
use crate::context::Context;
use crate::tools::wallpaper::{
    _internal::{list_images, pathbuf_to_string},
    apply::orchestrator,
};
use crate::utils::exec::rofi_prompt;

/// Apply a selcted wallpaper
///
/// This function will apply selected wallpaper/s to respective monitors.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Errors
/// Returns an error if
/// * the wallpapers are not found
/// * `_internal::apply` fails
pub(crate) fn switch(ctx: &Context) -> anyhow::Result<()> {
    if ctx.system.monitors.is_empty() {
        anyhow::bail!("failed to load monitors info");
    }

    let selected = display(ctx)?;

    orchestrator(ctx, &selected)?;
    Ok(())
}

/// Display the wallpapers options
///
/// Uses choose_wallpaper to display the wallpapers options to the user.
/// Selected wallpapers are returned to for further processing.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Errors
/// Returns an error if
/// * the wallpapers are not found
/// * rofi prompt fails
fn display(ctx: &Context) -> anyhow::Result<Vec<String>> {
    let wps = pathbuf_to_string(list_images(&ctx.config.wallpaper_dir)?);
    let opts = wps.iter().map(|s| s.as_str()).collect();
    let selected = choose_wallpaper(ctx, opts)?;
    Ok(selected)
}

/// Shows the rofi prompt to the user
///
/// This function will show the rofi prompt to the user. The prompt will
/// display the available wallpapers and the user will be prompted to
/// select one or more wallpapers. The selected wallpapers are returned
/// to the caller.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
/// * `opts` - Vector of strings containing the names of the wallpapers
///
/// # Errors
/// Returns an error if rofi prompt fails
fn choose_wallpaper(ctx: &Context, opts: Vec<&str>) -> anyhow::Result<Vec<String>> {
    let mut selected: Vec<String> = vec![];
    let monitors = &ctx.system.monitors.clone();

    for monitor in monitors {
        match rofi_prompt(&format!("Select for {}", monitor), &opts, true)? {
            Some(c) => selected.push(c),
            None if selected.is_empty() => anyhow::bail!("no wallpaper selected"),
            None => break,
        }

        if selected.len() == monitors.len() {
            break;
        }

        match rofi_prompt("Selcted more", ["No", "Yes"], true)? {
            Some(more) if more == "Yes" => continue,
            _ => break,
        }
    }

    Ok(selected)
}
