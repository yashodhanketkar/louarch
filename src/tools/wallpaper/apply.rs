//! Handles application of hyprpaper configuration and theme.
use anyhow::Ok;

use crate::{
    context::Context,
    tools::wallpaper::_internal::format_image,
    utils::exec::{self, is_installed},
};
use std::fmt::Write;

/// Apply wallpapers to the monitors
///
/// This function will apply the wallpapers to the monitors. The wallpapers
/// are selected based on the user's input.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
/// * `selected` - Vector of strings containing the names of the wallpapers
///
/// # Errors
/// Returns an error if
/// * the wallpapers directory cannot be found
/// * any process fails such as `hyprpaper`, `wallust`, etc.
pub(crate) fn orchestrator(ctx: &Context, selected: &Vec<String>) -> anyhow::Result<()> {
    let imgs = format_image(&ctx, &selected)?;
    write_hyprpaper_config(&ctx, &imgs)?;
    pallete_generator(&imgs[0])?;
    apply_theme()?;
    Ok(())
}

/// Write the hyprpaper config file
///
/// # Arguments
/// * `ctx` - Context containing the configuration
/// * `imgs` - Vector of strings containing the names of the wallpapers
///
/// This function will write the hyprpaper config file to the user's
/// home directory. The config file will contain the wallpapers to be
/// applied to the monitors.
///
/// # Errors
/// Returns an error if writer fails
fn write_hyprpaper_config(ctx: &Context, imgs: &Vec<String>) -> anyhow::Result<()> {
    let mut content = String::from("ipc=true\nsplash=false\n\n");

    for (i, monitor) in ctx.system.monitors.iter().enumerate() {
        let wallpaper = &imgs[i % imgs.len()];
        writer(&mut content, &monitor, &wallpaper)?;
    }

    writer(&mut content, "", &imgs[0])?;

    let config_path = &ctx.config.hyprpaper_path;
    std::fs::write(config_path, content)?;

    Ok(())
}

/// Format the wallpaper config block
///
/// # Arguments
/// * `content` - String containing the wallpaper config block
/// * `monitor` - Name of the monitor
/// * `wallpaper` - Path to the wallpaper
///
/// This function will format the wallpaper config block. The block will
/// contain the monitor name and the wallpaper path.
///
/// # Errors
/// Returns an error if the config file cannot be written
fn writer(content: &mut String, monitor: &str, wallpaper: &str) -> anyhow::Result<()> {
    writeln!(
        content,
        "wallpaper {{\n  monitor = {}\n  path = {}\n}}\n",
        monitor, wallpaper
    )?;

    Ok(())
}

/// Generate the color pallete
///
/// This function will generate the color pallete for the wallpaper using
/// wallust.
///
/// # Arguments
/// * `main_wallpaper` - Path to the main/default wallpaper
///
/// # Requirements
/// * wallust must be installed
///
/// # Errors
/// Returns an error if the command fails
fn pallete_generator(main_wallpaper: &str) -> anyhow::Result<()> {
    let (ok, output) = exec::run("wallust", &[&"run", &"-q", &"-u", &main_wallpaper])?;

    if !ok {
        anyhow::bail!("wallust failed: {}", output);
    }

    if output.contains("Error") {
        anyhow::bail!("wallust failed: {}", output);
    }

    Ok(())
}

/// Apply the theme
///
/// Apply the generated color pallete to the UI elements.
///
/// # Requirements
/// * hyprctl must be installed
///
/// # Errors
/// Returns an error if the command fails
fn apply_theme() -> anyhow::Result<()> {
    let check = |(ok, out): (bool, String)| {
        if ok {
            Ok(out)
        } else {
            anyhow::bail!("Cmd failed: {}", out)
        }
    };

    // handles wallpaper application
    if is_installed("hyprpaper") {
        let _ = exec::run("pkill", &[&"hyprpaper"]);
        check(exec::run("hyprctl", &[&"dispatch", &"exec", &"hyprpaper"])?)?;
    }

    // handles waybar colorscheme
    if is_installed("waybar") {
        let _ = exec::run("pkill", &[&"waybar"]);
        check(exec::run("hyprctl", &[&"dispatch", &"exec", &"waybar"])?)?;
    }

    // handles swaync colorscheme
    if is_installed("swaync") {
        let (swaync_running, _) = exec::run("pidof", &[&"swaync"])?;
        if !swaync_running {
            check(exec::run("hyprctl", &[&"dispatch", &"exec", &"swaync"])?)?;
        }
        check(exec::run("swaync-client", &[&"-rs"])?)?;
    }

    // handles tmux colorscheme
    if is_installed("tmux") {
        let tmux_config = shellexpand::full("{}/.config/tmux/colors.conf")?;
        let _ = exec::run("tmux", &[&"source-file", &tmux_config.as_ref()]);
    }

    // handles kitty colorscheme
    if is_installed("kitty") {
        let (kitty_running, _) = exec::run("pgrep", &[&"kitty"])?;
        if kitty_running {
            check(exec::run("pkill", &[&"-USR1", &"kitty"])?)?;
        }
    }

    Ok(())
}
