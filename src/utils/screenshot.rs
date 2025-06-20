use crate::{config::Config, error::Result};
use std::path::Path;

/// Screenshot utilities for capturing and managing screen images
pub struct Screenshot;

impl Screenshot {
    /// Takes a full screen screenshot
    pub async fn take_full_screen() -> Result<Vec<u8>> {
        // Platform-specific implementation will be used
        todo!("Full screen screenshot implementation")
    }

    /// Takes a screenshot of a specific region
    pub async fn take_region(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>> {
        // TODO: Implement region screenshot
        todo!("Region screenshot implementation")
    }

    /// Takes a screenshot of a specific window
    pub async fn take_window(window_handle: WindowHandle) -> Result<Vec<u8>> {
        // TODO: Implement window screenshot
        todo!("Window screenshot implementation")
    }

    /// Saves screenshot to file
    pub fn save_to_file<P: AsRef<Path>>(
        screenshot_data: &[u8],
        path: P,
        format: crate::utils::image::ImageFormat,
    ) -> Result<()> {
        // TODO: Implement screenshot saving
        todo!("Screenshot saving implementation")
    }

    /// Saves screenshot with timestamp
    pub fn save_with_timestamp<P: AsRef<Path>>(
        screenshot_data: &[u8],
        directory: P,
        prefix: &str,
        format: crate::utils::image::ImageFormat,
    ) -> Result<std::path::PathBuf> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S_%3f");
        let filename = format!("{}_{}.{}", prefix, timestamp, format_extension(format));
        let filepath = directory.as_ref().join(filename);
        
        Self::save_to_file(screenshot_data, &filepath, format)?;
        Ok(filepath)
    }

    /// Automatically saves screenshot on test failure
    pub async fn auto_save_on_failure(
        config: &Config,
        test_name: &str,
        error: &crate::error::BryndzaError,
    ) -> Result<Option<std::path::PathBuf>> {
        if !config.screenshot.auto_screenshot_on_failure {
            return Ok(None);
        }

        let screenshot = Self::take_full_screen().await?;
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S_%3f");
        let filename = format!("failure_{}_{}.{}", 
            test_name, 
            timestamp, 
            format_extension(config.screenshot.format.clone().into())
        );
        
        // Create screenshots directory if it doesn't exist
        let screenshots_dir = std::path::Path::new("screenshots");
        if !screenshots_dir.exists() {
            std::fs::create_dir_all(screenshots_dir)?;
        }
        
        let filepath = screenshots_dir.join(filename);
        Self::save_to_file(&screenshot, &filepath, config.screenshot.format.clone().into())?;
        
        Ok(Some(filepath))
    }

    /// Compares current screen with reference image
    pub async fn compare_with_reference<P: AsRef<Path>>(
        reference_path: P,
        threshold: f64,
    ) -> Result<ScreenshotComparison> {
        let current_screenshot = Self::take_full_screen().await?;
        let reference_image = std::fs::read(reference_path)?;
        
        let similarity = crate::utils::image::ImageComparison::compare_images(
            &current_screenshot,
            &reference_image,
            crate::utils::image::ComparisonAlgorithm::StructuralSimilarity,
        )?;
        
        Ok(ScreenshotComparison {
            similarity,
            passed: similarity >= threshold,
            threshold,
            current_screenshot,
            reference_image,
        })
    }

    /// Takes a diff screenshot highlighting differences
    pub fn create_diff_image(
        image1: &[u8],
        image2: &[u8],
    ) -> Result<Vec<u8>> {
        // TODO: Implement diff image generation
        todo!("Diff image generation implementation")
    }
}

/// Platform-specific window handle
#[derive(Debug, Clone)]
pub enum WindowHandle {
    /// Windows HWND
    Windows(isize),
    /// macOS window number
    MacOS(u32),
    /// X11 window ID (Linux)
    X11(u64),
}

/// Result of screenshot comparison
#[derive(Debug)]
pub struct ScreenshotComparison {
    /// Similarity score (0.0 to 1.0)
    pub similarity: f64,
    /// Whether the comparison passed the threshold
    pub passed: bool,
    /// Threshold used for comparison
    pub threshold: f64,
    /// Current screenshot data
    pub current_screenshot: Vec<u8>,
    /// Reference image data
    pub reference_image: Vec<u8>,
}

impl ScreenshotComparison {
    /// Saves comparison results to files
    pub fn save_results<P: AsRef<Path>>(
        &self,
        output_dir: P,
        test_name: &str,
    ) -> Result<ComparisonFiles> {
        let output_dir = output_dir.as_ref();
        std::fs::create_dir_all(output_dir)?;
        
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S_%3f");
        
        // Save current screenshot
        let current_path = output_dir.join(format!("current_{}_{}.png", test_name, timestamp));
        Screenshot::save_to_file(&self.current_screenshot, &current_path, crate::utils::image::ImageFormat::PNG)?;
        
        // Save reference image
        let reference_path = output_dir.join(format!("reference_{}_{}.png", test_name, timestamp));
        Screenshot::save_to_file(&self.reference_image, &reference_path, crate::utils::image::ImageFormat::PNG)?;
        
        // Create and save diff image
        let diff_image = Screenshot::create_diff_image(&self.current_screenshot, &self.reference_image)?;
        let diff_path = output_dir.join(format!("diff_{}_{}.png", test_name, timestamp));
        Screenshot::save_to_file(&diff_image, &diff_path, crate::utils::image::ImageFormat::PNG)?;
        
        Ok(ComparisonFiles {
            current: current_path,
            reference: reference_path,
            diff: diff_path,
        })
    }
}

/// Paths to comparison result files
#[derive(Debug)]
pub struct ComparisonFiles {
    pub current: std::path::PathBuf,
    pub reference: std::path::PathBuf,
    pub diff: std::path::PathBuf,
}

fn format_extension(format: crate::utils::image::ImageFormat) -> &'static str {
    match format {
        crate::utils::image::ImageFormat::PNG => "png",
        crate::utils::image::ImageFormat::JPEG => "jpg",
        crate::utils::image::ImageFormat::BMP => "bmp",
        crate::utils::image::ImageFormat::TIFF => "tiff",
        crate::utils::image::ImageFormat::WEBP => "webp",
    }
}
