//! Default values
//!
//! Generate default values for the application configuration
use std::path::PathBuf;

/// Macro for generating default functions
macro_rules! default_fns {
    ($($(#[$meta:meta])* $name:ident => $val:expr), *) => {
        $(
            $(#[$meta])*
            pub(crate) fn $name() -> String { $val.into() }
        )*
    };
}

/// Macro for generating default functions with path
macro_rules! default_paths_fns {
    ($($(#[$meta:meta])* $name:ident => $val:expr), *) => {
        $(
            $(#[$meta])*
            pub(crate) fn $name() -> PathBuf { PathBuf::from(shellexpand::full($val).unwrap().as_ref()) }
        )*
    };
}

default_fns! {
    /// Default values for the editor
    default_editor => "nano",
    /// Default values for the browser
    default_browser => "firefox",
    /// Default values for the search engine
    default_search_engine => "https://duckduckgo.com/?q="
}

default_paths_fns! {
    /// Default path for the wallpaper directory
    default_wallpaper_dir => "$HOME/Pictures/wallpapers/",
    /// Default path for the hyprpaper configuration
    default_hyprpaper_path => "$HOME/.config/hypr/hyprpaper.conf",
    /// Default path for the database
    default_db_path => "$HOME/.cache/louarch/bookmarks.sqlite"
}

/// Default values (vector of paths) for the application configuration
pub(crate) fn default_tmux_dirs() -> Vec<PathBuf> {
    Vec::<PathBuf>::new()
}
