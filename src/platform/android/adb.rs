use crate::{
    config::Config,
    element::{Element, Locator, element::SwipeDirection},
    error::Result,
};
use std::process::Command;

/// ADB connection handler for Android automation
pub struct AdbConnection {
    config: Config,
    device_serial: Option<String>,
}

impl AdbConnection {
    /// Creates a new ADB connection
    pub async fn new(config: &Config) -> Result<Self> {
        let mut connection = Self {
            config: config.clone(),
            device_serial: config.platform.android.device_serial.clone(),
        };

        // Verify ADB is available
        connection.verify_adb().await?;
        
        // Connect to device
        connection.connect_device().await?;
        
        Ok(connection)
    }

    /// Disconnects from ADB
    pub async fn disconnect(self) -> Result<()> {
        // No explicit disconnection needed for ADB
        Ok(())
    }

    /// Verifies that ADB is available and responsive
    async fn verify_adb(&self) -> Result<()> {
        let output = Command::new("adb")
            .arg("version")
            .output()
            .map_err(|e| crate::error::BryndzaError::connection(
                format!("ADB not found or not accessible: {}", e)
            ))?;

        if !output.status.success() {
            return Err(crate::error::BryndzaError::connection(
                "ADB command failed - make sure Android SDK is installed and ADB is in PATH"
            ));
        }

        Ok(())
    }

    /// Connects to Android device
    async fn connect_device(&mut self) -> Result<()> {
        // List connected devices
        let mut cmd = Command::new("adb");
        cmd.arg("devices");
        
        let output = cmd.output()
            .map_err(|e| crate::error::BryndzaError::connection(
                format!("Failed to list ADB devices: {}", e)
            ))?;

        if !output.status.success() {
            return Err(crate::error::BryndzaError::connection(
                "Failed to list connected Android devices"
            ));
        }

        let devices_output = String::from_utf8_lossy(&output.stdout);
        
        // Parse device list and select device
        if self.device_serial.is_none() {
            // Auto-select first available device
            for line in devices_output.lines().skip(1) {
                if line.contains("\tdevice") {
                    let serial = line.split('\t').next().unwrap().to_string();
                    self.device_serial = Some(serial);
                    break;
                }
            }
        }

        if self.device_serial.is_none() {
            return Err(crate::error::BryndzaError::connection(
                "No Android devices found"
            ));
        }

        Ok(())
    }

    /// Builds ADB command with device serial if specified
    fn adb_command(&self) -> Command {
        let mut cmd = Command::new("adb");
        if let Some(serial) = &self.device_serial {
            cmd.arg("-s").arg(serial);
        }
        cmd
    }

    /// Takes a screenshot using ADB
    pub async fn screenshot(&self) -> Result<Vec<u8>> {
        let mut cmd = self.adb_command();
        cmd.args(&["exec-out", "screencap", "-p"]);

        let output = cmd.output()
            .map_err(|e| crate::error::BryndzaError::screenshot(
                format!("Failed to take screenshot: {}", e)
            ))?;

        if !output.status.success() {
            return Err(crate::error::BryndzaError::screenshot(
                "ADB screencap command failed"
            ));
        }

        Ok(output.stdout)
    }

    /// Finds an element using UI Automator
    pub async fn find_element(&self, locator: &Locator) -> Result<Element> {
        // TODO: Implement element finding using UI Automator dump and parsing
        todo!("Android find_element implementation")
    }

    /// Finds multiple elements using UI Automator
    pub async fn find_elements(&self, locator: &Locator) -> Result<Vec<Element>> {
        // TODO: Implement elements finding using UI Automator
        todo!("Android find_elements implementation")
    }

    /// Clicks on an element using ADB input tap
    pub async fn click_element(&self, element: &Element) -> Result<()> {
        let center = element.center();
        
        let mut cmd = self.adb_command();
        cmd.args(&["shell", "input", "tap", &center.x.to_string(), &center.y.to_string()]);

        let output = cmd.output()
            .map_err(|e| crate::error::BryndzaError::Platform(
                crate::error::PlatformError::AndroidADB { 
                    message: format!("Failed to click element: {}", e) 
                }
            ))?;

        if !output.status.success() {
            return Err(crate::error::BryndzaError::Platform(
                crate::error::PlatformError::AndroidADB { 
                    message: "ADB input tap command failed".to_string() 
                }
            ));
        }

        Ok(())
    }

    /// Performs a long press on an element
    pub async fn long_press_element(&self, element: &Element, duration: std::time::Duration) -> Result<()> {
        let center = element.center();
        let duration_ms = duration.as_millis() as u64;
        
        let mut cmd = self.adb_command();
        cmd.args(&[
            "shell", "input", "swipe", 
            &center.x.to_string(), &center.y.to_string(),
            &center.x.to_string(), &center.y.to_string(),
            &duration_ms.to_string()
        ]);

        let output = cmd.output()
            .map_err(|e| crate::error::BryndzaError::Platform(
                crate::error::PlatformError::AndroidADB { 
                    message: format!("Failed to long press element: {}", e) 
                }
            ))?;

        if !output.status.success() {
            return Err(crate::error::BryndzaError::Platform(
                crate::error::PlatformError::AndroidADB { 
                    message: "ADB long press command failed".to_string() 
                }
            ));
        }

        Ok(())
    }

    /// Types text using ADB input text
    pub async fn type_text(&self, _element: &Element, text: &str) -> Result<()> {
        let mut cmd = self.adb_command();
        cmd.args(&["shell", "input", "text", text]);

        let output = cmd.output()
            .map_err(|e| crate::error::BryndzaError::Platform(
                crate::error::PlatformError::AndroidADB { 
                    message: format!("Failed to type text: {}", e) 
                }
            ))?;

        if !output.status.success() {
            return Err(crate::error::BryndzaError::Platform(
                crate::error::PlatformError::AndroidADB { 
                    message: "ADB input text command failed".to_string() 
                }
            ));
        }

        Ok(())
    }

    /// Clears text from an element
    pub async fn clear_element(&self, element: &Element) -> Result<()> {
        // Click on element first to focus it
        self.click_element(element).await?;
        
        // Select all text and delete
        let mut cmd = self.adb_command();
        cmd.args(&["shell", "input", "keyevent", "KEYCODE_CTRL_A"]);
        cmd.output().ok();

        let mut cmd = self.adb_command();
        cmd.args(&["shell", "input", "keyevent", "KEYCODE_DEL"]);
        
        let output = cmd.output()
            .map_err(|e| crate::error::BryndzaError::Platform(
                crate::error::PlatformError::AndroidADB { 
                    message: format!("Failed to clear element: {}", e) 
                }
            ))?;

        if !output.status.success() {
            return Err(crate::error::BryndzaError::Platform(
                crate::error::PlatformError::AndroidADB { 
                    message: "ADB clear text command failed".to_string() 
                }
            ));
        }

        Ok(())
    }

    /// Takes a screenshot of a specific element
    pub async fn element_screenshot(&self, element: &Element) -> Result<Vec<u8>> {
        // For now, take full screenshot and crop later
        // TODO: Implement proper element bounds cropping
        self.screenshot().await
    }

    /// Scrolls to bring an element into view
    pub async fn scroll_into_view(&self, _element: &Element) -> Result<()> {
        // TODO: Implement scrolling using UI Automator scrollable
        todo!("Android scroll_into_view implementation")
    }

    /// Performs a swipe gesture on an element
    pub async fn swipe_element(&self, element: &Element, direction: SwipeDirection, distance: f64) -> Result<()> {
        let center = element.center();
        let (end_x, end_y) = match direction {
            SwipeDirection::Up => (center.x, center.y - distance as i32),
            SwipeDirection::Down => (center.x, center.y + distance as i32),
            SwipeDirection::Left => (center.x - distance as i32, center.y),
            SwipeDirection::Right => (center.x + distance as i32, center.y),
        };

        let mut cmd = self.adb_command();
        cmd.args(&[
            "shell", "input", "swipe",
            &center.x.to_string(), &center.y.to_string(),
            &end_x.to_string(), &end_y.to_string()
        ]);

        let output = cmd.output()
            .map_err(|e| crate::error::BryndzaError::Platform(
                crate::error::PlatformError::AndroidADB { 
                    message: format!("Failed to swipe element: {}", e) 
                }
            ))?;

        if !output.status.success() {
            return Err(crate::error::BryndzaError::Platform(
                crate::error::PlatformError::AndroidADB { 
                    message: "ADB swipe command failed".to_string() 
                }
            ));
        }

        Ok(())
    }
}
