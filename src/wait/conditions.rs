use crate::{element::{Element, Locator}, error::Result, session::Session};
use std::time::Duration;

/// Standard wait conditions for common automation scenarios
pub struct WaitConditions;

impl WaitConditions {
    /// Waits for an element to be present in the DOM/UI tree
    pub async fn element_present(
        session: &Session,
        locator: &Locator,
        timeout: Duration,
    ) -> Result<Element> {
        crate::wait::wait_for_element(session, locator, timeout).await
    }

    /// Waits for an element to be visible
    pub async fn element_visible(
        session: &Session,
        locator: &Locator,
        timeout: Duration,
    ) -> Result<Element> {
        crate::wait::wait_for_element_visible(session, locator, timeout).await
    }

    /// Waits for an element to be clickable (visible and enabled)
    pub async fn element_clickable(
        session: &Session,
        locator: &Locator,
        timeout: Duration,
    ) -> Result<Element> {
        crate::wait::wait_for_element_clickable(session, locator, timeout).await
    }

    /// Waits for an element to be invisible or not present
    pub async fn element_not_visible(
        session: &Session,
        locator: &Locator,
        timeout: Duration,
    ) -> Result<()> {
        crate::wait::wait_for_condition(
            timeout,
            crate::wait::WaitStrategy::default(),
            || async {
                match session.find_element(locator).await {
                    Ok(element) => {
                        if !element.is_visible() {
                            Ok(())
                        } else {
                            Err(crate::error::BryndzaError::timeout(
                                Duration::from_millis(0),
                                "Element still visible"
                            ))
                        }
                    }
                    Err(crate::error::BryndzaError::ElementNotFound { .. }) => Ok(()),
                    Err(e) => Err(e),
                }
            },
        ).await
    }

    /// Waits for an element to contain specific text
    pub async fn element_text_contains(
        session: &Session,
        locator: &Locator,
        expected_text: &str,
        timeout: Duration,
    ) -> Result<Element> {
        let expected_text = expected_text.to_string();
        crate::wait::wait_for_condition(
            timeout,
            crate::wait::WaitStrategy::default(),
            || {
                let expected_text = expected_text.clone();
                async move {
                    let element = session.find_element(locator).await?;
                    if let Some(text) = element.text() {
                        if text.contains(&expected_text) {
                            Ok(element)
                        } else {
                            Err(crate::error::BryndzaError::timeout(
                                Duration::from_millis(0),
                                format!("Element text '{}' does not contain '{}'", text, expected_text)
                            ))
                        }
                    } else {
                        Err(crate::error::BryndzaError::timeout(
                            Duration::from_millis(0),
                            "Element has no text content"
                        ))
                    }
                }
            },
        ).await
    }

    /// Waits for an element's text to match exactly
    pub async fn element_text_equals(
        session: &Session,
        locator: &Locator,
        expected_text: &str,
        timeout: Duration,
    ) -> Result<Element> {
        let expected_text = expected_text.to_string();
        crate::wait::wait_for_condition(
            timeout,
            crate::wait::WaitStrategy::default(),
            || {
                let expected_text = expected_text.clone();
                async move {
                    let element = session.find_element(locator).await?;
                    if let Some(text) = element.text() {
                        if text == expected_text {
                            Ok(element)
                        } else {
                            Err(crate::error::BryndzaError::timeout(
                                Duration::from_millis(0),
                                format!("Element text '{}' does not equal '{}'", text, expected_text)
                            ))
                        }
                    } else {
                        Err(crate::error::BryndzaError::timeout(
                            Duration::from_millis(0),
                            "Element has no text content"
                        ))
                    }
                }
            },
        ).await
    }

    /// Waits for an element's attribute to have a specific value
    pub async fn element_attribute_equals(
        session: &Session,
        locator: &Locator,
        attribute_name: &str,
        expected_value: &str,
        timeout: Duration,
    ) -> Result<Element> {
        let attribute_name = attribute_name.to_string();
        let expected_value = expected_value.to_string();
        crate::wait::wait_for_condition(
            timeout,
            crate::wait::WaitStrategy::default(),
            || {
                let attribute_name = attribute_name.clone();
                let expected_value = expected_value.clone();
                async move {
                    let element = session.find_element(locator).await?;
                    if let Some(value) = element.attribute(&attribute_name) {
                        if value == expected_value {
                            Ok(element)
                        } else {
                            Err(crate::error::BryndzaError::timeout(
                                Duration::from_millis(0),
                                format!("Attribute '{}' value '{}' does not equal '{}'", attribute_name, value, expected_value)
                            ))
                        }
                    } else {
                        Err(crate::error::BryndzaError::timeout(
                            Duration::from_millis(0),
                            format!("Element does not have attribute '{}'", attribute_name)
                        ))
                    }
                }
            },
        ).await
    }

    /// Waits for a specific number of elements to be present
    pub async fn element_count_equals(
        session: &Session,
        locator: &Locator,
        expected_count: usize,
        timeout: Duration,
    ) -> Result<Vec<Element>> {
        crate::wait::wait_for_condition(
            timeout,
            crate::wait::WaitStrategy::default(),
            || async {
                let elements = session.find_elements(locator).await?;
                if elements.len() == expected_count {
                    Ok(elements)
                } else {
                    Err(crate::error::BryndzaError::timeout(
                        Duration::from_millis(0),
                        format!("Expected {} elements, found {}", expected_count, elements.len())
                    ))
                }
            },
        ).await
    }

    /// Waits for at least a minimum number of elements to be present
    pub async fn element_count_at_least(
        session: &Session,
        locator: &Locator,
        min_count: usize,
        timeout: Duration,
    ) -> Result<Vec<Element>> {
        crate::wait::wait_for_condition(
            timeout,
            crate::wait::WaitStrategy::default(),
            || async {
                let elements = session.find_elements(locator).await?;
                if elements.len() >= min_count {
                    Ok(elements)
                } else {
                    Err(crate::error::BryndzaError::timeout(
                        Duration::from_millis(0),
                        format!("Expected at least {} elements, found {}", min_count, elements.len())
                    ))
                }
            },
        ).await
    }

    /// Waits for a custom condition function to return true
    pub async fn custom_condition<F, Fut>(
        condition: F,
        timeout: Duration,
    ) -> Result<()>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<bool>>,
    {
        crate::wait::wait_for_condition(
            timeout,
            crate::wait::WaitStrategy::default(),
            || async {
                if condition().await? {
                    Ok(())
                } else {
                    Err(crate::error::BryndzaError::timeout(
                        Duration::from_millis(0),
                        "Custom condition not met"
                    ))
                }
            },
        ).await
    }
}
