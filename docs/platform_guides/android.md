# Android Platform Guide

This guide covers platform-specific details for automating Android applications using Bryndza.

## Prerequisites

- Android SDK installed and `adb` in PATH
- Android device or emulator with USB debugging enabled
- Target application installed on the device
- Android 4.4 (API level 19) or later

## How It Works

Bryndza uses Android Debug Bridge (ADB) and UI Automator to interact with Android applications:

- **ADB**: Communication with Android devices and emulators
- **UI Automator**: Android's built-in UI testing framework
- **Input commands**: Touch, swipe, and keyboard input via ADB shell
- **Accessibility services**: Access to UI element hierarchy

## Device Setup

### Enable Developer Options

1. Go to **Settings** > **About phone**
2. Tap **Build number** 7 times
3. Go back to **Settings** > **Developer options**
4. Enable **USB debugging**

### Connect Device

```bash
# Check connected devices
adb devices

# Connect via WiFi (optional)
adb tcpip 5555
adb connect <device_ip>:5555
```

### Verify Connection

```rust
// Bryndza will automatically detect and connect to available devices
let mut session = Session::builder().build()?;
session.start().await?; // Will fail if no devices found
```

## Supported Locators

### Resource ID

```rust
let locator = Locator::id("com.example.app:id/login_button");
```

The most reliable locator for Android. Format: `package:id/resource_name`

### Text Content

```rust
let locator = Locator::text("Sign In");
```

Finds elements by their displayed text.

### Content Description

```rust
let locator = Locator::accessibility_id("Login button");
```

Uses the `contentDescription` attribute.

### Class Name

```rust
let locator = Locator::class_name("android.widget.Button");
```

Finds elements by their Android widget class.

### XPath (UI Automator)

```rust
let locator = Locator::xpath("//android.widget.Button[@text='Sign In']");
```

Note: This uses UI Automator's XPath implementation.

## Element Types

Common Android UI elements:

- **android.widget.Button**: Buttons
- **android.widget.EditText**: Text input fields
- **android.widget.TextView**: Text display
- **android.widget.ImageView**: Images
- **android.widget.CheckBox**: Checkboxes
- **android.widget.RadioButton**: Radio buttons
- **android.widget.Spinner**: Dropdowns
- **android.widget.ListView**: Lists
- **android.widget.RecyclerView**: Recycler views
- **android.widget.ScrollView**: Scrollable containers
- **android.support.v7.widget.RecyclerView**: Support library RecyclerView

## Touch Interactions

### Basic Touch

```rust
// Tap
element.click().await?;

// Long press
element.long_press(Duration::from_millis(1000)).await?;

// Double tap
element.double_click().await?;
```

### Swipe Gestures

```rust
use bryndza::element::SwipeDirection;

// Swipe on element
element.swipe(SwipeDirection::Up, 300.0).await?;
element.swipe(SwipeDirection::Left, 200.0).await?;

// Screen swipes
session.swipe_screen(100, 500, 100, 200).await?;
```

### Text Input

```rust
// Type text
element.click().await?; // Focus first
element.type_text("Hello Android").await?;

// Clear text
element.clear().await?;

// Replace text
element.clear().await?;
element.type_text("New text").await?;
```

## Configuration

Android-specific configuration options:

```rust
use bryndza::config::AndroidConfig;

let android_config = AndroidConfig {
    adb_timeout: Duration::from_secs(30),
    device_serial: Some("emulator-5554".to_string()), // Specific device
    enable_ui_automator_optimizations: true,
};
```

## Common Patterns

### App Lifecycle Management

```rust
// Launch app
session.launch_app("com.example.myapp").await?;

// Check if app is running
let is_running = session.is_app_running("com.example.myapp").await?;

// Close app
session.close_app("com.example.myapp").await?;
```

### Login Flow

```rust
// Find and fill username
let username_field = session.find_element(&Locator::id("com.example.app:id/username")).await?;
username_field.click().await?;
username_field.clear().await?;
username_field.type_text("testuser").await?;

// Find and fill password
let password_field = session.find_element(&Locator::id("com.example.app:id/password")).await?;
password_field.click().await?;
password_field.type_text("password123").await?;

// Click login button
let login_button = session.find_element(&Locator::text("Login")).await?;
login_button.click().await?;

// Wait for home screen
let home_indicator = session.wait_for_element(
    &Locator::id("com.example.app:id/home_title")
).await?;
```

### List and RecyclerView Interaction

```rust
// Find list
let recycler_view = session.find_element(&Locator::class_name("android.support.v7.widget.RecyclerView")).await?;

// Scroll to find item
let target_item = loop {
    match session.find_element(&Locator::text("Target Item")).await {
        Ok(item) => break item,
        Err(_) => {
            // Scroll down and try again
            recycler_view.swipe(SwipeDirection::Up, 300.0).await?;
        }
    }
};

target_item.click().await?;
```

### Navigation

```rust
// Back button
session.press_back().await?;

// Home button
session.press_home().await?;

// Recent apps
session.press_recent_apps().await?;

// Menu button
session.press_menu().await?;
```

### Notifications

```rust
// Open notification panel
session.open_notifications().await?;

// Find specific notification
let notification = session.find_element(&Locator::text("New message")).await?;
notification.click().await?;

// Clear notifications
session.clear_notifications().await?;
```

### System Dialogs

```rust
// Handle permission dialog
if let Ok(allow_button) = session.find_element(&Locator::text("Allow")).await {
    allow_button.click().await?;
}

// Handle system alerts
if let Ok(ok_button) = session.find_element(&Locator::text("OK")).await {
    ok_button.click().await?;
}
```

## Device Capabilities

### Screen Operations

```rust
// Get screen size
let (width, height) = session.get_screen_size().await?;

// Take screenshot
let screenshot = session.screenshot().await?;
std::fs::write("android_screenshot.png", screenshot)?;

// Screen orientation
session.set_orientation(Orientation::Landscape).await?;
```

### Device Interaction

```rust
// Volume controls
session.press_volume_up().await?;
session.press_volume_down().await?;

// Power button
session.press_power().await?;

// Wake device
session.wake_device().await?;

// Check if screen is on
let is_awake = session.is_screen_on().await?;
```

## Performance Tips

1. **Use resource IDs when available** - most reliable and fastest
2. **Minimize UI Automator dumps** - expensive operation
3. **Use partial text matching** for dynamic content
4. **Cache element references** for repeated operations
5. **Use specific locators** to avoid scanning entire hierarchy

## Troubleshooting

### Common Issues

1. **No devices found**

   ```
   Error: No Android devices found
   ```

   - Check `adb devices`
   - Enable USB debugging
   - Install proper USB drivers

2. **Permission denied**

   ```
   Error: ADB command failed - permission denied
   ```

   - Check device authorization
   - Try `adb kill-server && adb start-server`

3. **Element not found**

   - Use `adb shell uiautomator dump` to examine UI hierarchy
   - Check if element is visible and not covered
   - Wait for animations to complete

4. **Slow automation**
   - Reduce wait timeouts for testing
   - Use more specific locators
   - Enable UI Automator optimizations

### Debugging Tools

```bash
# UI hierarchy dump
adb shell uiautomator dump /sdcard/ui.xml
adb pull /sdcard/ui.xml

# View current activity
adb shell dumpsys activity activities | grep mResumedActivity

# App info
adb shell pm list packages | grep myapp
adb shell pm dump com.example.myapp

# Device logs
adb logcat | grep MyApp
```

## Example Applications

### Gmail Automation

```rust
// Open Gmail
session.launch_app("com.google.android.gm").await?;

// Compose new email
let compose_button = session.find_element(&Locator::accessibility_id("Compose")).await?;
compose_button.click().await?;

// Fill recipient
let to_field = session.find_element(&Locator::text("To")).await?;
to_field.click().await?;
to_field.type_text("test@example.com").await?;

// Fill subject
let subject_field = session.find_element(&Locator::text("Subject")).await?;
subject_field.click().await?;
subject_field.type_text("Test Email").await?;

// Fill body
let body_field = session.find_element(&Locator::text("Compose email")).await?;
body_field.click().await?;
body_field.type_text("Hello from Bryndza!").await?;

// Send (but cancel for testing)
let send_button = session.find_element(&Locator::accessibility_id("Send")).await?;
// send_button.click().await?; // Uncomment to actually send

// Navigate back instead
session.press_back().await?;
```

### Settings Navigation

```rust
// Open Settings
session.launch_app("com.android.settings").await?;

// Navigate to WiFi settings
let wifi_item = session.find_element(&Locator::text("Wi-Fi")).await?;
wifi_item.click().await?;

// Find available networks
let networks = session.find_elements(&Locator::class_name("android.widget.TextView")).await?;
for network in networks {
    if let Some(ssid) = network.text() {
        if ssid.contains("MyNetwork") {
            network.click().await?;
            break;
        }
    }
}
```

## Security Considerations

- ADB provides powerful system access
- Only test on development/test devices
- Be cautious with system apps and sensitive data
- Consider security implications of USB debugging
- Some apps may detect and block automation

## Best Practices

1. Use resource IDs for reliable element identification
2. Implement proper wait conditions for dynamic content
3. Handle system dialogs and permissions gracefully
4. Test on multiple device sizes and Android versions
5. Keep automation scripts maintainable and readable
6. Use page object pattern for complex applications
