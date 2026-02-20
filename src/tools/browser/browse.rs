//! Handles browsing functionality
use crate::{
    context::Context,
    utils::exec::{rofi_prompt, run},
};

/// Browse a url or bookmark
///
/// Opens the url or bookmark in the configured browser
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Errors
/// Returns an error
/// * user cancels
/// * browser fails
pub(crate) fn handle(ctx: &Context) -> anyhow::Result<()> {
    let bookmarks = &get_bookmarks(&ctx, true)?;
    let options: Vec<&str> = bookmarks.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    let output = rofi_prompt("Open: ", &options, false)?
        .ok_or_else(|| anyhow::anyhow!("Url not found. Cancelling."))?;

    match output.as_str() {
        "Add" => add_bookmark(&ctx),
        "Remove" => remove_bookmark(&ctx),
        opts => {
            let (success, _) = run(&ctx.config.browser, &[opts])?;
            anyhow::ensure!(success, "Failed to open browser");
            Ok(())
        }
    }
}

/// Add a bookmark to the database
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Errors
/// Returns an error
/// * user cancels
/// * bookmark already exists
/// * failed to add bookmark
fn add_bookmark(ctx: &Context) -> anyhow::Result<()> {
    let bookmark = rofi_prompt("Add: ", &[], false)?;
    ctx.db
        .execute("INSERT INTO bookmarks (url) VALUES (?1)", [&bookmark])?;

    Ok(())
}

/// Delete a bookmark from database
///
/// # Arguments
/// * `ctx` - Context containing the configuration
///
/// # Errors
/// Returns an error
/// * user cancels
/// * bookmark doe not exists
/// * failed to delete bookmark
fn remove_bookmark(ctx: &Context) -> anyhow::Result<()> {
    let bookmarks = &get_bookmarks(ctx, false)?;
    let bookmarks_ref = &bookmarks.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let bookmark = rofi_prompt("Select to remove", &bookmarks_ref, true)?
        .ok_or_else(|| anyhow::anyhow!("No such bookmark found!. Cancelling."))?;
    ctx.db
        .execute("delete from bookmarks where url = (?1)", [bookmark])?;

    Ok(())
}

/// Get bookmarks from database
///
/// # Arguments
/// * `ctx` - Context containing the configuration
/// * `with_opts` - Whether to include options with the list
///
/// # Errors
/// Returns an error
/// * failed to fetch bookmarks
fn get_bookmarks(ctx: &Context, with_opts: bool) -> anyhow::Result<Vec<String>> {
    let mut stmt = ctx.db.prepare("SELECT url FROM bookmarks")?;
    let urls_iter = stmt.query_map([], |row| Ok(row.get(0)?))?;
    let mut urls = Vec::new();
    for url in urls_iter {
        urls.push(url?);
    }

    if with_opts {
        urls.extend(["Add", "Remove"].into_iter().map(String::from));
    }

    Ok(urls)
}
