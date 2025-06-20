# Bryndza

A cross-platform automation library for desktop and mobile applications, written in Rust.

## Features

- **Cross-platform support**: Windows, macOS, Android, and iOS
- **Multiple locator strategies**: ID, class name, XPath, CSS selectors, text content, and image matching
- **Async/await support**: Built with modern Rust async patterns
- **Type-safe API**: Leverages Rust's type system for reliable automation code
- **Image-based element location**: Advanced computer vision for UI element detection
- **Flexible wait conditions**: Customizable waiting strategies for dynamic UIs
- **Screenshot capabilities**: Built-in screenshot functionality for debugging and reporting

## Quick Start

Add Bryndza to your `Cargo.toml`:

```toml
[dependencies]
bryndza = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Desktop Automation (Windows/macOS)

```rust
use bryndza::{Session, Config, Platform, Locator};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        platform: Platform::Windows,
        app_path: Some("notepad.exe".to_string()),
        timeout: Duration::from_secs(30),
        ..Default::default()
    };

    let session = Session::new(config).await?;

    // Find a text input and type some text
    let text_field = session.find_element(Locator::ClassName("Edit".to_string())).await?;
    text_field.send_keys("Hello from Bryndza!").await?;

    // Take a screenshot
    let screenshot = session.take_screenshot().await?;
    std::fs::write("screenshot.png", screenshot)?;

    session.close().await?;
    Ok(())
}
```

### Mobile Automation (Android/iOS)

```rust
use bryndza::{Session, Config, Platform, Locator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        platform: Platform::Android,
        app_package: Some("com.example.myapp".to_string()),
        device_id: Some("emulator-5554".to_string()),
        ..Default::default()
    };

    let session = Session::new(config).await?;

    // Find and tap a button
    let login_button = session.find_element(Locator::Id("login_btn".to_string())).await?;
    login_button.click().await?;

    // Wait for an element to appear
    let welcome_text = session.find_element(Locator::Text("Welcome".to_string())).await?;
    welcome_text.wait_until_visible(Duration::from_secs(10)).await?;

    session.close().await?;
    Ok(())
}
```

## Supported Platforms

### Desktop

- **Windows**: Win32 and UWP applications via Windows API and Accessibility framework
- **macOS**: Cocoa applications via Core Foundation and Accessibility APIs

### Mobile

- **Android**: Native and hybrid apps via ADB and UIAutomator2
- **iOS**: Native apps via XCTest framework and WebDriverAgent

## Element Location Strategies

```rust
use bryndza::Locator;

// By ID
let element = session.find_element(Locator::Id("my-button".to_string())).await?;

// By class name
let element = session.find_element(Locator::ClassName("btn".to_string())).await?;

// By XPath
let element = session.find_element(Locator::XPath("//button[@id='submit']".to_string())).await?;

// By CSS selector
let element = session.find_element(Locator::Css(".submit-button".to_string())).await?;

// By text content
let element = session.find_element(Locator::Text("Submit".to_string())).await?;

// By image (computer vision)
let image_data = std::fs::read("button_image.png")?;
let element = session.find_element(Locator::Image(image_data)).await?;
```

## Wait Conditions

```rust
use bryndza::wait::conditions;
use std::time::Duration;

// Wait for element to be clickable
element.wait_until_clickable(Duration::from_secs(10)).await?;

// Wait for element to be visible
element.wait_until_visible(Duration::from_secs(5)).await?;

// Wait for specific text to appear
let condition = conditions::text_present(&element, "Loading complete");
session.wait_for(condition, Duration::from_secs(30)).await?;
```

## Configuration

```rust
use bryndza::{Config, Platform};
use std::time::Duration;

let config = Config {
    platform: Platform::Windows,
    app_path: Some("C:\\Program Files\\MyApp\\app.exe".to_string()),
    timeout: Duration::from_secs(30),
    implicit_wait: Duration::from_millis(500),
    screenshot_on_failure: true,
    ..Default::default()
};
```

## Platform-Specific Configuration

### Windows

```rust
let config = Config {
    platform: Platform::Windows,
    app_path: Some("notepad.exe".to_string()),
    // Windows-specific options
    ..Default::default()
};
```

### Android

```rust
let config = Config {
    platform: Platform::Android,
    app_package: Some("com.android.calculator2".to_string()),
    device_id: Some("emulator-5554".to_string()),
    // Android-specific options
    ..Default::default()
};
```

### iOS

```rust
let config = Config {
    platform: Platform::iOS,
    bundle_id: Some("com.apple.calculator".to_string()),
    device_id: Some("iPhone-12-Simulator".to_string()),
    // iOS-specific options
    ..Default::default()
};
```

## Building from Source

```bash
git clone https://github.com/knnmran/bryndza.git
cd bryndza
cargo build --release
```

### Platform-specific builds

```bash
# Windows features
cargo build --features windows

# macOS features
cargo build --features macos

# Android features
cargo build --features android

# iOS features
cargo build --features ios
```

## Testing

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration

# Run benchmarks
cargo bench
```

## Examples

The `examples/` directory contains complete examples:

- `basic_desktop.rs` - Desktop automation basics
- `mobile_app.rs` - Mobile app automation
- `green_wallet_test.rs` - Real-world app testing example

Run examples with:

```bash
cargo run --example basic_desktop
```

## Documentation

- [Getting Started Guide](docs/getting_started.md)
- [Platform-Specific Guides](docs/platform_guides/)
- [API Reference](docs/api_reference.md)

## Contributing

Contributions are welcome! Please see our contributing guidelines for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Web browser automation support
- [ ] Advanced image recognition algorithms
- [ ] Performance optimization for mobile platforms
- [ ] Visual test reporting
- [ ] CI/CD integration helpers
- [ ] Record and replay functionality
