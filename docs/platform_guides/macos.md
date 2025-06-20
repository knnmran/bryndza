# macOS Platform Guide

This guide covers platform-specific details for automating macOS desktop applications using Bryndza.

## Prerequisites

- macOS 10.14 (Mojave) or later
- Accessibility permissions granted to your application
- Target applications must support macOS Accessibility API

## How It Works

Bryndza uses the macOS Accessibility API (AXUIElement) to interact with desktop applications. This provides:

- **Accessibility tree navigation**: Find elements based on accessibility attributes
- **Action-based interactions**: Perform actions through accessibility actions
- **Attribute inspection**: Access detailed element attributes
- **System event integration**: Generate system-level input events

## Accessibility Permissions

Before using Bryndza on macOS, you must grant accessibility permissions:

1. Open **System Preferences** > **Security & Privacy** > **Privacy**
2. Select **Accessibility** from the left sidebar
3. Click the lock to make changes
4. Add your application (Terminal, IDE, or your compiled binary)
5. Ensure the checkbox is checked

You can check permissions programmatically:

```rust
let config = Config {
    platform: PlatformConfig {
        macos: MacOSConfig {
            check_accessibility_permissions: true,
            ..Default::default()
        },
        ..Default::default()
    },
    ..Default::default()
};
```

## Supported Locators

### Accessibility Identifier

```rust
let locator = Locator::accessibility_id("save-button");
```

The most reliable locator for macOS applications. Set by developers using `accessibilityIdentifier`.

### Accessibility Label

```rust
let locator = Locator::text("Save");
```

Finds elements by their accessibility label or title.

### Role-based Selection

```rust
let locator = Locator::attribute("AXRole", "AXButton");
```

Finds elements by their accessibility role.

### Combined Attributes

```rust
let locator = Locator::and(vec![
    Locator::attribute("AXRole", "AXButton"),
    Locator::text("Save"),
]);
```

## Element Types (AX Roles)

macOS applications use these common accessibility roles:

- **AXButton**: Clickable buttons
- **AXTextField**: Text input fields
- **AXStaticText**: Non-editable text
- **AXPopUpButton**: Dropdown menus
- **AXCheckBox**: Toggle controls
- **AXRadioButton**: Single-choice options
- **AXList**: List containers
- **AXTable**: Table views
- **AXOutline**: Tree/outline views
- **AXTabGroup**: Tab controls
- **AXMenuItem**: Menu items
- **AXWindow**: Windows
- **AXApplication**: Applications

## Interactions

### Basic Actions

```rust
// Press action (for buttons)
element.click().await?;

// Confirm action (for default buttons)
element.confirm().await?;

// Cancel action
element.cancel().await?;
```

### Text Input

```rust
// Set value (for text fields)
element.type_text("Hello World").await?;
element.clear().await?;

// Get current value
let text = element.get_value()?;
```

### Selection and Navigation

```rust
// Pick action (for menu items, list items)
element.select().await?;

// Increment/Decrement (for steppers)
element.increment().await?;
element.decrement().await?;
```

### Window Management

```rust
// Raise window
window.raise().await?;

// Minimize window
window.minimize().await?;

// Close window
window.close().await?;
```

## Configuration

macOS-specific configuration options:

```rust
use bryndza::config::MacOSConfig;

let macos_config = MacOSConfig {
    check_accessibility_permissions: true,
    use_system_events: false, // Use AX actions vs system events
};
```

## Common Patterns

### Application Targeting

```rust
// Target specific application
let app = session.find_element(&Locator::attribute("AXRole", "AXApplication")).await?;

// Find application by bundle identifier
let finder = session.find_element(&Locator::attribute("AXBundleIdentifier", "com.apple.finder")).await?;
```

### Menu Bar Navigation

```rust
// Access menu bar
let menu_bar = session.find_element(&Locator::attribute("AXRole", "AXMenuBar")).await?;

// Click File menu
let file_menu = menu_bar.find_child(&Locator::text("File")).await?;
file_menu.click().await?;

// Click New item
let new_item = session.find_element(&Locator::text("New")).await?;
new_item.click().await?;
```

### Window Operations

```rust
// Find main window
let window = session.find_element(&Locator::and(vec![
    Locator::attribute("AXRole", "AXWindow"),
    Locator::attribute("AXMain", "true"),
])).await?;

// Get window title
let title = window.attribute("AXTitle")?;
println!("Window title: {}", title);

// Resize window
window.set_attribute("AXSize", "{800, 600}").await?;
```

### Form Handling

```rust
// Fill text field
let name_field = session.find_element(&Locator::accessibility_id("name-field")).await?;
name_field.click().await?;
name_field.type_text("John Doe").await?;

// Select from popup button (dropdown)
let country_popup = session.find_element(&Locator::accessibility_id("country-popup")).await?;
country_popup.click().await?;

let usa_option = session.find_element(&Locator::text("United States")).await?;
usa_option.click().await?;

// Check checkbox
let agree_checkbox = session.find_element(&Locator::accessibility_id("agree-checkbox")).await?;
if !agree_checkbox.is_checked()? {
    agree_checkbox.click().await?;
}
```

### Table and List Interaction

```rust
// Find table
let table = session.find_element(&Locator::attribute("AXRole", "AXTable")).await?;

// Get rows
let rows = table.find_children(&Locator::attribute("AXRole", "AXRow")).await?;
println!("Found {} rows", rows.len());

// Select specific row
let first_row = &rows[0];
first_row.select().await?;

// Get cell values
let cells = first_row.find_children(&Locator::attribute("AXRole", "AXCell")).await?;
for cell in cells {
    if let Some(value) = cell.attribute("AXValue") {
        println!("Cell value: {}", value);
    }
}
```

## Troubleshooting

### Common Issues

1. **Accessibility permissions denied**

   ```
   Error: macOS Accessibility error: Permission denied
   ```

   Solution: Grant accessibility permissions in System Preferences

2. **Elements not found**

   - Use Accessibility Inspector to examine the element tree
   - Check if the application properly implements accessibility
   - Verify attribute names and values

3. **Actions not working**
   - Ensure element supports the action (check AXActions)
   - Try alternative interaction methods
   - Check if element is enabled and visible

### Debugging Tools

- **Accessibility Inspector**: Built into Xcode, shows accessibility tree
- **Console.app**: View system logs for accessibility errors
- **Activity Monitor**: Check if assistive applications are running

### Performance Tips

1. Use accessibility identifiers when available
2. Cache element references for repeated operations
3. Use specific attribute combinations to narrow searches
4. Avoid deep tree traversals

## Example Applications

### TextEdit Automation

```rust
// Open TextEdit document
let mut session = Session::builder().build()?;
session.start().await?;

// Find text area
let text_area = session.find_element(&Locator::attribute("AXRole", "AXTextArea")).await?;
text_area.click().await?;
text_area.type_text("Hello from Bryndza on macOS!").await?;

// Save document
let file_menu = session.find_element(&Locator::text("File")).await?;
file_menu.click().await?;

let save_item = session.find_element(&Locator::text("Save…")).await?;
save_item.click().await?;

// Handle save dialog
let save_dialog = session.wait_for_element(&Locator::attribute("AXRole", "AXSheet")).await?;
let name_field = save_dialog.find_child(&Locator::attribute("AXRole", "AXTextField")).await?;
name_field.type_text("test-document").await?;

let save_button = save_dialog.find_child(&Locator::text("Save")).await?;
save_button.click().await?;
```

### Finder Automation

```rust
// Open new Finder window
let finder = session.find_element(&Locator::attribute("AXBundleIdentifier", "com.apple.finder")).await?;

// Navigate to Desktop
let sidebar = finder.find_child(&Locator::attribute("AXRole", "AXOutline")).await?;
let desktop_item = sidebar.find_child(&Locator::text("Desktop")).await?;
desktop_item.select().await?;

// Create new folder
let file_menu = session.find_element(&Locator::text("File")).await?;
file_menu.click().await?;

let new_folder_item = session.find_element(&Locator::text("New Folder")).await?;
new_folder_item.click().await?;

// Rename folder
let new_folder = session.wait_for_element(&Locator::and(vec![
    Locator::attribute("AXRole", "AXStaticText"),
    Locator::text("untitled folder"),
])).await?;

new_folder.double_click().await?;
new_folder.type_text("Bryndza Test Folder").await?;
```

### System Preferences Automation

```rust
// Open System Preferences
// Note: This requires additional permissions and careful handling

let system_prefs = session.find_element(&Locator::attribute("AXBundleIdentifier", "com.apple.preference")).await?;

// Navigate to specific preference pane
let search_field = system_prefs.find_child(&Locator::attribute("AXRole", "AXTextField")).await?;
search_field.click().await?;
search_field.type_text("Accessibility").await?;

// Click on Accessibility preference
let accessibility_pref = session.find_element(&Locator::text("Accessibility")).await?;
accessibility_pref.click().await?;
```

## Security and Privacy

- macOS requires explicit user consent for accessibility features
- Some system applications may have additional restrictions
- Consider sandboxing implications for App Store applications
- Be respectful of user privacy and system security
- Test automation may trigger security dialogs

## Best Practices

1. Always check accessibility permissions before starting
2. Use accessibility identifiers for reliable element location
3. Implement proper error handling for permission issues
4. Respect system security boundaries
5. Test on multiple macOS versions for compatibility
6. Use the Accessibility Inspector for development and debugging
