# iOS Platform Guide

This guide covers platform-specific details for automating iOS applications using Bryndza.

## Prerequisites

- macOS development machine
- Xcode installed with iOS SDK
- iOS device or iOS Simulator
- Target application installed on device/simulator
- iOS 9.0 or later

## How It Works

Bryndza uses the XCTest framework and iOS testing infrastructure to interact with iOS applications:

- **XCTest**: Apple's testing framework for iOS automation
- **XCUITest**: UI testing component of XCTest
- **WebDriverAgent**: Facebook's WebDriver implementation for iOS
- **Accessibility**: iOS accessibility features for element identification

## Device Setup

### Physical Device Setup

1. Enable **Developer Mode** in Settings > Privacy & Security
2. Connect device via USB
3. Trust the development machine when prompted
4. Install the target application

### iOS Simulator Setup

```bash
# List available simulators
xcrun simctl list devices

# Boot simulator
xcrun simctl boot "iPhone 14"

# Install app on simulator
xcrun simctl install booted path/to/app.app
```

### Xcode Configuration

1. Open Xcode
2. Add your Apple ID in Preferences > Accounts
3. Select appropriate development team
4. Ensure proper provisioning profiles are available

## Supported Locators

### Accessibility Identifier

```rust
let locator = Locator::accessibility_id("login-button");
```

The most reliable locator for iOS. Set using `accessibilityIdentifier`.

### Accessibility Label

```rust
let locator = Locator::text("Sign In");
```

Uses the `accessibilityLabel` property.

### Element Type

```rust
let locator = Locator::class_name("XCUIElementTypeButton");
```

Finds elements by their XCUIElementType.

### Predicate-based Locators

```rust
let locator = Locator::predicate("label CONTAINS 'Welcome'");
```

Uses NSPredicate syntax for complex queries.

## Element Types

Common iOS UI element types:

- **XCUIElementTypeButton**: Buttons
- **XCUIElementTypeTextField**: Text input fields
- **XCUIElementTypeSecureTextField**: Password fields
- **XCUIElementTypeStaticText**: Labels and text
- **XCUIElementTypeImage**: Images
- **XCUIElementTypeSwitch**: Toggle switches
- **XCUIElementTypeSlider**: Sliders
- **XCUIElementTypeNavigationBar**: Navigation bars
- **XCUIElementTypeTabBar**: Tab bars
- **XCUIElementTypeTable**: Table views
- **XCUIElementTypeCollectionView**: Collection views
- **XCUIElementTypeCell**: Table/collection cells
- **XCUIElementTypeAlert**: Alert dialogs
- **XCUIElementTypeSheet**: Action sheets

## Touch Interactions

### Basic Touch

```rust
// Tap
element.click().await?;

// Double tap
element.double_tap().await?;

// Long press
element.long_press(Duration::from_millis(1000)).await?;

// Force touch (3D Touch)
element.force_touch().await?;
```

### Swipe Gestures

```rust
use bryndza::element::SwipeDirection;

// Swipe on element
element.swipe(SwipeDirection::Up, 300.0).await?;
element.swipe(SwipeDirection::Left, 200.0).await?;

// Swipe with velocity
element.swipe_with_velocity(SwipeDirection::Down, 0.5).await?;
```

### Multi-touch Gestures

```rust
// Pinch to zoom
element.pinch(2.0, 1.0).await?; // scale, velocity

// Rotation
element.rotate(45.0, 1.0).await?; // rotation, velocity

// Two-finger tap
element.two_finger_tap().await?;
```

### Text Input

```rust
// Type text
element.click().await?; // Focus first
element.type_text("Hello iOS").await?;

// Clear and type
element.clear_and_type("New text").await?;

// Use system keyboard
session.use_keyboard().type_text("System input").await?;
```

## Configuration

iOS-specific configuration options:

```rust
use bryndza::config::IOSConfig;

let ios_config = IOSConfig {
    xctest_bundle_id: Some("com.example.TestRunner".to_string()),
    device_udid: Some("00008030-0000000000000000".to_string()),
    enable_xctest_optimizations: true,
};
```

## Common Patterns

### App Lifecycle

```rust
// Launch app
session.launch_app("com.example.MyApp").await?;

// Activate app (bring to foreground)
session.activate_app("com.example.MyApp").await?;

// Terminate app
session.terminate_app("com.example.MyApp").await?;

// Check app state
let state = session.app_state("com.example.MyApp").await?;
```

### Navigation

```rust
// Navigation bar interaction
let nav_bar = session.find_element(&Locator::class_name("XCUIElementTypeNavigationBar")).await?;
let back_button = nav_bar.find_child(&Locator::text("Back")).await?;
back_button.click().await?;

// Tab bar navigation
let tab_bar = session.find_element(&Locator::class_name("XCUIElementTypeTabBar")).await?;
let settings_tab = tab_bar.find_child(&Locator::text("Settings")).await?;
settings_tab.click().await?;
```

### Table View Interaction

```rust
// Find table
let table = session.find_element(&Locator::class_name("XCUIElementTypeTable")).await?;

// Scroll to find cell
let target_cell = loop {
    match table.find_child(&Locator::text("Target Cell")).await {
        Ok(cell) => break cell,
        Err(_) => {
            table.swipe(SwipeDirection::Up, 300.0).await?;
        }
    }
};

target_cell.click().await?;

// Swipe actions (delete, etc.)
target_cell.swipe(SwipeDirection::Left, 100.0).await?;
let delete_button = session.find_element(&Locator::text("Delete")).await?;
delete_button.click().await?;
```

### Form Interaction

```rust
// Text field
let email_field = session.find_element(&Locator::accessibility_id("email-field")).await?;
email_field.click().await?;
email_field.clear_and_type("test@example.com").await?;

// Secure text field
let password_field = session.find_element(&Locator::class_name("XCUIElementTypeSecureTextField")).await?;
password_field.click().await?;
password_field.type_text("password123").await?;

// Switch/toggle
let notifications_switch = session.find_element(&Locator::accessibility_id("notifications-switch")).await?;
if !notifications_switch.is_selected()? {
    notifications_switch.click().await?;
}

// Slider
let volume_slider = session.find_element(&Locator::accessibility_id("volume-slider")).await?;
volume_slider.set_slider_value(0.8).await?;
```

### Alert Handling

```rust
// Wait for alert
let alert = session.wait_for_element(&Locator::class_name("XCUIElementTypeAlert")).await?;

// Get alert text
let alert_text = alert.find_child(&Locator::class_name("XCUIElementTypeStaticText")).await?;
let message = alert_text.text().unwrap_or("");

// Handle alert buttons
match message {
    msg if msg.contains("Permission") => {
        let allow_button = alert.find_child(&Locator::text("Allow")).await?;
        allow_button.click().await?;
    }
    msg if msg.contains("Confirm") => {
        let ok_button = alert.find_child(&Locator::text("OK")).await?;
        ok_button.click().await?;
    }
    _ => {
        let cancel_button = alert.find_child(&Locator::text("Cancel")).await?;
        cancel_button.click().await?;
    }
}
```

### System Interactions

```rust
// Home button (iPhone with home button)
session.press_home().await?;

// Control Center
session.open_control_center().await?;

// Notification Center
session.open_notification_center().await?;

// App Switcher
session.open_app_switcher().await?;

// Siri
session.activate_siri().await?;
```

## Device Capabilities

### Screen Operations

```rust
// Take screenshot
let screenshot = session.screenshot().await?;
std::fs::write("ios_screenshot.png", screenshot)?;

// Device orientation
session.set_orientation(DeviceOrientation::Landscape).await?;
let orientation = session.get_orientation().await?;

// Screen dimensions
let (width, height) = session.get_screen_size().await?;
```

### Hardware Simulation

```rust
// Shake gesture
session.shake_device().await?;

// Lock device
session.lock_device().await?;

// Unlock device
session.unlock_device().await?;

// Simulate memory pressure
session.simulate_memory_pressure().await?;
```

## Performance Testing

### Launch Time

```rust
use std::time::Instant;

let start = Instant::now();
session.launch_app("com.example.MyApp").await?;

// Wait for specific element to ensure app is loaded
session.wait_for_element(&Locator::accessibility_id("home-screen")).await?;

let launch_time = start.elapsed();
println!("App launch time: {:?}", launch_time);
```

### Animation Handling

```rust
// Disable animations for faster testing
session.set_animation_speed(0.0).await?;

// Wait for animations to complete
session.wait_for_animations_idle().await?;
```

## Troubleshooting

### Common Issues

1. **Developer mode not enabled**

   ```
   Error: Developer mode is not enabled
   ```

   Enable in Settings > Privacy & Security > Developer Mode

2. **App not installed**

   ```
   Error: App with bundle ID not found
   ```

   Install app on device/simulator first

3. **Element not found**

   - Use Xcode's Accessibility Inspector
   - Check if element has accessibility properties
   - Verify element is not covered by other elements

4. **Permission dialogs**
   - Handle system permission alerts
   - Grant necessary permissions in Settings
   - Use proper entitlements in test runner

### Debugging Tools

- **Accessibility Inspector**: Built into Xcode
- **Console.app**: View device logs
- **Xcode Debugger**: Step through test execution
- **Instruments**: Performance analysis

### Performance Tips

1. Use accessibility identifiers for reliable element finding
2. Disable animations during testing
3. Use specific locators to reduce search time
4. Cache element references for repeated operations
5. Implement proper wait conditions

## Example Applications

### iOS Settings Automation

```rust
// Open Settings app
session.launch_app("com.apple.Preferences").await?;

// Navigate to Wi-Fi settings
let wifi_cell = session.find_element(&Locator::text("Wi-Fi")).await?;
wifi_cell.click().await?;

// Toggle Wi-Fi switch
let wifi_switch = session.find_element(&Locator::class_name("XCUIElementTypeSwitch")).await?;
wifi_switch.click().await?;

// Go back
let back_button = session.find_element(&Locator::text("Settings")).await?;
back_button.click().await?;
```

### Safari Automation

```rust
// Open Safari
session.launch_app("com.apple.mobilesafari").await?;

// Tap address bar
let address_bar = session.find_element(&Locator::accessibility_id("URL")).await?;
address_bar.click().await?;

// Type URL
address_bar.type_text("https://example.com").await?;

// Tap Go
let go_button = session.find_element(&Locator::text("Go")).await?;
go_button.click().await?;

// Wait for page load
session.wait_for_element(&Locator::text("Example Domain")).await?;
```

### Custom App Testing

```rust
// Login flow
let username_field = session.find_element(&Locator::accessibility_id("username")).await?;
username_field.click().await?;
username_field.type_text("testuser").await?;

let password_field = session.find_element(&Locator::accessibility_id("password")).await?;
password_field.click().await?;
password_field.type_text("password123").await?;

let login_button = session.find_element(&Locator::accessibility_id("login-button")).await?;
login_button.click().await?;

// Wait for home screen
let home_indicator = session.wait_for_element(&Locator::accessibility_id("home-screen")).await?;
assert!(home_indicator.is_displayed());
```

## Security and Privacy

- iOS apps run in sandboxed environments
- System apps may have additional restrictions
- Handle privacy permission dialogs appropriately
- Be aware of App Store guidelines if testing distributed apps
- Consider impact on user data and privacy

## Best Practices

1. Use accessibility identifiers for reliable automation
2. Handle system dialogs and permissions gracefully
3. Implement proper wait strategies for network operations
4. Test on multiple iOS versions and device sizes
5. Use page object pattern for maintainable tests
6. Disable animations for consistent timing
7. Test both portrait and landscape orientations
8. Handle background/foreground app state changes
