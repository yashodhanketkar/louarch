//! This module contains the `Context` struct which is the central
//! information store for the app.
mod config;
mod system;

use crate::context::{config::AppConfig, system::SystemConfig};

/// Central information store for the app
///
/// `Context` gathers all system information and provides a unified
/// interface to the rest of the app. It is single source of truth
/// for the app.
pub struct Context {
    /// System configuration
    pub system: SystemConfig,
    /// Application configuration
    pub config: AppConfig,
}

impl Context {
    /// Initialize a new `Context` instance
    ///
    /// This contructor will load all the information needed by the
    /// app and
    ///
    /// # Errors
    /// Returns an error if
    /// * the system configuration cannot be loaded
    /// * the application configuration cannot be loaded
    pub fn new() -> anyhow::Result<Self> {
        let system = SystemConfig::load()?;
        let config = AppConfig::load()?;

        Ok(Self { system, config })
    }
}
