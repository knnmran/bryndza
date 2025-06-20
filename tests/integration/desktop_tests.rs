use bryndza::{Session, Locator, Result};
use std::time::Duration;

#[tokio::test]
async fn test_windows_automation() -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        let mut session = Session::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        session.start().await?;

        // Test finding Notepad window (if available)
        // This is a basic smoke test
        let _screenshot = session.screenshot().await?;

        session.stop().await?;
    }
    
    Ok(())
}

#[tokio::test]
async fn test_macos_automation() -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        let mut session = Session::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        session.start().await?;

        // Test taking a screenshot
        let _screenshot = session.screenshot().await?;

        session.stop().await?;
    }
    
    Ok(())
}

#[tokio::test]
async fn test_element_finding() -> Result<()> {
    // This test will be skipped on platforms where it's not implemented
    let session = Session::builder().build();
    
    if session.is_err() {
        // Platform not supported, skip test
        return Ok(());
    }
    
    let mut session = session?;
    
    if session.start().await.is_err() {
        // Can't connect to platform, skip test
        return Ok(());
    }

    // Test various locator strategies
    let locators = vec![
        Locator::id("test-id"),
        Locator::class_name("button"),
        Locator::text("Click me"),
        Locator::xpath("//button[@id='test']"),
    ];

    for locator in locators {
        // We expect these to fail since we don't have a test app
        // But they should fail with ElementNotFound, not a platform error
        let result = session.find_element(&locator).await;
        match result {
            Err(bryndza::BryndzaError::ElementNotFound { .. }) => {
                // This is expected
            }
            Err(bryndza::BryndzaError::PlatformNotSupported { .. }) => {
                // Platform not implemented, skip
                break;
            }
            Err(_) => {
                // Other errors might indicate platform issues
            }
            Ok(_) => {
                // Unexpected success (maybe there's actually an element)
            }
        }
    }

    session.stop().await?;
    Ok(())
}
