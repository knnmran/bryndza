pub mod xctest;
pub mod element;

use crate::{
    config::Config,
    element::{Element, Locator},
    error::Result,
    platform::{Platform, PlatformCapabilities},
};
use async_trait::async_trait;

/// iOS platform implementation using XCTest framework
pub struct IOSPlatform {
    config: Config,
    xctest_connection: Option<xctest::XCTestConnection>,
}

impl IOSPlatform {
    /// Creates a new iOS platform instance
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self {
            config,
            xctest_connection: None,
        })
    }
}

#[async_trait]
impl Platform for IOSPlatform {
    async fn connect(&mut self) -> Result<()> {
        let xctest_connection = xctest::XCTestConnection::new(&self.config).await?;
        self.xctest_connection = Some(xctest_connection);
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        if let Some(connection) = self.xctest_connection.take() {
            connection.disconnect().await?;
        }
        Ok(())
    }

    async fn screenshot(&self) -> Result<Vec<u8>> {
        let connection = self.xctest_connection.as_ref()
            .ok_or_else(|| crate::error::BryndzaError::session("Not connected to iOS device"))?;
        connection.screenshot().await
    }

    async fn find_element(&self, locator: &Locator) -> Result<Element> {
        let connection = self.xctest_connection.as_ref()
            .ok_or_else(|| crate::error::BryndzaError::session("Not connected to iOS device"))?;
        connection.find_element(locator).await
    }

    async fn find_elements(&self, locator: &Locator) -> Result<Vec<Element>> {
        let connection = self.xctest_connection.as_ref()
            .ok_or_else(|| crate::error::BryndzaError::session("Not connected to iOS device"))?;
        connection.find_elements(locator).await
    }

    async fn element_exists(&self, locator: &Locator) -> Result<bool> {
        match self.find_element(locator).await {
            Ok(_) => Ok(true),
            Err(crate::error::BryndzaError::ElementNotFound { .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn click_element(&self, element: &Element) -> Result<()> {
        let connection = self.xctest_connection.as_ref()
            .ok_or_else(|| crate::error::BryndzaError::session("Not connected to iOS device"))?;
        connection.tap_element(element).await
    }

    async fn double_click_element(&self, element: &Element) -> Result<()> {
        let connection = self.xctest_connection.as_ref()
            .ok_or_else(|| crate::error::BryndzaError::session("Not connected to iOS device"))?;
        connection.double_tap_element(element).await
    }

    async fn long_press_element(&self, element: &Element, duration: std::time::Duration) -> Result<()> {
        let connection = self.xctest_connection.as_ref()
            .ok_or_else(|| crate::error::BryndzaError::session("Not connected to iOS device"))?;
        connection.long_press_element(element, duration).await
    }

    async fn type_text(&self, element: &Element, text: &str) -> Result<()> {
        let connection = self.xctest_connection.as_ref()
            .ok_or_else(|| crate::error::BryndzaError::session("Not connected to iOS device"))?;
        connection.type_text(element, text).await
    }

    async fn clear_element(&self, element: &Element) -> Result<()> {
        let connection = self.xctest_connection.as_ref()
            .ok_or_else(|| crate::error::BryndzaError::session("Not connected to iOS device"))?;
        connection.clear_element(element).await
    }

    async fn element_screenshot(&self, element: &Element) -> Result<Vec<u8>> {
        let connection = self.xctest_connection.as_ref()
            .ok_or_else(|| crate::error::BryndzaError::session("Not connected to iOS device"))?;
        connection.element_screenshot(element).await
    }

    async fn scroll_into_view(&self, element: &Element) -> Result<()> {
        let connection = self.xctest_connection.as_ref()
            .ok_or_else(|| crate::error::BryndzaError::session("Not connected to iOS device"))?;
        connection.scroll_into_view(element).await
    }

    async fn swipe_element(&self, element: &Element, direction: crate::element::element::SwipeDirection, distance: f64) -> Result<()> {
        let connection = self.xctest_connection.as_ref()
            .ok_or_else(|| crate::error::BryndzaError::session("Not connected to iOS device"))?;
        connection.swipe_element(element, direction, distance).await
    }

    fn platform_name(&self) -> &'static str {
        "iOS"
    }

    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities {
            supports_touch: true,
            supports_mouse: false,
            supports_keyboard: true,
            supports_image_location: true,
            supports_accessibility: true,
            supports_multiple_windows: false, // iOS apps typically single window
            supports_screenshots: true,
        }
    }
}
