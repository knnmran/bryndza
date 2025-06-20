use crate::element::{Element, element::Rectangle};
use std::collections::HashMap;

/// iOS-specific element implementation
pub struct IOSElement {
    /// XCUIElement attributes
    pub attributes: HashMap<String, String>,
    /// Element bounds
    pub bounds: Rectangle,
    /// Element type (XCUIElementType)
    pub element_type: IOSElementType,
}

impl IOSElement {
    /// Creates a new iOS element
    pub fn new(
        attributes: HashMap<String, String>, 
        bounds: Rectangle, 
        element_type: IOSElementType
    ) -> Self {
        Self { 
            attributes, 
            bounds, 
            element_type 
        }
    }

    /// Converts iOS XCUIElement to generic Element
    pub fn to_element(&self) -> crate::error::Result<Element> {
        let visible = self.attributes
            .get("visible")
            .map(|v| v == "true")
            .unwrap_or(true);

        let enabled = self.attributes
            .get("enabled")
            .map(|v| v == "true")
            .unwrap_or(true);

        let id = self.attributes
            .get("identifier")
            .cloned()
            .unwrap_or_else(|| format!("ios_{}", self.bounds.x + self.bounds.y));

        Ok(Element::new(
            id,
            self.attributes.clone(),
            self.bounds,
            visible,
            enabled,
        ))
    }

    /// Gets the element's accessibility identifier
    pub fn accessibility_identifier(&self) -> Option<&str> {
        self.attributes.get("identifier").map(|s| s.as_str())
    }

    /// Gets the element's label
    pub fn label(&self) -> Option<&str> {
        self.attributes.get("label").map(|s| s.as_str())
    }

    /// Gets the element's value
    pub fn value(&self) -> Option<&str> {
        self.attributes.get("value").map(|s| s.as_str())
    }

    /// Gets the element's title
    pub fn title(&self) -> Option<&str> {
        self.attributes.get("title").map(|s| s.as_str())
    }

    /// Gets the element's placeholder text
    pub fn placeholder_value(&self) -> Option<&str> {
        self.attributes.get("placeholderValue").map(|s| s.as_str())
    }

    /// Gets the element type
    pub fn element_type(&self) -> &IOSElementType {
        &self.element_type
    }

    /// Checks if the element is enabled
    pub fn is_enabled(&self) -> bool {
        self.attributes
            .get("enabled")
            .map(|v| v == "true")
            .unwrap_or(true)
    }

    /// Checks if the element is visible
    pub fn is_visible(&self) -> bool {
        self.attributes
            .get("visible")
            .map(|v| v == "true")
            .unwrap_or(true)
    }

    /// Checks if the element exists
    pub fn exists(&self) -> bool {
        self.attributes
            .get("exists")
            .map(|v| v == "true")
            .unwrap_or(true)
    }

    /// Checks if the element is hittable (can receive touch events)
    pub fn is_hittable(&self) -> bool {
        self.attributes
            .get("hittable")
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

    /// Checks if the element has keyboard focus
    pub fn has_keyboard_focus(&self) -> bool {
        self.attributes
            .get("hasKeyboardFocus")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// Gets the element's bounds
    pub fn bounds(&self) -> &Rectangle {
        &self.bounds
    }
}

/// iOS XCUIElement types
#[derive(Debug, Clone, PartialEq)]
pub enum IOSElementType {
    Any,
    Other,
    Application,
    Group,
    Window,
    Sheet,
    Drawer,
    Alert,
    Dialog,
    Button,
    RadioButton,
    RadioGroup,
    CheckBox,
    DisclosureTriangle,
    PopUpButton,
    ComboBox,
    MenuButton,
    ToolbarButton,
    Popover,
    Keyboard,
    Key,
    NavigationBar,
    TabBar,
    TabGroup,
    Toolbar,
    StatusBar,
    Table,
    TableRow,
    TableColumn,
    Outline,
    OutlineRow,
    Browser,
    CollectionView,
    Slider,
    PageIndicator,
    ProgressIndicator,
    ActivityIndicator,
    SegmentedControl,
    Picker,
    PickerWheel,
    Switch,
    Toggle,
    Link,
    Image,
    Icon,
    SearchField,
    ScrollView,
    ScrollBar,
    StaticText,
    TextField,
    SecureTextField,
    DatePicker,
    TextView,
    Menu,
    MenuItem,
    MenuBar,
    MenuBarItem,
    Map,
    WebView,
    IncrementArrow,
    DecrementArrow,
    Timeline,
    RatingIndicator,
    ValueIndicator,
    SplitGroup,
    Splitter,
    RelevanceIndicator,
    ColorWell,
    HelpTag,
    Matte,
    DockItem,
    Ruler,
    RulerMarker,
    Grid,
    LevelIndicator,
    Cell,
    LayoutArea,
    LayoutItem,
    Handle,
    Stepper,
    Tab,
}

impl Default for IOSElementType {
    fn default() -> Self {
        Self::Any
    }
}
