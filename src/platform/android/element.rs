use crate::element::{Element, element::Rectangle};
use std::collections::HashMap;

/// Android-specific element implementation
pub struct AndroidElement {
    /// UI Automator element attributes
    pub attributes: HashMap<String, String>,
    /// Element bounds
    pub bounds: Rectangle,
}

impl AndroidElement {
    /// Creates a new Android element
    pub fn new(attributes: HashMap<String, String>, bounds: Rectangle) -> Self {
        Self { attributes, bounds }
    }

    /// Converts Android UI Automator element to generic Element
    pub fn to_element(&self) -> crate::error::Result<Element> {
        let visible = self.attributes
            .get("displayed")
            .map(|v| v == "true")
            .unwrap_or(true);

        let enabled = self.attributes
            .get("enabled")
            .map(|v| v == "true")
            .unwrap_or(true);

        let id = self.attributes
            .get("resource-id")
            .cloned()
            .unwrap_or_else(|| format!("android_{}", self.bounds.x + self.bounds.y));

        Ok(Element::new(
            id,
            self.attributes.clone(),
            self.bounds,
            visible,
            enabled,
        ))
    }

    /// Gets the element's resource ID
    pub fn resource_id(&self) -> Option<&str> {
        self.attributes.get("resource-id").map(|s| s.as_str())
    }

    /// Gets the element's class name
    pub fn class_name(&self) -> Option<&str> {
        self.attributes.get("class").map(|s| s.as_str())
    }

    /// Gets the element's text content
    pub fn text(&self) -> Option<&str> {
        self.attributes.get("text").map(|s| s.as_str())
    }

    /// Gets the element's content description
    pub fn content_desc(&self) -> Option<&str> {
        self.attributes.get("content-desc").map(|s| s.as_str())
    }

    /// Gets the element's package name
    pub fn package(&self) -> Option<&str> {
        self.attributes.get("package").map(|s| s.as_str())
    }

    /// Checks if the element is checkable
    pub fn is_checkable(&self) -> bool {
        self.attributes
            .get("checkable")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// Checks if the element is checked
    pub fn is_checked(&self) -> bool {
        self.attributes
            .get("checked")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// Checks if the element is clickable
    pub fn is_clickable(&self) -> bool {
        self.attributes
            .get("clickable")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// Checks if the element is enabled
    pub fn is_enabled(&self) -> bool {
        self.attributes
            .get("enabled")
            .map(|v| v == "true")
            .unwrap_or(true)
    }

    /// Checks if the element is focusable
    pub fn is_focusable(&self) -> bool {
        self.attributes
            .get("focusable")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// Checks if the element is focused
    pub fn is_focused(&self) -> bool {
        self.attributes
            .get("focused")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// Checks if the element is scrollable
    pub fn is_scrollable(&self) -> bool {
        self.attributes
            .get("scrollable")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// Checks if the element is long-clickable
    pub fn is_long_clickable(&self) -> bool {
        self.attributes
            .get("long-clickable")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// Checks if the element is password field
    pub fn is_password(&self) -> bool {
        self.attributes
            .get("password")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// Checks if the element is selected
    pub fn is_selected(&self) -> bool {
        self.attributes
            .get("selected")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// Gets the element's bounds
    pub fn bounds(&self) -> &Rectangle {
        &self.bounds
    }
}
