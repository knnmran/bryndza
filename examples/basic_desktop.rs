use bryndza::{Session, Locator, Result};
use std::time::Duration;

/// Basic desktop automation example
/// This example shows how to:
/// - Create a session
/// - Find elements
/// - Perform interactions
/// - Take screenshots
#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting basic desktop automation example...");

    // Create a new automation session with custom timeout
    let mut session = Session::builder()
        .timeout(Duration::from_secs(30))
        .max_retries(3)
        .retry_delay(Duration::from_millis(500))
        .build()?;

    // Start the session (connects to the platform)
    session.start().await?;
    println!("Session started on platform: {}", session.platform().platform_name());

    // Take a screenshot before starting
    match session.screenshot().await {
        Ok(screenshot) => {
            println!("Screenshot taken: {} bytes", screenshot.len());
            // Optionally save to file
            // std::fs::write("before.png", screenshot)?;
        }
        Err(e) => println!("Failed to take screenshot: {}", e),
    }

    // Example 1: Find element by ID (common in web and desktop apps)
    match session.find_element(&Locator::id("start-button")).await {
        Ok(element) => {
            println!("Found start button!");
            if element.is_clickable() {
                element.click().await?;
                println!("Clicked start button");
            }
        }
        Err(_) => println!("Start button not found"),
    }

    // Example 2: Find element by text content
    match session.find_element(&Locator::text("File")).await {
        Ok(menu) => {
            println!("Found File menu");
            menu.click().await?;
            
            // Wait for submenu to appear and click "New"
            tokio::time::sleep(Duration::from_millis(500)).await;
            if let Ok(new_item) = session.find_element(&Locator::text("New")).await {
                new_item.click().await?;
                println!("Clicked File > New");
            }
        }
        Err(_) => println!("File menu not found"),
    }

    // Example 3: Find text input and type text
    match session.find_element(&Locator::class_name("TextBox")).await {
        Ok(textbox) => {
            println!("Found text input");
            textbox.click().await?; // Focus the textbox
            textbox.clear().await?; // Clear existing text
            textbox.type_text("Hello from Bryndza!").await?;
            println!("Typed text into textbox");
        }
        Err(_) => println!("Text input not found"),
    }

    // Example 4: Wait for an element to appear
    println!("Waiting for dialog to appear...");
    match session.wait_for_element(&Locator::class_name("Dialog")).await {
        Ok(dialog) => {
            println!("Dialog appeared!");
            
            // Find OK button in the dialog
            if let Ok(ok_button) = session.find_element(&Locator::text("OK")).await {
                ok_button.click().await?;
                println!("Clicked OK button");
            }
        }
        Err(_) => println!("Dialog did not appear within timeout"),
    }

    // Example 5: Find multiple elements
    match session.find_elements(&Locator::class_name("Button")).await {
        Ok(buttons) => {
            println!("Found {} buttons", buttons.len());
            for (i, button) in buttons.iter().enumerate() {
                if let Some(text) = button.text() {
                    println!("Button {}: '{}'", i + 1, text);
                }
            }
        }
        Err(_) => println!("No buttons found"),
    }

    // Take a final screenshot
    match session.screenshot().await {
        Ok(screenshot) => {
            println!("Final screenshot taken: {} bytes", screenshot.len());
            // std::fs::write("after.png", screenshot)?;
        }
        Err(e) => println!("Failed to take final screenshot: {}", e),
    }

    // Stop the session (disconnects from platform)
    session.stop().await?;
    println!("Session stopped");

    Ok(())
}
