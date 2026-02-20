//! Internal module containing database operations
use std::path::PathBuf;

use rusqlite::Connection;

/// Load the database connection
///
/// # Arguments
/// * `self` - Application configuration
///
/// # Errors
/// Returns an error if
/// * the database cannot be opened
/// * initialization fails
pub(crate) fn open_db(db_path: &PathBuf) -> anyhow::Result<Connection> {
    let conn = Connection::open(db_path)?;
    init_db(&conn)?;
    Ok(conn)
}

/// Initialize the database
///
/// This function creates the database tables if they do not exist.
///
/// # Arguments
/// * `conn` - Database connection
///
/// # Errors
/// Returns an error if table fails to be created
fn init_db(conn: &Connection) -> anyhow::Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS bookmarks (
            id INTEGER PRIMARY KEY,
            url TEXT NOT NULL UNIQUE
        );

        CREATE INDEX IF NOT EXISTS idx_bookmarks_url ON bookmarks(url);
        "#,
    )?;

    Ok(())
}
