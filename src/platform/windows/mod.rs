pub mod connection;
pub mod element;

use crate::{
    config::Config,
    element::{Element, Locator},
    error::Result,
    platform::{Platform, PlatformCapabilities},
};
use async_trait::async_trait;

/// Windows platform implementation using UI Automation
pub struct WindowsPlatform {
    config: Config,
    connection: Option<connection::WindowsConnection>,
}

impl WindowsPlatform {
    /// Creates a new Windows platform instance
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self {
            config,
            connection: None,
        })
    }
}

#[async_trait]
impl Platform for WindowsPlatform {
    async fn connect(&mut self) -> Result<()> {
        let connection = connection::WindowsConnection::new(&self.config).await?;
        self.connection = Some(connection);
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        if let Some(connection) = self.connection.take() {
            connection.disconnect().await?;
        }
        Ok(())
    }

    async fn screenshot(&self) -> Result<Vec<u8>> {
        let connection = self.connection.as_ref().ok_or_else(|| {
            crate::error::BryndzaError::session("Not connected to Windows platform")
        })?;
        connection.screenshot().await
    }

    async fn find_element(&self, locator: &Locator) -> Result<Element> {
        let connection = self.connection.as_ref().ok_or_else(|| {
            crate::error::BryndzaError::session("Not connected to Windows platform")
        })?;
        connection.find_element(locator).await
    }

    async fn find_elements(&self, locator: &Locator) -> Result<Vec<Element>> {
        let connection = self.connection.as_ref().ok_or_else(|| {
            crate::error::BryndzaError::session("Not connected to Windows platform")
        })?;
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
        let connection = self.connection.as_ref().ok_or_else(|| {
            crate::error::BryndzaError::session("Not connected to Windows platform")
        })?;
        connection.click_element(element).await
    }

    async fn double_click_element(&self, element: &Element) -> Result<()> {
        let connection = self.connection.as_ref().ok_or_else(|| {
            crate::error::BryndzaError::session("Not connected to Windows platform")
        })?;
        connection.double_click_element(element).await
    }

    async fn long_press_element(
        &self,
        element: &Element,
        duration: std::time::Duration,
    ) -> Result<()> {
        // Long press is not typically used on Windows desktop
        // We'll simulate it with a regular click for now
        self.click_element(element).await
    }

    async fn type_text(&self, element: &Element, text: &str) -> Result<()> {
        let connection = self.connection.as_ref().ok_or_else(|| {
            crate::error::BryndzaError::session("Not connected to Windows platform")
        })?;
        connection.type_text(element, text).await
    }

    async fn clear_element(&self, element: &Element) -> Result<()> {
        let connection = self.connection.as_ref().ok_or_else(|| {
            crate::error::BryndzaError::session("Not connected to Windows platform")
        })?;
        connection.clear_element(element).await
    }

    async fn element_screenshot(&self, element: &Element) -> Result<Vec<u8>> {
        let connection = self.connection.as_ref().ok_or_else(|| {
            crate::error::BryndzaError::session("Not connected to Windows platform")
        })?;
        connection.element_screenshot(element).await
    }

    async fn scroll_into_view(&self, element: &Element) -> Result<()> {
        let connection = self.connection.as_ref().ok_or_else(|| {
            crate::error::BryndzaError::session("Not connected to Windows platform")
        })?;
        connection.scroll_into_view(element).await
    }

    async fn swipe_element(
        &self,
        element: &Element,
        direction: crate::element::element::SwipeDirection,
        distance: f64,
    ) -> Result<()> {
        // Swipe gestures are not typically used on Windows desktop
        // We could implement this as mouse drag in the future
        Err(crate::error::BryndzaError::platform_not_supported(
            "Swipe gestures not supported on Windows desktop",
        ))
    }

    fn platform_name(&self) -> &'static str {
        "Windows"
    }

    fn capabilities(&self) -> PlatformCapabilities {
        PlatformCapabilities {
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
