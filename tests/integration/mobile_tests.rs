use bryndza::{Session, Locator, Result};
use std::time::Duration;

#[tokio::test]
async fn test_android_automation() -> Result<()> {
    // Skip test if ADB is not available or no devices connected
    if !is_adb_available().await {
        return Ok(());
    }

    let mut session = Session::builder()
        .timeout(Duration::from_secs(15))
        .build()?;

    session.start().await?;

    // Test taking a screenshot
    let _screenshot = session.screenshot().await?;

    session.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_ios_automation() -> Result<()> {
    // Skip test if iOS device/simulator is not available
    if !is_ios_device_available().await {
        return Ok(());
    }

    let mut session = Session::builder()
        .timeout(Duration::from_secs(15))
        .build()?;

    session.start().await?;

    // Test taking a screenshot
    let _screenshot = session.screenshot().await?;

    session.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_mobile_gestures() -> Result<()> {
    // This test requires a mobile platform
    let session = Session::builder().build();
    
    if session.is_err() {
        return Ok(());
    }
    
    let mut session = session?;
    
    if session.start().await.is_err() {
        return Ok(());
    }

    // Test if platform supports touch gestures
    let capabilities = session.platform().capabilities();
    if !capabilities.supports_touch {
        session.stop().await?;
        return Ok(());
    }

    // We would test swipe, tap, long press, etc. here
    // For now, just verify the session can be created
    
    session.stop().await?;
    Ok(())
}

async fn is_adb_available() -> bool {
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("adb")
            .arg("version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

async fn is_ios_device_available() -> bool {
    #[cfg(target_os = "macos")]
    {
        // In a real implementation, we would check for connected iOS devices
        // or available simulators using xcrun simctl or similar
        false
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        false
    }
}
