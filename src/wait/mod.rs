pub mod conditions;
pub mod strategy;

pub use conditions::*;
pub use strategy::*;

use crate::{element::{Element, Locator}, error::Result, session::Session};
use std::time::Duration;

/// Waits for an element to become available
pub async fn wait_for_element(
    session: &Session,
    locator: &Locator,
    timeout: Duration,
) -> Result<Element> {
    wait_for_condition(
        timeout,
        WaitStrategy::default(),
        || async { session.find_element(locator).await },
    ).await
}

/// Waits for an element to become visible
pub async fn wait_for_element_visible(
    session: &Session,
    locator: &Locator,
    timeout: Duration,
) -> Result<Element> {
    wait_for_condition(
        timeout,
        WaitStrategy::default(),
        || async {
            let element = session.find_element(locator).await?;
            if element.is_visible() {
                Ok(element)
            } else {
                Err(crate::error::BryndzaError::element_not_interactable(
                    "Element not visible"
                ))
            }
        },
    ).await
}

/// Waits for an element to become clickable
pub async fn wait_for_element_clickable(
    session: &Session,
    locator: &Locator,
    timeout: Duration,
) -> Result<Element> {
    wait_for_condition(
        timeout,
        WaitStrategy::default(),
        || async {
            let element = session.find_element(locator).await?;
            if element.is_clickable() {
                Ok(element)
            } else {
                Err(crate::error::BryndzaError::element_not_interactable(
                    "Element not clickable"
                ))
            }
        },
    ).await
}

/// Waits for an element to disappear
pub async fn wait_for_element_not_present(
    session: &Session,
    locator: &Locator,
    timeout: Duration,
) -> Result<()> {
    wait_for_condition(
        timeout,
        WaitStrategy::default(),
        || async {
            match session.find_element(locator).await {
                Ok(_) => Err(crate::error::BryndzaError::timeout(
                    Duration::from_millis(0),
                    "Element still present"
                )),
                Err(crate::error::BryndzaError::ElementNotFound { .. }) => Ok(()),
                Err(e) => Err(e),
            }
        },
    ).await
}

/// Generic wait function that polls a condition until it succeeds or times out
pub async fn wait_for_condition<T, F, Fut>(
    timeout: Duration,
    strategy: WaitStrategy,
    condition: F,
) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let start_time = std::time::Instant::now();
    let mut attempt = 0;

    loop {
        attempt += 1;
        
        match condition().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                let elapsed = start_time.elapsed();
                
                if elapsed >= timeout {
                    return Err(crate::error::BryndzaError::timeout(
                        timeout,
                        format!("Condition not met after {} attempts", attempt)
                    ));
                }

                // Calculate next polling interval
                let next_interval = strategy.next_interval(attempt, elapsed, timeout);
                
                // Don't sleep if we would exceed the timeout
                if elapsed + next_interval >= timeout {
                    // Try one more time immediately
                    continue;
                }

                tokio::time::sleep(next_interval).await;
            }
        }
    }
}
