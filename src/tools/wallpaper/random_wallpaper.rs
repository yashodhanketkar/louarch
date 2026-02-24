//! Handle random wallpapers action
use rand::seq::SliceRandom;

use crate::context::Context;
use crate::tools::wallpaper::{
    _internal::{list_images, pathbuf_to_string},
    apply::orchestrator,
};

/// Apply a random wallpaper
///
/// This function will apply a random wallpaper to all monitors.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Errors
/// Returns an error if randomizer or orchestrator fails
pub(crate) fn switch(ctx: &Context) -> anyhow::Result<()> {
    let imgs = randomizer(ctx)?;
    orchestrator(ctx, &imgs)?;
    Ok(())
}

/// Generate a random wallpaper
///
/// This function will generate a random wallpaper from the wallpapers
/// directory.
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Errors
/// Returns an error if the wallpapers directory cannot be found
pub fn randomizer(ctx: &Context) -> anyhow::Result<Vec<String>> {
    let images = pathbuf_to_string(list_images(&ctx.config.wallpaper_dir)?);

    let img = images
        .choose(&mut rand::thread_rng())
        .map(|p| p.to_string())
        .ok_or_else(|| anyhow::anyhow!("No wallpapers found"))?;

    Ok(vec![img])
}
