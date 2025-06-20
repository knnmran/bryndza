# Windows Platform Guide

This guide covers platform-specific details for automating Windows desktop applications using Bryndza.

## Prerequisites

- Windows 10 or later
- UI Automation must be enabled (usually enabled by default)
- Your application must support UI Automation (most modern Windows apps do)

## How It Works

Bryndza uses the Windows UI Automation API to interact with desktop applications. This provides:

- **Accessibility tree navigation**: Find elements based on their accessibility properties
- **Pattern-based interactions**: Use UI Automation patterns for reliable interactions
- **Event handling**: Respond to UI changes and events
- **Property inspection**: Access detailed element properties

## Supported Locators

### AutomationId

```rust
let locator = Locator::id("SaveButton");
```

The AutomationId is the most reliable locator for Windows applications. It's set by developers and should remain stable across application updates.

### Name (Text Content)

```rust
let locator = Locator::text("Save");
```

Finds elements by their displayed text or accessible name.

### ClassName

```rust
let locator = Locator::class_name("Button");
```

Finds elements by their Windows class name (e.g., "Button", "Edit", "ComboBox").

### XPath-like Expressions

```rust
let locator = Locator::xpath("//Button[@Name='Save']");
```

Note: This is a simplified XPath implementation for UI Automation trees.

## Element Types

Windows applications typically contain these element types:

- **Button**: Clickable buttons
- **Edit**: Text input fields
- **ComboBox**: Dropdown selections
- **CheckBox**: Toggle controls
- **RadioButton**: Single-choice options
- **ListItem**: Items in lists
- **TabItem**: Tab controls
- **MenuItem**: Menu items
- **TreeItem**: Tree view items

## Interactions

### Click Patterns

```rust
// Standard click
element.click().await?;

// Invoke pattern (for buttons)
element.invoke().await?;

// Toggle pattern (for checkboxes)
element.toggle().await?;
```

### Text Input

```rust
// Value pattern (for text fields)
element.type_text("Hello World").await?;
element.clear().await?;

// Get current value
let text = element.get_value()?;
```

### Selection

```rust
// Selection pattern (for list items)
element.select().await?;

// SelectionItem pattern
element.add_to_selection().await?;
element.remove_from_selection().await?;
```

### Scrolling

```rust
// Scroll pattern
element.scroll_into_view().await?;
element.scroll(ScrollDirection::Down, 100).await?;
```

## Configuration

Windows-specific configuration options:

```rust
use bryndza::config::WindowsConfig;

let windows_config = WindowsConfig {
    optimize_tree_traversal: true,
    enable_element_caching: true,
};
```

## Common Patterns

### Finding Window Elements

```rust
// Find main application window
let window = session.find_element(&Locator::class_name("ApplicationFrameWindow")).await?;

// Find specific dialog
let dialog = session.find_element(&Locator::and(vec![
    Locator::class_name("Dialog"),
    Locator::text("Save As"),
])).await?;
```

### Menu Navigation

```rust
// Open File menu
let file_menu = session.find_element(&Locator::text("File")).await?;
file_menu.click().await?;

// Click New item
let new_item = session.find_element(&Locator::text("New")).await?;
new_item.click().await?;
```

### Form Filling

```rust
// Fill out a form
let name_field = session.find_element(&Locator::id("NameField")).await?;
name_field.click().await?;
name_field.clear().await?;
name_field.type_text("John Doe").await?;

let email_field = session.find_element(&Locator::id("EmailField")).await?;
email_field.click().await?;
email_field.type_text("john@example.com").await?;

let submit_button = session.find_element(&Locator::text("Submit")).await?;
submit_button.click().await?;
```

### Dialog Handling

```rust
// Wait for dialog to appear
let dialog = session.wait_for_element(&Locator::class_name("Dialog")).await?;

// Handle different dialog types
match dialog.attribute("Name") {
    Some("Save As") => {
        // Handle save dialog
        let filename_field = dialog.find_child(&Locator::class_name("Edit")).await?;
        filename_field.type_text("document.txt").await?;

        let save_button = dialog.find_child(&Locator::text("Save")).await?;
        save_button.click().await?;
    }
    Some("Confirm") => {
        // Handle confirmation dialog
        let yes_button = dialog.find_child(&Locator::text("Yes")).await?;
        yes_button.click().await?;
    }
    _ => {
        // Handle unknown dialog
        let ok_button = dialog.find_child(&Locator::text("OK")).await?;
        ok_button.click().await?;
    }
}
```

## Troubleshooting

### Common Issues

1. **Elements not found**

   - Use tools like Inspect.exe or UI Automation Verify to examine the UI tree
   - Check if the application supports UI Automation
   - Verify the AutomationId or Name properties

2. **Slow element finding**

   - Enable element caching in configuration
   - Use more specific locators (AutomationId > ClassName > Name)
   - Implement proper wait conditions

3. **Click not working**
   - Ensure element is visible and enabled
   - Try using Invoke pattern instead of Click
   - Check if element is covered by another element

### Debugging Tools

- **Inspect.exe**: Built into Windows SDK, shows UI Automation tree
- **UI Automation Verify**: Microsoft tool for testing UI Automation
- **Accessibility Insights**: Microsoft tool for accessibility testing

### Performance Tips

1. Use AutomationId when available
2. Enable element caching for repeated operations
3. Use scoped searches (search within parent elements)
4. Avoid deep XPath expressions

## Example Applications

### Notepad Automation

```rust
// Open Notepad
let mut session = Session::builder().build()?;
session.start().await?;

// Type some text
let edit_area = session.find_element(&Locator::class_name("Edit")).await?;
edit_area.click().await?;
edit_area.type_text("Hello from Bryndza!").await?;

// Save file
let file_menu = session.find_element(&Locator::text("File")).await?;
file_menu.click().await?;

let save_item = session.find_element(&Locator::text("Save")).await?;
save_item.click().await?;
```

### Calculator Automation

```rust
// Click calculator buttons
let button_1 = session.find_element(&Locator::text("1")).await?;
button_1.click().await?;

let button_plus = session.find_element(&Locator::text("+")).await?;
button_plus.click().await?;

let button_2 = session.find_element(&Locator::text("2")).await?;
button_2.click().await?;

let button_equals = session.find_element(&Locator::text("=")).await?;
button_equals.click().await?;

// Get result
let result = session.find_element(&Locator::id("CalculatorResults")).await?;
let result_text = result.text().unwrap_or("");
println!("Result: {}", result_text);
```

## Security Considerations

- UI Automation requires appropriate permissions
- Some applications may restrict automation for security reasons
- Consider running with appropriate user privileges
- Be aware of UAC (User Account Control) dialogs
