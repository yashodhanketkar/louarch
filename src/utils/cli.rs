//! CLI structure and command methods
//!
//! This module contains the CLI structure and the command methods
//! available to the user.
use clap::{Parser, Subcommand};
use clap_complete::Shell;

/// Handles the CLI
///
/// This struct contains the CLI struct which is the central action
/// methods available to the user.
#[derive(Parser, Debug)]
#[command(name = "louarch")]
#[command(about = "Single-binary system utility")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// The command enum contains the commands available to the user
#[derive(Subcommand, Debug)]
pub enum Command {
    Wallpaper {
        #[command(subcommand)]
        action: Wallpaper,
    },
    Osmode {
        #[command(subcommand)]
        action: Osmode,
    },
    Audio {
        #[command(subcommand)]
        action: Audio,
    },
    Wifi {
        #[command(subcommand)]
        action: Wifi,
    },
    Bluetooth {
        #[command(subcommand)]
        action: Bluetooth,
    },
    Browser {
        #[command(subcommand)]
        action: Browser,
    },
    Tmux {
        #[command(subcommand)]
        action: Tmux,
    },
    /// Set of subcommands for completion actions
    Completions {
        #[arg(value_enum)]
        action: Shell,

        #[arg(short, long)]
        silent: bool,
    },
    Config {
        #[command(subcommand)]
        action: Config,
    },
}

/// Set of subcommands for wallpaper actions
#[derive(Subcommand, Debug)]
pub enum Wallpaper {
    /// Assigns random wallpaper to all monitors
    Random,
    /// Selects a wallpaper for each monitor
    Select,
}

/// Set of subcommands for osmode actions
#[derive(Subcommand, Debug)]
pub enum Osmode {
    /// Toggles game mode
    Game,
    /// Toggles night mode
    Night,
}

/// Set of subcommands for audio actions
#[derive(Subcommand, Debug)]
pub enum Audio {
    /// Toggles audio output
    Sink,
    /// Toggles audio input
    Source,
}

/// Set of subcommands for browser actions
#[derive(Subcommand, Debug)]
pub enum Browser {
    /// Search for term
    Search,
    /// Open a URL
    Browse,
}

/// Set of wifi actions for network subcommand
#[derive(Subcommand, Debug)]
pub enum Wifi {
    /// Connect to a WiFi network
    Connect,
    /// Disconnect from a WiFi network
    Disconnect,
}

/// Set of bluetooth actions for network subcommand
#[derive(Subcommand, Debug)]
pub enum Bluetooth {
    /// Connect to a bluetooth device
    Connect,
    /// Disconnect from a bluetooth device
    Disconnect,
}

/// Set of subcommands for tmux actions
#[derive(Subcommand, Debug)]
pub enum Tmux {
    /// Attach to a session (creates it if missing)
    Attach,
    /// Kill a tmux session
    Kill,
}

/// Set of subcommands for config actions
#[derive(Subcommand, Debug)]
pub enum Config {
    /// View the config
    View,
    /// Edit the config file
    Edit,
}
