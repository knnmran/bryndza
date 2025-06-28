use crate::{
    config::Config,
    element::{Element, Locator},
    error::Result,
};

#[cfg(target_os = "macos")]
use std::{
    ffi::CString,
    os::raw::{c_char, c_void},
    ptr,
};

#[cfg(target_os = "macos")]
mod ax_bindings {
    use super::*;

    // Core Foundation types
    pub type CFTypeRef = *const c_void;
    pub type CFStringRef = CFTypeRef;
    pub type CFArrayRef = CFTypeRef;
    pub type CFIndex = isize;
    pub type Boolean = u8;

    // Accessibility types
    pub type AXUIElementRef = CFTypeRef;
    pub type AXError = i32;

    // AX Error codes
    pub const K_AX_ERROR_SUCCESS: AXError = 0;
    pub const K_AX_ERROR_ACCESSIBILITY_NOT_ENABLED: AXError = -25200;

    #[link(name = "ApplicationServices", kind = "framework")]
    unsafe extern "C" {
        // Accessibility functions
        pub fn AXUIElementCreateSystemWide() -> AXUIElementRef;
        pub fn AXUIElementCreateApplication(pid: i32) -> AXUIElementRef;
        pub fn AXUIElementCopyAttributeNames(
            element: AXUIElementRef,
            names: *mut CFArrayRef,
        ) -> AXError;
        pub fn AXUIElementCopyAttributeValue(
            element: AXUIElementRef,
            attribute: CFStringRef,
            value: *mut CFTypeRef,
        ) -> AXError;
        pub fn AXUIElementPerformAction(element: AXUIElementRef, action: CFStringRef) -> AXError;
        pub fn AXIsProcessTrusted() -> Boolean;

        // Core Foundation functions
        pub fn CFStringCreateWithCString(
            alloc: CFTypeRef,
            cstr: *const c_char,
            encoding: u32,
        ) -> CFStringRef;
        pub fn CFRelease(cf: CFTypeRef);
        pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;

        // CFString functions
        pub fn CFStringGetLength(string: CFStringRef) -> i64;
        pub fn CFStringGetCString(
            string: CFStringRef,
            buffer: *mut i8,
            buffer_size: i64,
            encoding: u32,
        ) -> i32;
        pub fn CFArrayGetCount(array: CFArrayRef) -> CFIndex;
        pub fn CFArrayGetValueAtIndex(array: CFArrayRef, idx: CFIndex) -> CFTypeRef;
    }

    // String encoding
    pub const K_CF_STRING_ENCODING_UTF8: u32 = 0x08000100;

    // Common AX attribute strings
    pub const K_AX_TITLE_ATTRIBUTE: &str = "AXTitle";
    pub const K_AX_ROLE_ATTRIBUTE: &str = "AXRole";
    pub const K_AX_IDENTIFIER_ATTRIBUTE: &str = "AXIdentifier";
    pub const K_AX_DESCRIPTION_ATTRIBUTE: &str = "AXDescription";
    pub const K_AX_CHILDREN_ATTRIBUTE: &str = "AXChildren";
    pub const K_AX_POSITION_ATTRIBUTE: &str = "AXPosition";
    pub const K_AX_SIZE_ATTRIBUTE: &str = "AXSize";

    // Common AX actions
    pub const K_AX_PRESS_ACTION: &str = "AXPress";

    // Helper function to create CFString
    pub unsafe fn create_cfstring(s: &str) -> CFStringRef {
        let c_str = CString::new(s).unwrap();
        unsafe { CFStringCreateWithCString(ptr::null(), c_str.as_ptr(), K_CF_STRING_ENCODING_UTF8) }
    }
}

/// macOS Accessibility API connection handler
pub struct MacOSConnection {
    #[cfg(target_os = "macos")]
    system_element: ax_bindings::AXUIElementRef,
    config: Config,
}

// Safety: AXUIElementRef is a Core Foundation object that can be safely shared
// across threads when properly retained/released. The system element is immutable.
#[cfg(target_os = "macos")]
unsafe impl Send for MacOSConnection {}
#[cfg(target_os = "macos")]
unsafe impl Sync for MacOSConnection {}

impl MacOSConnection {
    /// Creates a new macOS connection
    pub async fn new(config: &Config) -> Result<Self> {
        #[cfg(target_os = "macos")]
        {
            // Check for accessibility permissions
            if config.platform.macos.check_accessibility_permissions {
                if !Self::check_accessibility_permissions() {
                    return Err(crate::error::BryndzaError::ConnectionError {
                        message: "Accessibility permissions not granted. Please enable accessibility permissions for this application in System Preferences > Privacy & Security > Privacy > Accessibility".to_string()
                    }.into());
                }
            }

            let system_element = unsafe { ax_bindings::AXUIElementCreateSystemWide() };
            if system_element.is_null() {
                return Err(crate::error::BryndzaError::ConnectionError {
                    message: "Failed to create system-wide accessibility element".to_string(),
                }
                .into());
            }

            Ok(Self {
                config: config.clone(),
                #[cfg(target_os = "macos")]
                system_element,
            })
        }

        #[cfg(not(target_os = "macos"))]
        {
            Ok(Self {
                config: config.clone(),
            })
        }
    }

    /// Checks if accessibility permissions are granted
    #[cfg(target_os = "macos")]
    pub fn check_accessibility_permissions() -> bool {
        unsafe { ax_bindings::AXIsProcessTrusted() != 0 }
    }

    /// Helper function to guide users through enabling accessibility permissions
    pub fn accessibility_permissions_help() -> String {
        "To enable accessibility permissions:\n\
         1. Open System Preferences (or System Settings on macOS 13+)\n\
         2. Go to Security & Privacy > Privacy > Accessibility\n\
         3. Click the lock icon and enter your password\n\
         4. Add your application to the list or check the box next to it\n\
         5. Restart your application"
            .to_string()
    }

    /// Searches for elements using the accessibility tree
    #[cfg(target_os = "macos")]
    async fn search_elements(
        &self,
        locator: &Locator,
        find_multiple: bool,
    ) -> Result<Vec<Element>> {
        let mut found_elements = Vec::new();

        // Start search from system-wide element
        self.search_element_recursive(
            self.system_element,
            locator,
            &mut found_elements,
            find_multiple,
        )?;

        Ok(found_elements)
    }

    /// Recursively searches for elements in the accessibility tree
    #[cfg(target_os = "macos")]
    fn search_element_recursive(
        &self,
        ax_element: ax_bindings::AXUIElementRef,
        locator: &Locator,
        found_elements: &mut Vec<Element>,
        find_multiple: bool,
    ) -> Result<()> {
        use ax_bindings::*;

        // Check if current element matches the locator
        if self.element_matches_locator(ax_element, locator)? {
            let element = self.ax_element_to_element(ax_element)?;
            found_elements.push(element);

            if !find_multiple {
                return Ok(());
            }
        }

        // Get children and search recursively
        let children_attr = unsafe { create_cfstring(K_AX_CHILDREN_ATTRIBUTE) };
        let mut children_ref: CFTypeRef = ptr::null();

        let result =
            unsafe { AXUIElementCopyAttributeValue(ax_element, children_attr, &mut children_ref) };

        unsafe { CFRelease(children_attr) };

        if result == K_AX_ERROR_SUCCESS && !children_ref.is_null() {
            let children_count = unsafe { CFArrayGetCount(children_ref as CFArrayRef) };

            for i in 0..children_count {
                let child = unsafe {
                    CFArrayGetValueAtIndex(children_ref as CFArrayRef, i) as AXUIElementRef
                };

                self.search_element_recursive(child, locator, found_elements, find_multiple)?;

                if !find_multiple && !found_elements.is_empty() {
                    break;
                }
            }

            unsafe { CFRelease(children_ref) };
        }

        Ok(())
    }

    /// Checks if an AX element matches the given locator
    #[cfg(target_os = "macos")]
    fn element_matches_locator(
        &self,
        ax_element: ax_bindings::AXUIElementRef,
        locator: &Locator,
    ) -> Result<bool> {
        use ax_bindings::*;

        match locator {
            Locator::Id(id) => self
                .get_ax_string_attribute(ax_element, K_AX_IDENTIFIER_ATTRIBUTE)
                .map(|value| value.as_deref() == Some(id)),
            Locator::ClassName(class_name) => self
                .get_ax_string_attribute(ax_element, K_AX_ROLE_ATTRIBUTE)
                .map(|value| value.as_deref() == Some(class_name)),
            Locator::Text(text) => self
                .get_ax_string_attribute(ax_element, K_AX_TITLE_ATTRIBUTE)
                .map(|value| value.as_deref() == Some(text)),
            Locator::AccessibilityId(label) => self
                .get_ax_string_attribute(ax_element, K_AX_DESCRIPTION_ATTRIBUTE)
                .map(|value| value.as_deref() == Some(label)),
            Locator::And(locators) => {
                for sub_locator in locators {
                    if !self.element_matches_locator(ax_element, sub_locator)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            Locator::Or(locators) => {
                for sub_locator in locators {
                    if self.element_matches_locator(ax_element, sub_locator)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            _ => Ok(false), // Other locator types not yet supported
        }
    }

    /// Gets a string attribute from an AX element
    #[cfg(target_os = "macos")]
    fn get_ax_string_attribute(
        &self,
        ax_element: ax_bindings::AXUIElementRef,
        attribute_name: &str,
    ) -> Result<Option<String>> {
        use ax_bindings::*;

        let attr_name = unsafe { create_cfstring(attribute_name) };
        let mut value_ref: CFTypeRef = ptr::null();

        let result =
            unsafe { AXUIElementCopyAttributeValue(ax_element, attr_name, &mut value_ref) };

        unsafe { CFRelease(attr_name) };

        if result == K_AX_ERROR_SUCCESS && !value_ref.is_null() {
            // Convert CFString to Rust String
            let cf_string = value_ref as CFStringRef;
            let rust_string = Self::cfstring_to_string(cf_string);
            unsafe { CFRelease(value_ref) };
            Ok(Some(rust_string))
        } else {
            Ok(None)
        }
    }

    /// Converts a CFString to a Rust String
    #[cfg(target_os = "macos")]
    fn cfstring_to_string(cf_string: ax_bindings::CFStringRef) -> String {
        use ax_bindings::*;

        if cf_string.is_null() {
            return String::new();
        }

        unsafe {
            let length = CFStringGetLength(cf_string);
            if length == 0 {
                return String::new();
            }

            let buffer_size = (length * 4 + 1) as usize; // UTF-8 worst case
            let mut buffer = vec![0u8; buffer_size];

            let result = CFStringGetCString(
                cf_string,
                buffer.as_mut_ptr() as *mut i8,
                buffer_size as i64,
                K_CF_STRING_ENCODING_UTF8,
            );

            if result != 0 {
                // Find the null terminator
                if let Some(null_pos) = buffer.iter().position(|&x| x == 0) {
                    buffer.truncate(null_pos);
                }
                String::from_utf8_lossy(&buffer).into_owned()
            } else {
                String::new()
            }
        }
    }

    /// Converts an AXUIElement to our Element type
    #[cfg(target_os = "macos")]
    fn ax_element_to_element(&self, _ax_element: ax_bindings::AXUIElementRef) -> Result<Element> {
        use ax_bindings::*;
        use std::collections::HashMap;

        // Extract element properties from the AX element
        let mut attributes = HashMap::new();

        // Get basic attributes
        if let Ok(Some(role)) = self.get_ax_string_attribute(_ax_element, K_AX_ROLE_ATTRIBUTE) {
            attributes.insert("role".to_string(), role);
        }

        if let Ok(Some(title)) = self.get_ax_string_attribute(_ax_element, K_AX_TITLE_ATTRIBUTE) {
            attributes.insert("title".to_string(), title);
        }

        if let Ok(Some(identifier)) =
            self.get_ax_string_attribute(_ax_element, K_AX_IDENTIFIER_ATTRIBUTE)
        {
            attributes.insert("identifier".to_string(), identifier);
        }

        if let Ok(Some(description)) =
            self.get_ax_string_attribute(_ax_element, K_AX_DESCRIPTION_ATTRIBUTE)
        {
            attributes.insert("description".to_string(), description);
        }

        // Get position and size (simplified - would need proper AXValue handling)
        // TODO: Implement AXValue extraction for position and size
        let bounds = crate::element::element::Rectangle::new(0, 0, 0, 0);

        // Create a unique ID for the element based on its attributes
        let element_id = if let Some(identifier) = attributes.get("identifier") {
            identifier.clone()
        } else if let Some(title) = attributes.get("title") {
            format!("title:{}", title)
        } else if let Some(role) = attributes.get("role") {
            format!("role:{}", role)
        } else {
            format!("ax_element:{:p}", _ax_element)
        };

        // For now, assume all elements are visible and enabled
        // TODO: Check AXEnabled and other state attributes

        Ok(Element::new(
            element_id, attributes, bounds, true, // visible
            true, // enabled
        ))
    }

    /// Disconnects from macOS Accessibility API
    pub async fn disconnect(self) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            // Release the system element reference
            if !self.system_element.is_null() {
                unsafe { ax_bindings::CFRelease(self.system_element) };
            }
        }
        Ok(())
    }

    /// Takes a screenshot using macOS Core Graphics
    pub async fn screenshot(&self) -> Result<Vec<u8>> {
        // TODO: Implement macOS screenshot using CGDisplayCreateImage
        // Leave for later as requested
        todo!("macOS screenshot implementation")
    }

    /// Finds an element using macOS Accessibility API
    pub async fn find_element(&self, locator: &Locator) -> Result<Element> {
        #[cfg(target_os = "macos")]
        {
            let elements = self.search_elements(locator, false).await?;
            elements
                .into_iter()
                .next()
                .ok_or_else(|| crate::error::BryndzaError::ElementNotFound {
                    locator: format!("{:?}", locator),
                })
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = locator;
            Err(crate::error::BryndzaError::PlatformNotSupported {
                platform: "macOS".to_string(),
            })
        }
    }

    /// Finds multiple elements using macOS Accessibility API
    pub async fn find_elements(&self, locator: &Locator) -> Result<Vec<Element>> {
        #[cfg(target_os = "macos")]
        {
            self.search_elements(locator, true).await
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = locator;
            Err(crate::error::BryndzaError::PlatformError(
                "macOS platform not supported on this system".to_string(),
            )
            .into())
        }
    }

    /// Clicks on an element using macOS Accessibility API
    pub async fn click_element(&self, element: &Element) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            // TODO: Get AXUIElementRef from element and perform click action
            // For now, this is a simplified implementation that would need
            // a way to associate AXUIElementRef with the Element

            // In a real implementation, we would need to either:
            // 1. Store the AXUIElementRef in the Element somehow
            // 2. Re-find the element based on its properties
            // 3. Use a different architecture

            // For demonstration, let's assume we could re-find the element
            // and then perform the click action:

            /*
            use ax_bindings::*;

            // Re-find the element (this is inefficient but shows the concept)
            let locator = Locator::Id(element.id.clone());
            let found_elements = self.search_elements(&locator, false).await?;

            if let Some(found_element) = found_elements.first() {
                // Get the AXUIElementRef (this would need to be stored in Element)
                let ax_element = ...; // Would need architecture change

                let press_action = unsafe { create_cfstring(K_AX_PRESS_ACTION) };
                let result = unsafe { AXUIElementPerformAction(ax_element, press_action) };
                unsafe { CFRelease(press_action) };

                if result == K_AX_ERROR_SUCCESS {
                    Ok(())
                } else {
                    Err(crate::error::BryndzaError::PlatformError(
                        format!("Failed to click element: AX error {}", result)
                    ).into())
                }
            } else {
                Err(crate::error::BryndzaError::ElementNotFound {
                    locator: format!("Element with id: {}", element.id)
                }.into())
            }
            */

            // For now, return a placeholder implementation
            let _ = element; // Use the parameter
            todo!("Click implementation requires storing AXUIElementRef in Element")
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = element;
            Err(crate::error::BryndzaError::PlatformError(
                "macOS platform not supported on this system".to_string(),
            )
            .into())
        }
    }

    /// Double-clicks on an element
    pub async fn double_click_element(&self, element: &Element) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            // Perform two clicks in succession
            self.click_element(element).await?;
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            self.click_element(element).await?;
            Ok(())
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = element;
            Err(crate::error::BryndzaError::PlatformError(
                "macOS platform not supported on this system".to_string(),
            )
            .into())
        }
    }

    /// Types text into an element
    pub async fn type_text(&self, element: &Element, text: &str) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            // For macOS text input, we would typically:
            // 1. Set focus to the element (AXFocused attribute)
            // 2. Set the AXValue attribute to the text, or
            // 3. Use AXUIElementPerformAction with type actions

            // This is a complex implementation that would require:
            // - Getting the AXUIElementRef from the element
            // - Setting the AXValue attribute with the text
            // - Or using keyboard events

            let _ = (element, text);
            todo!("Type text implementation requires AXValue attribute setting or keyboard events")
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = (element, text);
            Err(crate::error::BryndzaError::PlatformError(
                "macOS platform not supported on this system".to_string(),
            )
            .into())
        }
    }

    /// Clears text from an element
    pub async fn clear_element(&self, element: &Element) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            // Clear by setting empty text
            self.type_text(element, "").await
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = element;
            Err(crate::error::BryndzaError::PlatformError(
                "macOS platform not supported on this system".to_string(),
            )
            .into())
        }
    }

    /// Takes a screenshot of a specific element
    pub async fn element_screenshot(&self, element: &Element) -> Result<Vec<u8>> {
        #[cfg(target_os = "macos")]
        {
            // TODO: Implement macOS element screenshot
            // This would involve:
            // 1. Getting the element's position and size
            // 2. Taking a full screenshot
            // 3. Cropping to the element's bounds
            let _ = element;
            todo!("macOS element_screenshot implementation")
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = element;
            Err(crate::error::BryndzaError::PlatformError(
                "macOS platform not supported on this system".to_string(),
            )
            .into())
        }
    }

    /// Scrolls to bring an element into view
    pub async fn scroll_into_view(&self, element: &Element) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            // TODO: Implement macOS scroll using AXScrollToVisible
            // This would call the AXScrollToVisible action on the element
            let _ = element;
            todo!("macOS scroll_into_view implementation")
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = element;
            Err(crate::error::BryndzaError::PlatformError(
                "macOS platform not supported on this system".to_string(),
            )
            .into())
        }
    }
}
