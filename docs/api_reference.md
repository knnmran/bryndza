# Bryndza API Reference

## Core Modules

### Session

The `Session` struct is the main entry point for automation. It manages the connection to the automation backend and provides methods for finding and interacting with elements.

#### Methods

```rust
pub async fn new(config: Config) -> Result<Self, BryndzaError>
```

Creates a new automation session with the specified configuration.

```rust
pub async fn find_element(&self, locator: Locator) -> Result<Element, BryndzaError>
```

Finds a single element using the provided locator strategy.

```rust
pub async fn find_elements(&self, locator: Locator) -> Result<Vec<Element>, BryndzaError>
```

Finds multiple elements matching the locator.

```rust
pub async fn take_screenshot(&self) -> Result<Vec<u8>, BryndzaError>
```

Takes a screenshot of the current screen.

```rust
pub async fn close(self) -> Result<(), BryndzaError>
```

Closes the automation session and cleans up resources.

### Element

The `Element` struct represents a UI element that can be interacted with.

#### Methods

```rust
pub async fn click(&self) -> Result<(), BryndzaError>
```

Clicks on the element.

```rust
pub async fn send_keys(&self, text: &str) -> Result<(), BryndzaError>
```

Sends text input to the element.

```rust
pub async fn get_text(&self) -> Result<String, BryndzaError>
```

Gets the text content of the element.

```rust
pub async fn get_attribute(&self, name: &str) -> Result<Option<String>, BryndzaError>
```

Gets the value of an attribute.

```rust
pub async fn is_enabled(&self) -> Result<bool, BryndzaError>
```

Checks if the element is enabled.

```rust
pub async fn is_visible(&self) -> Result<bool, BryndzaError>
```

Checks if the element is visible.

```rust
pub async fn wait_until_clickable(&self, timeout: Duration) -> Result<(), BryndzaError>
```

Waits until the element becomes clickable.

### Locator

Enum for different element location strategies.

#### Variants

```rust
Id(String)          // Find by ID
ClassName(String)   // Find by class name
Name(String)        // Find by name attribute
XPath(String)       // Find by XPath expression
Css(String)         // Find by CSS selector
Text(String)        // Find by text content
Image(Vec<u8>)      // Find by image matching
```

### Config

Configuration struct for setting up automation sessions.

#### Fields

```rust
pub platform: Platform              // Target platform
pub app_path: Option<String>        // Path to application (for desktop)
pub device_id: Option<String>       // Device identifier (for mobile)
pub app_package: Option<String>     // App package name (Android)
pub bundle_id: Option<String>       // Bundle identifier (iOS)
pub timeout: Duration               // Default timeout for operations
pub implicit_wait: Duration         // Implicit wait time
pub screenshot_on_failure: bool     // Take screenshot on test failure
```

### Platform

Enum representing supported platforms.

#### Variants

```rust
Windows    // Windows desktop
MacOS      // macOS desktop
Android    // Android mobile
iOS        // iOS mobile
```

### Wait Conditions

Predefined wait conditions for common scenarios.

#### Functions

```rust
pub fn element_clickable(element: &Element) -> impl WaitCondition
```

Waits for element to be clickable.

```rust
pub fn element_visible(element: &Element) -> impl WaitCondition
```

Waits for element to be visible.

```rust
pub fn element_present(session: &Session, locator: Locator) -> impl WaitCondition
```

Waits for element to be present in DOM.

```rust
pub fn text_present(element: &Element, text: &str) -> impl WaitCondition
```

Waits for specific text to be present in element.

### Error Types

```rust
pub enum BryndzaError {
    ElementNotFound(String),
    InvalidLocator(String),
    Timeout(String),
    PlatformError(String),
    ConnectionError(String),
    SerializationError(String),
    IoError(std::io::Error),
}
```

## Platform-Specific Features

### Windows

- Native Windows API integration
- Support for Win32 and UWP applications
- Accessibility API integration

### macOS

- Core Foundation integration
- Support for Cocoa applications
- Accessibility Inspector compatibility

### Android

- ADB (Android Debug Bridge) integration
- UIAutomator2 backend
- Support for native and hybrid apps

### iOS

- XCTest framework integration
- WebDriverAgent support
- Simulator and real device support

## Examples

### Basic Usage

```rust
use bryndza::{Session, Config, Platform, Locator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        platform: Platform::Windows,
        app_path: Some("notepad.exe".to_string()),
        timeout: Duration::from_secs(30),
        ..Default::default()
    };

    let session = Session::new(config).await?;
    let element = session.find_element(Locator::ClassName("Edit".to_string())).await?;
    element.send_keys("Hello, Bryndza!").await?;

    session.close().await?;
    Ok(())
}
```

### Mobile Automation

```rust
use bryndza::{Session, Config, Platform, Locator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        platform: Platform::Android,
        app_package: Some("com.example.app".to_string()),
        device_id: Some("emulator-5554".to_string()),
        ..Default::default()
    };

    let session = Session::new(config).await?;
    let button = session.find_element(Locator::Id("login_button".to_string())).await?;
    button.click().await?;

    session.close().await?;
    Ok(())
}
```
