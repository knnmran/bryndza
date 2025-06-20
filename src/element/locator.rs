use serde::{Deserialize, Serialize};

/// Strategies for locating elements in the UI
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Locator {
    /// Locate by element ID
    Id(String),
    
    /// Locate by class name
    ClassName(String),
    
    /// Locate by text content (exact match)
    Text(String),
    
    /// Locate by partial text content
    PartialText(String),
    
    /// Locate by accessibility identifier
    AccessibilityId(String),
    
    /// Locate by XPath expression
    XPath(String),
    
    /// Locate by CSS selector (web elements)
    CssSelector(String),
    
    /// Locate by tag name
    TagName(String),
    
    /// Locate by attribute value
    Attribute { name: String, value: String },
    
    /// Locate by image comparison
    Image(ImageLocator),
    
    /// Locate by coordinates
    Coordinates { x: i32, y: i32 },
    
    /// Combine multiple locators with AND logic
    And(Vec<Locator>),
    
    /// Combine multiple locators with OR logic
    Or(Vec<Locator>),
}

impl Locator {
    /// Creates a locator for element ID
    pub fn id<S: Into<String>>(id: S) -> Self {
        Self::Id(id.into())
    }

    /// Creates a locator for class name
    pub fn class_name<S: Into<String>>(class_name: S) -> Self {
        Self::ClassName(class_name.into())
    }

    /// Creates a locator for exact text match
    pub fn text<S: Into<String>>(text: S) -> Self {
        Self::Text(text.into())
    }

    /// Creates a locator for partial text match
    pub fn partial_text<S: Into<String>>(text: S) -> Self {
        Self::PartialText(text.into())
    }

    /// Creates a locator for accessibility identifier
    pub fn accessibility_id<S: Into<String>>(id: S) -> Self {
        Self::AccessibilityId(id.into())
    }

    /// Creates a locator for XPath expression
    pub fn xpath<S: Into<String>>(xpath: S) -> Self {
        Self::XPath(xpath.into())
    }

    /// Creates a locator for CSS selector
    pub fn css_selector<S: Into<String>>(selector: S) -> Self {
        Self::CssSelector(selector.into())
    }

    /// Creates a locator for tag name
    pub fn tag_name<S: Into<String>>(tag_name: S) -> Self {
        Self::TagName(tag_name.into())
    }

    /// Creates a locator for attribute value
    pub fn attribute<S: Into<String>>(name: S, value: S) -> Self {
        Self::Attribute {
            name: name.into(),
            value: value.into(),
        }
    }

    /// Creates a locator for image comparison
    pub fn image(image_locator: ImageLocator) -> Self {
        Self::Image(image_locator)
    }

    /// Creates a locator for coordinates
    pub fn coordinates(x: i32, y: i32) -> Self {
        Self::Coordinates { x, y }
    }

    /// Combines locators with AND logic
    pub fn and(locators: Vec<Locator>) -> Self {
        Self::And(locators)
    }

    /// Combines locators with OR logic
    pub fn or(locators: Vec<Locator>) -> Self {
        Self::Or(locators)
    }

    /// Returns a string representation of the locator for error messages
    pub fn description(&self) -> String {
        match self {
            Self::Id(id) => format!("id='{}'", id),
            Self::ClassName(class) => format!("className='{}'", class),
            Self::Text(text) => format!("text='{}'", text),
            Self::PartialText(text) => format!("partialText='{}'", text),
            Self::AccessibilityId(id) => format!("accessibilityId='{}'", id),
            Self::XPath(xpath) => format!("xpath='{}'", xpath),
            Self::CssSelector(selector) => format!("cssSelector='{}'", selector),
            Self::TagName(tag) => format!("tagName='{}'", tag),
            Self::Attribute { name, value } => format!("{}='{}'", name, value),
            Self::Image(img) => format!("image='{}'", img.description()),
            Self::Coordinates { x, y } => format!("coordinates=({}, {})", x, y),
            Self::And(locators) => {
                let descriptions: Vec<String> = locators.iter().map(|l| l.description()).collect();
                format!("AND({})", descriptions.join(", "))
            }
            Self::Or(locators) => {
                let descriptions: Vec<String> = locators.iter().map(|l| l.description()).collect();
                format!("OR({})", descriptions.join(", "))
            }
        }
    }
}

/// Image-based element location
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageLocator {
    /// Base64 encoded image data or file path
    pub image_data: ImageData,
    /// Similarity threshold (0.0 to 1.0)
    pub threshold: f64,
    /// Region of screen to search within
    pub search_region: Option<crate::element::element::Rectangle>,
}

impl ImageLocator {
    /// Creates a new image locator from file path
    pub fn from_file<S: Into<String>>(path: S) -> Self {
        Self {
            image_data: ImageData::FilePath(path.into()),
            threshold: 0.8,
            search_region: None,
        }
    }

    /// Creates a new image locator from base64 data
    pub fn from_base64<S: Into<String>>(data: S) -> Self {
        Self {
            image_data: ImageData::Base64(data.into()),
            threshold: 0.8,
            search_region: None,
        }
    }

    /// Sets the similarity threshold
    pub fn threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Sets the search region
    pub fn search_region(mut self, region: crate::element::element::Rectangle) -> Self {
        self.search_region = Some(region);
        self
    }

    /// Returns a description of the image locator
    pub fn description(&self) -> String {
        match &self.image_data {
            ImageData::FilePath(path) => format!("file:{} (threshold: {})", path, self.threshold),
            ImageData::Base64(_) => format!("base64_image (threshold: {})", self.threshold),
        }
    }
}

/// Image data types for image locators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImageData {
    /// File path to image
    FilePath(String),
    /// Base64 encoded image data
    Base64(String),
}
