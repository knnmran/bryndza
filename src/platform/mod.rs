pub mod android;
pub mod ios;
pub mod macos;
pub mod windows;

use crate::{
    config::Config,
    element::{Element, Locator},
    error::Result,
};
use async_trait::async_trait;

/// Detects the current platform and creates the appropriate platform implementation
pub fn detect_and_create(config: &Config) -> Result<Box<dyn Platform>> {
    #[cfg(target_os = "windows")]
    {
        Ok(Box::new(windows::WindowsPlatform::new(config.clone())?))
    }

    #[cfg(target_os = "macos")]
    {
        Ok(Box::new(macos::MacOSPlatform::new(config.clone())?))
    }

    #[cfg(target_os = "linux")]
    {
        // For now, we'll try Android first on Linux (for development/testing)
        // In the future, we might add native Linux desktop support
        Ok(Box::new(android::AndroidPlatform::new(config.clone())?))
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err(crate::error::BryndzaError::platform_not_supported(
            std::env::consts::OS,
        ))
    }
}

/// Trait defining the platform-specific automation interface
#[async_trait]
pub trait Platform: Send + Sync {
    /// Establishes connection to the platform's automation service
    async fn connect(&mut self) -> Result<()>;

    /// Disconnects from the platform's automation service
    async fn disconnect(&mut self) -> Result<()>;

    /// Takes a screenshot of the current screen
    async fn screenshot(&self) -> Result<Vec<u8>>;

    /// Finds a single element using the specified locator
    async fn find_element(&self, locator: &Locator) -> Result<Element>;

    /// Finds multiple elements using the specified locator
    async fn find_elements(&self, locator: &Locator) -> Result<Vec<Element>>;

    /// Checks if an element exists without throwing an error
    async fn element_exists(&self, locator: &Locator) -> Result<bool>;

    /// Performs a click action on the specified element
    async fn click_element(&self, element: &Element) -> Result<()>;

    /// Performs a double-click action on the specified element
    async fn double_click_element(&self, element: &Element) -> Result<()>;

    /// Performs a long press on the specified element
    async fn long_press_element(
        &self,
        element: &Element,
        duration: std::time::Duration,
    ) -> Result<()>;

    /// Types text into the specified element
    async fn type_text(&self, element: &Element, text: &str) -> Result<()>;

    /// Clears the text content of the specified element
    async fn clear_element(&self, element: &Element) -> Result<()>;

    /// Takes a screenshot of the specified element
    async fn element_screenshot(&self, element: &Element) -> Result<Vec<u8>>;

    /// Scrolls to bring the element into view
    async fn scroll_into_view(&self, element: &Element) -> Result<()>;

    /// Performs a swipe gesture on the element
    async fn swipe_element(
        &self,
        element: &Element,
        direction: crate::element::element::SwipeDirection,
        distance: f64,
    ) -> Result<()>;

    /// Gets the platform name
    fn platform_name(&self) -> &'static str;

    /// Gets platform-specific capabilities
    fn capabilities(&self) -> PlatformCapabilities;
}

/// Describes the capabilities of a platform
#[derive(Debug, Clone)]
pub struct PlatformCapabilities {
    /// Supports touch interactions
    pub supports_touch: bool,
    /// Supports mouse interactions
    pub supports_mouse: bool,
    /// Supports keyboard interactions
    pub supports_keyboard: bool,
    /// Supports image-based element location
    pub supports_image_location: bool,
    /// Supports accessibility tree navigation
    pub supports_accessibility: bool,
    /// Supports multiple windows/apps
    pub supports_multiple_windows: bool,
    /// Supports screenshot functionality
    pub supports_screenshots: bool,
}

impl Default for PlatformCapabilities {
    fn default() -> Self {
        Self {
            supports_touch: false,
            supports_mouse: true,
            supports_keyboard: true,
            supports_image_location: true,
            supports_accessibility: true,
            supports_multiple_windows: true,
            supports_screenshots: true,
        }
    }
}
