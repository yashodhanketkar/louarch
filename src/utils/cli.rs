//! Handles the CLI
//!
//! This module contains the CLI struct which is the central
//! action methods available to the user.
use clap::{Parser, Subcommand};
use clap_complete::Shell;

/// Handles the CLI
///
/// This struct contains the CLI struct which is the central
/// action methods available to the user.
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
    Network {
        #[command(subcommand)]
        action: Network,
    },
    Browser {
        #[command(subcommand)]
        action: Browser,
    },
    Completions {
        #[arg(value_enum)]
        action: Shell,

        #[arg(short, long)]
        silent: bool,
    },
}

/// Set of subcommands for wallpaper actions
#[derive(Subcommand, Debug)]
pub enum Wallpaper {
    /// Assings random wallpaper to all monitors
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

/// Set of subcommands for newtork actions
#[derive(Subcommand, Debug)]
pub enum Network {
    /// Select a WiFi device
    Wifi {
        #[command(subcommand)]
        action: WifiAction,
    },
    /// Select a bluetooth device
    Bluetooth {
        #[command(subcommand)]
        action: BluetoothAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum WifiAction {
    /// Connect to a WiFi network
    Connect,
    /// Connect to a WiFi network
    Disconnect,
}

#[derive(Subcommand, Debug)]
pub enum BluetoothAction {
    /// Connect to a WiFi network
    Connect,
    /// Connect to a WiFi network
    Disconnect,
}
