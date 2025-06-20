use crate::{config::Config, error::Result, platform::Platform};
use uuid::Uuid;

/// Represents an automation session for interacting with applications
pub struct Session {
    /// Unique session identifier
    pub id: Uuid,
    /// Session configuration
    pub config: Config,
    /// Platform implementation
    platform: Box<dyn Platform>,
}

impl Session {
    /// Creates a new automation session
    pub fn new(config: Config) -> Result<Self> {
        let platform = crate::platform::detect_and_create(&config)?;

        Ok(Self {
            id: Uuid::new_v4(),
            config,
            platform,
        })
    }

    /// Creates a new session with default configuration
    pub fn builder() -> SessionBuilder {
        SessionBuilder::default()
    }

    /// Starts the automation session
    pub async fn start(&mut self) -> Result<()> {
        self.platform.connect().await?;
        Ok(())
    }

    /// Stops the automation session
    pub async fn stop(&mut self) -> Result<()> {
        self.platform.disconnect().await?;
        Ok(())
    }

    /// Takes a screenshot of the current screen
    pub async fn screenshot(&self) -> Result<Vec<u8>> {
        self.platform.screenshot().await
    }

    /// Finds an element using the specified locator
    pub async fn find_element(
        &self,
        locator: &crate::element::Locator,
    ) -> Result<crate::element::Element> {
        self.platform.find_element(locator).await
    }

    /// Finds multiple elements using the specified locator
    pub async fn find_elements(
        &self,
        locator: &crate::element::Locator,
    ) -> Result<Vec<crate::element::Element>> {
        self.platform.find_elements(locator).await
    }

    /// Waits for an element to become available
    pub async fn wait_for_element(
        &self,
        locator: &crate::element::Locator,
    ) -> Result<crate::element::Element> {
        crate::wait::wait_for_element(self, locator, self.config.default_timeout).await
    }

    /// Gets the platform implementation
    pub fn platform(&self) -> &dyn Platform {
        self.platform.as_ref()
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // Best effort cleanup
        if let Err(e) = futures::executor::block_on(self.platform.disconnect()) {
            eprintln!("Warning: Failed to disconnect session {}: {}", self.id, e);
        }
    }
}

/// Builder for creating Session instances with custom configuration
#[derive(Default)]
pub struct SessionBuilder {
    config: Config,
}

impl SessionBuilder {
    /// Sets the default timeout for operations
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config.default_timeout = timeout;
        self
    }

    /// Sets the maximum number of retry attempts
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.config.max_retries = max_retries;
        self
    }

    /// Sets the delay between retry attempts
    pub fn retry_delay(mut self, delay: std::time::Duration) -> Self {
        self.config.retry_delay = delay;
        self
    }

    /// Sets the configuration
    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    /// Builds the session
    pub fn build(self) -> Result<Session> {
        Session::new(self.config)
    }
}
