use bryndza::{Session, Locator, Result, element::element::SwipeDirection};
use std::time::Duration;

/// Mobile app automation example for Android/iOS
/// This example demonstrates:
/// - Touch gestures (tap, swipe, long press)
/// - Mobile-specific locators
/// - Device orientation
/// - Screenshot comparison
#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting mobile app automation example...");

    // Create a session for mobile automation
    let mut session = Session::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    session.start().await?;
    
    let platform_name = session.platform().platform_name();
    let capabilities = session.platform().capabilities();
    
    println!("Connected to {} platform", platform_name);
    println!("Touch support: {}", capabilities.supports_touch);

    if !capabilities.supports_touch {
        println!("This platform doesn't support touch gestures. Exiting.");
        session.stop().await?;
        return Ok(());
    }

    // Take initial screenshot
    let screenshot = session.screenshot().await?;
    println!("Initial screenshot: {} bytes", screenshot.len());

    // Example 1: Find and tap a button by resource ID (Android) or accessibility ID (iOS)
    match session.find_element(&Locator::accessibility_id("login-button")).await {
        Ok(button) => {
            println!("Found login button");
            button.click().await?; // On mobile, this is a tap
            
            // Wait a moment for the UI to respond
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
        Err(_) => println!("Login button not found"),
    }

    // Example 2: Find text input and enter text
    match session.find_element(&Locator::class_name("android.widget.EditText")).await {
        Ok(input) => {
            println!("Found text input");
            input.click().await?; // Tap to focus
            input.clear().await?;
            input.type_text("test@example.com").await?;
            println!("Entered email address");
        }
        Err(_) => {
            // Try iOS text field if Android EditText not found
            if let Ok(input) = session.find_element(&Locator::class_name("XCUIElementTypeTextField")).await {
                println!("Found iOS text field");
                input.click().await?;
                input.clear().await?;
                input.type_text("test@example.com").await?;
                println!("Entered email address");
            } else {
                println!("No text input found");
            }
        }
    }

    // Example 3: Password field (secure text)
    match session.find_element(&Locator::class_name("android.widget.EditText")).await {
        Ok(password_field) => {
            if password_field.attribute("password").unwrap_or("false") == "true" {
                println!("Found password field");
                password_field.click().await?;
                password_field.type_text("secretpassword").await?;
            }
        }
        Err(_) => println!("Password field not found"),
    }

    // Example 4: Swipe gestures
    if let Ok(scrollable) = session.find_element(&Locator::class_name("android.widget.ScrollView")).await {
        println!("Found scrollable view, performing swipe");
        scrollable.swipe(SwipeDirection::Up, 300.0).await?;
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        scrollable.swipe(SwipeDirection::Down, 300.0).await?;
        println!("Completed swipe gestures");
    }

    // Example 5: Long press gesture
    if let Ok(element) = session.find_element(&Locator::text("Menu Item")).await {
        println!("Performing long press on menu item");
        element.long_press(Duration::from_millis(1000)).await?;
        
        // Wait for context menu to appear
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        // Try to find and tap "Delete" option
        if let Ok(delete_option) = session.find_element(&Locator::text("Delete")).await {
            delete_option.click().await?;
            println!("Selected delete from context menu");
        }
    }

    // Example 6: Find elements in a list
    match session.find_elements(&Locator::class_name("android.widget.TextView")).await {
        Ok(text_views) => {
            println!("Found {} text views", text_views.len());
            for (i, text_view) in text_views.iter().enumerate().take(5) {
                if let Some(text) = text_view.text() {
                    println!("Text view {}: '{}'", i + 1, text);
                }
            }
        }
        Err(_) => println!("No text views found"),
    }

    // Example 7: Wait for specific text to appear
    println!("Waiting for success message...");
    match bryndza::wait::WaitConditions::element_text_contains(
        &session,
        &Locator::class_name("android.widget.TextView"),
        "Success",
        Duration::from_secs(10),
    ).await {
        Ok(element) => {
            println!("Success message appeared: {}", element.text().unwrap_or(""));
        }
        Err(_) => println!("Success message did not appear"),
    }

    // Example 8: Handle alerts/dialogs
    if let Ok(alert) = session.find_element(&Locator::class_name("android.app.AlertDialog")).await {
        println!("Found alert dialog");
        
        // Try to find and click OK button
        if let Ok(ok_button) = session.find_element(&Locator::text("OK")).await {
            ok_button.click().await?;
            println!("Dismissed alert");
        }
    }

    // Take final screenshot
    let final_screenshot = session.screenshot().await?;
    println!("Final screenshot: {} bytes", final_screenshot.len());

    // Stop the session
    session.stop().await?;
    println!("Mobile automation session completed");

    Ok(())
}
