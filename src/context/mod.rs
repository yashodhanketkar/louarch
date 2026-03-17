//! Context for the app
//!
//! Gather all ssystem information and application configuration
//! and provide a unified interface to the rest of the app.
pub mod app;
pub mod system;

/// Central information store for the app
///
/// `Context` gathers all system information and provides a unified
/// interface to the rest of the app. It is single source of truth
/// for the app.
pub struct Context {
    /// System configuration
    pub system: system::Config,
    /// Application configuration
    pub app: app::Config,
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
        let system = system::Config::load()?;
        let app = app::Config::load()?;

        Ok(Self { system, app })
    }
}
