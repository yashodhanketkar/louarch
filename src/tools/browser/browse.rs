//! Handles browsing functionality
use rusqlite::Connection;

use crate::{
    context::Context,
    utils::exec::{rofi_prompt, run},
};

use crate::tools::browser::_internal::open_db;

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
    let conn = open_db(&ctx.config.db_path)?;
    let bookmarks = &get_bookmarks(&conn, true)?;
    let output = rofi_prompt("Open: ", bookmarks, false)?
        .ok_or_else(|| anyhow::anyhow!("Url not found. Cancelling."))?;

    match output.as_str() {
        "Add" => add_bookmark(&conn),
        "Remove" => remove_bookmark(&conn),
        opts => {
            let (success, _) = run(&ctx.config.browser, [opts])?;
            anyhow::ensure!(success, "Failed to open browser");
            Ok(())
        }
    }
}

/// Add a bookmark to the database
///
/// # Arguments
/// * `conn` - Database connection
///
/// # Errors
/// Returns an error
/// * user cancels
/// * bookmark already exists
/// * failed to add bookmark
fn add_bookmark(conn: &Connection) -> anyhow::Result<()> {
    let bookmark = rofi_prompt("Add: ", std::iter::empty::<&str>(), false)?;
    conn.execute("INSERT INTO bookmarks (url) VALUES (?1)", [&bookmark])?;

    Ok(())
}

/// Delete a bookmark from database
///
/// # Arguments
/// * `conn` - Database connection
///
/// # Errors
/// Returns an error
/// * user cancels
/// * bookmark doe not exists
/// * failed to delete bookmark
fn remove_bookmark(conn: &Connection) -> anyhow::Result<()> {
    let bookmarks = &get_bookmarks(conn, false)?;
    // let bookmarks_ref = &bookmarks.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    // let bookmark = rofi_prompt("Select to remove", bookmarks_ref, true)?
    let bookmark = rofi_prompt("Select to remove", bookmarks, true)?
        .ok_or_else(|| anyhow::anyhow!("No such bookmark found!. Cancelling."))?;
    conn.execute("delete from bookmarks where url = (?1)", [bookmark])?;

    Ok(())
}

/// Get bookmarks from database
///
/// # Arguments
/// * `conn` - Database connection
/// * `with_opts` - Whether to include options with the list
///
/// # Errors
/// Returns an error
/// * failed to fetch bookmarks
fn get_bookmarks(conn: &Connection, with_opts: bool) -> anyhow::Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT url FROM bookmarks")?;
    let urls_iter = stmt.query_map([], |row| row.get(0))?;

    let mut urls = Vec::new();
    for url in urls_iter {
        urls.push(url?);
    }

    if with_opts {
        urls.extend(["Add", "Remove"].into_iter().map(String::from));
    }

    Ok(urls)
}
