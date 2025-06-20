use thiserror::Error;

/// Main error type for Bryndza automation operations
#[derive(Error, Debug)]
pub enum BryndzaError {
    /// Element not found with the given locator
    #[error("Element not found: {locator}")]
    ElementNotFound { locator: String },

    /// Element found but not visible/interactable
    #[error("Element not interactable: {reason}")]
    ElementNotInteractable { reason: String },

    /// Timeout occurred while waiting for condition
    #[error("Timeout after {duration:?}: {condition}")]
    Timeout {
        duration: std::time::Duration,
        condition: String,
    },

    /// Platform-specific connection error
    #[error("Connection error: {message}")]
    ConnectionError { message: String },

    /// Invalid configuration
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    /// Platform not supported
    #[error("Platform not supported: {platform}")]
    PlatformNotSupported { platform: String },

    /// Screenshot operation failed
    #[error("Screenshot failed: {reason}")]
    ScreenshotError { reason: String },

    /// Image comparison failed
    #[error("Image comparison failed: {reason}")]
    ImageComparisonError { reason: String },

    /// Session operation failed
    #[error("Session error: {message}")]
    SessionError { message: String },

    /// Platform-specific errors
    #[error("Platform error: {0}")]
    Platform(#[from] PlatformError),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Platform-specific error types
#[derive(Error, Debug)]
pub enum PlatformError {
    /// Windows UI Automation errors
    #[error("Windows UI Automation error: {message}")]
    WindowsUIAutomation { message: String },

    /// macOS Accessibility API errors
    #[error("macOS Accessibility error: {message}")]
    MacOSAccessibility { message: String },

    /// Android ADB errors
    #[error("Android ADB error: {message}")]
    AndroidADB { message: String },

    /// iOS XCTest errors
    #[error("iOS XCTest error: {message}")]
    IOSXCTest { message: String },
}

/// Result type alias for Bryndza operations
pub type Result<T> = std::result::Result<T, BryndzaError>;

impl BryndzaError {
    /// Creates a new element not found error
    pub fn element_not_found<S: Into<String>>(locator: S) -> Self {
        Self::ElementNotFound {
            locator: locator.into(),
        }
    }

    /// Creates a new element not interactable error
    pub fn element_not_interactable<S: Into<String>>(reason: S) -> Self {
        Self::ElementNotInteractable {
            reason: reason.into(),
        }
    }

    /// Creates a new timeout error
    pub fn timeout<S: Into<String>>(duration: std::time::Duration, condition: S) -> Self {
        Self::Timeout {
            duration,
            condition: condition.into(),
        }
    }

    /// Creates a new connection error
    pub fn connection<S: Into<String>>(message: S) -> Self {
        Self::ConnectionError {
            message: message.into(),
        }
    }

    /// Creates a new configuration error
    pub fn configuration<S: Into<String>>(message: S) -> Self {
        Self::ConfigurationError {
            message: message.into(),
        }
    }

    /// Creates a new platform not supported error
    pub fn platform_not_supported<S: Into<String>>(platform: S) -> Self {
        Self::PlatformNotSupported {
            platform: platform.into(),
        }
    }

    /// Creates a new screenshot error
    pub fn screenshot<S: Into<String>>(reason: S) -> Self {
        Self::ScreenshotError {
            reason: reason.into(),
        }
    }

    /// Creates a new session error
    pub fn session<S: Into<String>>(message: S) -> Self {
        Self::SessionError {
            message: message.into(),
        }
    }
}
