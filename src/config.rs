use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for Bryndza automation sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Default timeout for element operations
    pub default_timeout: Duration,
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Delay between retry attempts
    pub retry_delay: Duration,
    /// Platform-specific configuration
    pub platform: PlatformConfig,
    /// Screenshot configuration
    pub screenshot: ScreenshotConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_timeout: Duration::from_secs(10),
            max_retries: 3,
            retry_delay: Duration::from_millis(500),
            platform: PlatformConfig::default(),
            screenshot: ScreenshotConfig::default(),
        }
    }
}

/// Platform-specific configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    /// Windows-specific settings
    pub windows: WindowsConfig,
    /// macOS-specific settings
    pub macos: MacOSConfig,
    /// Android-specific settings
    pub android: AndroidConfig,
    /// iOS-specific settings
    pub ios: IOSConfig,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            windows: WindowsConfig::default(),
            macos: MacOSConfig::default(),
            android: AndroidConfig::default(),
            ios: IOSConfig::default(),
        }
    }
}

/// Windows UI Automation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsConfig {
    /// Enable UI Automation tree traversal optimization
    pub optimize_tree_traversal: bool,
    /// Cache UI elements for better performance
    pub enable_element_caching: bool,
}

impl Default for WindowsConfig {
    fn default() -> Self {
        Self {
            optimize_tree_traversal: true,
            enable_element_caching: true,
        }
    }
}

/// macOS Accessibility API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacOSConfig {
    /// Enable accessibility permissions check
    pub check_accessibility_permissions: bool,
    /// Use system events for interactions
    pub use_system_events: bool,
}

impl Default for MacOSConfig {
    fn default() -> Self {
        Self {
            check_accessibility_permissions: true,
            use_system_events: false,
        }
    }
}

/// Android automation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidConfig {
    /// ADB connection timeout
    pub adb_timeout: Duration,
    /// Device serial number (if specific device required)
    pub device_serial: Option<String>,
    /// Enable UI Automator optimizations
    pub enable_ui_automator_optimizations: bool,
}

impl Default for AndroidConfig {
    fn default() -> Self {
        Self {
            adb_timeout: Duration::from_secs(30),
            device_serial: None,
            enable_ui_automator_optimizations: true,
        }
    }
}

/// iOS automation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOSConfig {
    /// XCTest bundle identifier
    pub xctest_bundle_id: Option<String>,
    /// Device UDID (if specific device required)
    pub device_udid: Option<String>,
    /// Enable XCTest optimizations
    pub enable_xctest_optimizations: bool,
}

impl Default for IOSConfig {
    fn default() -> Self {
        Self {
            xctest_bundle_id: None,
            device_udid: None,
            enable_xctest_optimizations: true,
        }
    }
}

/// Screenshot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotConfig {
    /// Default image format for screenshots
    pub format: ImageFormat,
    /// Compression quality (0-100)
    pub quality: u8,
    /// Enable automatic screenshot on failure
    pub auto_screenshot_on_failure: bool,
}

impl Default for ScreenshotConfig {
    fn default() -> Self {
        Self {
            format: ImageFormat::PNG,
            quality: 90,
            auto_screenshot_on_failure: true,
        }
    }
}

/// Supported image formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    PNG,
    JPEG,
    WEBP,
}

impl From<ImageFormat> for crate::utils::image::ImageFormat {
    fn from(format: ImageFormat) -> Self {
        match format {
            ImageFormat::PNG => crate::utils::image::ImageFormat::PNG,
            ImageFormat::JPEG => crate::utils::image::ImageFormat::JPEG,
            ImageFormat::WEBP => crate::utils::image::ImageFormat::WEBP,
        }
    }
}
