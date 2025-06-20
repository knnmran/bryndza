# Getting Started with Bryndza

Bryndza is a cross-platform UI automation library that provides a unified API for automating user interfaces across different platforms: Windows, macOS, Android, and iOS.

## Installation

Add Bryndza to your `Cargo.toml`:

```toml
[dependencies]
bryndza = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

Here's a simple example to get you started:

```rust
use bryndza::{Session, Locator, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new automation session
    let mut session = Session::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    // Start the session (connects to the platform)
    session.start().await?;

    // Find an element and interact with it
    let button = session.find_element(&Locator::id("my-button")).await?;
    button.click().await?;

    // Take a screenshot
    let screenshot = session.screenshot().await?;
    std::fs::write("screenshot.png", screenshot)?;

    // Stop the session
    session.stop().await?;
    Ok(())
}
```

## Core Concepts

### Session

A `Session` represents a connection to a platform's automation service. Each session manages the lifecycle of automation operations for a specific platform.

```rust
let session = Session::builder()
    .timeout(Duration::from_secs(30))
    .max_retries(3)
    .retry_delay(Duration::from_millis(500))
    .build()?;
```

### Locators

Locators define how to find elements in the UI. Bryndza supports various locator strategies:

```rust
// Find by ID
let locator = Locator::id("login-button");

// Find by text content
let locator = Locator::text("Click me");

// Find by class name
let locator = Locator::class_name("btn-primary");

// Find by XPath
let locator = Locator::xpath("//button[@type='submit']");

// Find by accessibility identifier
let locator = Locator::accessibility_id("login-btn");

// Combine locators
let locator = Locator::and(vec![
    Locator::class_name("button"),
    Locator::text("Submit"),
]);
```

### Elements

Elements represent UI components that you can interact with:

```rust
let element = session.find_element(&locator).await?;

// Basic interactions
element.click().await?;
element.double_click().await?;
element.type_text("Hello, World!").await?;
element.clear().await?;

// Get element properties
let text = element.text();
let bounds = element.bounds();
let is_visible = element.is_visible();
let is_enabled = element.is_enabled();

// Mobile-specific gestures
element.long_press(Duration::from_millis(1000)).await?;
element.swipe(SwipeDirection::Up, 300.0).await?;
```

### Wait Conditions

Bryndza provides powerful wait conditions for robust automation:

```rust
use bryndza::wait::WaitConditions;

// Wait for element to appear
let element = WaitConditions::element_visible(
    &session,
    &Locator::id("result"),
    Duration::from_secs(10),
).await?;

// Wait for specific text
let element = WaitConditions::element_text_contains(
    &session,
    &Locator::class_name("status"),
    "Success",
    Duration::from_secs(5),
).await?;

// Wait for element count
let elements = WaitConditions::element_count_at_least(
    &session,
    &Locator::class_name("list-item"),
    5,
    Duration::from_secs(10),
).await?;
```

## Platform-Specific Features

### Desktop (Windows/macOS)

Desktop platforms support mouse and keyboard interactions:

```rust
// Mouse interactions
element.click().await?;
element.double_click().await?;
element.right_click().await?;
element.hover().await?;

// Keyboard interactions
element.send_keys(&[
    Key::Control,
    Key::char('a'),
]).await?;
```

### Mobile (Android/iOS)

Mobile platforms support touch gestures:

```rust
// Touch gestures
element.tap().await?;
element.long_press(Duration::from_millis(1000)).await?;
element.swipe(SwipeDirection::Left, 200.0).await?;

// Multi-touch gestures
element.pinch(2.0).await?; // Zoom in
```

## Error Handling

Bryndza provides comprehensive error handling:

```rust
match session.find_element(&locator).await {
    Ok(element) => {
        // Element found
        element.click().await?;
    }
    Err(BryndzaError::ElementNotFound { locator }) => {
        println!("Element not found: {}", locator);
    }
    Err(BryndzaError::Timeout { duration, condition }) => {
        println!("Timeout after {:?}: {}", duration, condition);
    }
    Err(e) => {
        println!("Other error: {}", e);
    }
}
```

## Configuration

Customize Bryndza's behavior with configuration:

```rust
use bryndza::Config;

let config = Config {
    default_timeout: Duration::from_secs(15),
    max_retries: 5,
    retry_delay: Duration::from_millis(200),
    screenshot: ScreenshotConfig {
        auto_screenshot_on_failure: true,
        format: ImageFormat::PNG,
        quality: 90,
    },
    ..Default::default()
};

let session = Session::new(config)?;
```

## Best Practices

1. **Use explicit waits**: Always use wait conditions instead of hardcoded sleeps
2. **Prefer stable locators**: Use IDs and accessibility identifiers when possible
3. **Handle errors gracefully**: Implement proper error handling for robust automation
4. **Take screenshots on failures**: Enable automatic screenshot capture for debugging
5. **Keep sessions short-lived**: Create sessions for specific test scenarios

## Next Steps

- Check out the [Platform Guides](platform_guides/) for platform-specific details
- See [Examples](../examples/) for complete automation scenarios
- Read the [API Reference](api_reference.md) for detailed documentation
