//! This module contains logic for interacting with hyprland.
//! It is broken down into submodules for each of the hyprland
//! APIs.
//!
//! The main module is `mod.rs` which contains the `hypr` struct
//! which is the main entry point for interacting with hyprland.
//!
//! The submodules are `osmode` and `wallpaper` which contain
//! the logic for interacting with the OS modes and wallpapers
//! respectively.
pub mod audio;
pub mod browser;
pub mod completions;
pub mod osmode;
pub mod wallpaper;
