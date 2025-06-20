use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a UI element in the automation framework
#[derive(Debug, Clone)]
pub struct Element {
    /// Platform-specific element identifier
    pub(crate) id: String,
    /// Element attributes
    pub(crate) attributes: HashMap<String, String>,
    /// Element bounds (x, y, width, height)
    pub(crate) bounds: Rectangle,
    /// Whether the element is visible
    pub(crate) visible: bool,
    /// Whether the element is enabled/interactable
    pub(crate) enabled: bool,
}

impl Element {
    /// Creates a new Element instance
    pub fn new(
        id: String,
        attributes: HashMap<String, String>,
        bounds: Rectangle,
        visible: bool,
        enabled: bool,
    ) -> Self {
        Self {
            id,
            attributes,
            bounds,
            visible,
            enabled,
        }
    }

    /// Gets the element's unique identifier
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Gets the element's text content
    pub fn text(&self) -> Option<&str> {
        self.attributes.get("text").map(|s| s.as_str())
    }

    /// Gets the element's class name
    pub fn class_name(&self) -> Option<&str> {
        self.attributes.get("className").map(|s| s.as_str())
    }

    /// Gets the element's resource ID (Android) or accessibility identifier (iOS)
    pub fn resource_id(&self) -> Option<&str> {
        self.attributes
            .get("resourceId")
            .or_else(|| self.attributes.get("accessibilityId"))
            .map(|s| s.as_str())
    }

    /// Gets an attribute value by name
    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    /// Gets all attributes
    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    /// Gets the element's bounds
    pub fn bounds(&self) -> &Rectangle {
        &self.bounds
    }

    /// Gets the center point of the element
    pub fn center(&self) -> Point {
        Point {
            x: self.bounds.x + self.bounds.width / 2,
            y: self.bounds.y + self.bounds.height / 2,
        }
    }

    /// Checks if the element is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Checks if the element is enabled/interactable
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Checks if the element is clickable
    pub fn is_clickable(&self) -> bool {
        self.visible && self.enabled
    }

    /// Clicks on the element
    pub async fn click(&self) -> Result<()> {
        crate::element::interaction::click(self).await
    }

    /// Double-clicks on the element
    pub async fn double_click(&self) -> Result<()> {
        crate::element::interaction::double_click(self).await
    }

    /// Long presses on the element (mobile platforms)
    pub async fn long_press(&self, duration: std::time::Duration) -> Result<()> {
        crate::element::interaction::long_press(self, duration).await
    }

    /// Types text into the element
    pub async fn type_text(&self, text: &str) -> Result<()> {
        crate::element::interaction::type_text(self, text).await
    }

    /// Clears the element's text content
    pub async fn clear(&self) -> Result<()> {
        crate::element::interaction::clear(self).await
    }

    /// Gets the element's screenshot
    pub async fn screenshot(&self) -> Result<Vec<u8>> {
        crate::element::interaction::screenshot(self).await
    }

    /// Scrolls to bring the element into view
    pub async fn scroll_into_view(&self) -> Result<()> {
        crate::element::interaction::scroll_into_view(self).await
    }

    /// Swipes on the element in the specified direction
    pub async fn swipe(&self, direction: SwipeDirection, distance: f64) -> Result<()> {
        crate::element::interaction::swipe(self, direction, distance).await
    }
}

/// Represents a rectangular area
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    /// Creates a new Rectangle
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }

    /// Checks if the rectangle contains the given point
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.x
            && point.x < self.x + self.width
            && point.y >= self.y
            && point.y < self.y + self.height
    }

    /// Gets the center point of the rectangle
    pub fn center(&self) -> Point {
        Point {
            x: self.x + self.width / 2,
            y: self.y + self.height / 2,
        }
    }
}

/// Represents a point in 2D space
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Creates a new Point
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Swipe directions for touch interactions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
}
