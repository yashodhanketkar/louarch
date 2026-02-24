//! Handles various notifications
use crate::utils::exec::run;

/// Handles notifications types
pub enum NotifyType {
    Warn,
    Info,
    Hint,
    Error,
    Confused,
    Ok,
}

/// Returns the types as a numerical value with respect to hyprctl notify
/// documentation
impl NotifyType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Warn => "0",
            Self::Info => "1",
            Self::Hint => "2",
            Self::Error => "3",
            Self::Confused => "4",
            Self::Ok => "5",
        }
    }
}

/// Handles notifications colors
pub enum NotifyColor {
    Red,
    Green,
    Yellow,
}

/// Returns hex color codes for notification colors
impl NotifyColor {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Red => "rgb(FF0000)",
            Self::Green => "rgb(00FF00)",
            Self::Yellow => "rgb(FFFF00)",
        }
    }
}

/// Send notification to system
///
/// # Arguments
/// * `icon` - Notification icon
/// * `color` - Notification color
/// * `msg` - Notification message
///
/// # Requirements
/// * hyprctl must be installed
///
/// # Errors
/// Returns an error if hyprctl fails
pub fn send(icon: NotifyType, color: NotifyColor, msg: &str) -> anyhow::Result<()> {
    run(
        "hyprctl",
        ["notify", icon.as_str(), "5000", color.as_str(), msg],
    )?;
    Ok(())
}
