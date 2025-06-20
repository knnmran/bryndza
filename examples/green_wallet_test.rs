use bryndza::{Session, Locator, Result};
use std::time::Duration;

/// Green Wallet app automation test
/// This example demonstrates testing a real mobile app
/// with various UI automation scenarios
#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Green Wallet automation test...");

    let mut session = Session::builder()
        .timeout(Duration::from_secs(30))
        .max_retries(3)
        .build()?;

    session.start().await?;
    println!("Connected to {}", session.platform().platform_name());

    // Take initial screenshot
    let _screenshot = session.screenshot().await?;

    // Test scenario 1: Launch and setup
    test_app_launch(&session).await?;
    
    // Test scenario 2: Create new wallet
    test_wallet_creation(&session).await?;
    
    // Test scenario 3: Backup and recovery
    test_backup_recovery(&session).await?;
    
    // Test scenario 4: Send transaction
    test_send_transaction(&session).await?;
    
    // Test scenario 5: Settings navigation
    test_settings_navigation(&session).await?;

    session.stop().await?;
    println!("Green Wallet automation test completed");
    Ok(())
}

async fn test_app_launch(session: &Session) -> Result<()> {
    println!("Testing app launch and initial screens...");

    // Wait for app to load
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Check if we're on the welcome screen
    if let Ok(_) = session.find_element(&Locator::text("Welcome to Green")).await {
        println!("App launched successfully - on welcome screen");
        
        // Tap "Get Started" button
        if let Ok(get_started) = session.find_element(&Locator::text("Get Started")).await {
            get_started.click().await?;
            println!("Tapped 'Get Started'");
        }
    } else {
        println!("App might already be set up or different screen");
    }

    Ok(())
}

async fn test_wallet_creation(session: &Session) -> Result<()> {
    println!("Testing wallet creation flow...");

    // Look for "Create new wallet" option
    if let Ok(create_wallet) = session.find_element(&Locator::text("Create new wallet")).await {
        create_wallet.click().await?;
        println!("Selected 'Create new wallet'");
        
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        // Set wallet name
        if let Ok(name_field) = session.find_element(&Locator::accessibility_id("wallet-name-input")).await {
            name_field.click().await?;
            name_field.clear().await?;
            name_field.type_text("Test Wallet").await?;
            println!("Entered wallet name");
        }
        
        // Continue with setup
        if let Ok(continue_btn) = session.find_element(&Locator::text("Continue")).await {
            continue_btn.click().await?;
            println!("Continued with wallet setup");
        }
    }

    Ok(())
}

async fn test_backup_recovery(session: &Session) -> Result<()> {
    println!("Testing backup and recovery phrase...");

    // Look for recovery phrase screen
    if let Ok(_) = session.wait_for_element(&Locator::text("Recovery Phrase")).await {
        println!("On recovery phrase screen");
        
        // Find all word elements (usually numbered 1-24)
        match session.find_elements(&Locator::class_name("recovery-word")).await {
            Ok(words) => {
                println!("Found {} recovery words", words.len());
                
                // Simulate writing down the words (in real test, might validate them)
                for (i, word) in words.iter().enumerate().take(3) {
                    if let Some(text) = word.text() {
                        println!("Word {}: {}", i + 1, text);
                    }
                }
            }
            Err(_) => println!("Recovery words not found in expected format"),
        }
        
        // Confirm backup
        if let Ok(confirm_btn) = session.find_element(&Locator::text("I have written it down")).await {
            confirm_btn.click().await?;
            println!("Confirmed backup written down");
        }
    }

    Ok(())
}

async fn test_send_transaction(session: &Session) -> Result<()> {
    println!("Testing send transaction flow...");

    // Navigate to send screen
    if let Ok(send_btn) = session.find_element(&Locator::accessibility_id("send-button")).await {
        send_btn.click().await?;
        println!("Opened send screen");
        
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        // Enter recipient address
        if let Ok(address_field) = session.find_element(&Locator::accessibility_id("recipient-address")).await {
            address_field.click().await?;
            address_field.type_text("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh").await?;
            println!("Entered recipient address");
        }
        
        // Enter amount
        if let Ok(amount_field) = session.find_element(&Locator::accessibility_id("amount-input")).await {
            amount_field.click().await?;
            amount_field.type_text("0.001").await?;
            println!("Entered amount");
        }
        
        // Review transaction (but don't actually send)
        if let Ok(review_btn) = session.find_element(&Locator::text("Review")).await {
            review_btn.click().await?;
            println!("Reviewing transaction");
            
            // Cancel instead of sending
            if let Ok(cancel_btn) = session.find_element(&Locator::text("Cancel")).await {
                cancel_btn.click().await?;
                println!("Cancelled transaction");
            }
        }
    }

    Ok(())
}

async fn test_settings_navigation(session: &Session) -> Result<()> {
    println!("Testing settings navigation...");

    // Open settings (usually in menu or bottom nav)
    if let Ok(menu_btn) = session.find_element(&Locator::accessibility_id("menu-button")).await {
        menu_btn.click().await?;
        
        if let Ok(settings) = session.find_element(&Locator::text("Settings")).await {
            settings.click().await?;
            println!("Opened settings");
            
            // Test various settings sections
            let settings_sections = vec![
                "Security",
                "Network",
                "About",
                "Support",
            ];
            
            for section in settings_sections {
                if let Ok(section_item) = session.find_element(&Locator::text(section)).await {
                    section_item.click().await?;
                    println!("Opened {} settings", section);
                    
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    
                    // Go back
                    if let Ok(back_btn) = session.find_element(&Locator::accessibility_id("back-button")).await {
                        back_btn.click().await?;
                    }
                }
            }
        }
    }

    Ok(())
}
