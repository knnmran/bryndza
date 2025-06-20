//! # Bryndza
//! 
//! A cross-platform UI automation library for desktop and mobile applications.
//! 
//! Bryndza provides a unified API for automating user interfaces across different platforms:
//! - **Windows**: UI Automation API
//! - **macOS**: Accessibility API
//! - **Android**: ADB + UI Automator
//! - **iOS**: XCTest framework
//! 
//! ## Quick Start
//! 
//! ```rust,no_run
//! use bryndza::{Session, Locator};
//! 
//! #[tokio::main]
//! async fn main() -> bryndza::Result<()> {
//!     let mut session = Session::builder()
//!         .timeout(std::time::Duration::from_secs(30))
//!         .build()?;
//!     
//!     session.start().await?;
//!     
//!     let button = session.find_element(&Locator::id("my-button")).await?;
//!     button.click().await?;
//!     
//!     session.stop().await?;
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod error;
pub mod session;
pub mod element;
pub mod platform;
pub mod wait;
pub mod utils;

// Re-export commonly used types
pub use config::Config;
pub use error::{BryndzaError, Result};
pub use session::{Session, SessionBuilder};
pub use element::{Element, Locator};
pub use wait::{WaitConditions, WaitStrategy};

// Re-export platform capabilities for advanced usage
pub use platform::{Platform, PlatformCapabilities};
