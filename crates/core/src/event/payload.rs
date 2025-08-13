//! Event payload system.
//!
//! This module provides flexible payload handling for events, supporting
//! both typed payloads via generics and untyped payloads via dynamic dispatch.

use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt;

/// Trait for event payloads that can be carried by events.
///
/// This trait allows for both typed and untyped payload handling,
/// enabling flexible event system design.
pub trait EventPayload: Send + Sync + fmt::Debug {
    /// Returns the payload as a trait object for dynamic dispatch.
    fn as_any(&self) -> &dyn Any;

    /// Returns the type name of the payload for debugging.
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl<T> EventPayload for T
where
    T: Send + Sync + fmt::Debug + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A simple text payload for string-based events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextPayload {
    pub content: String,
}

impl TextPayload {
    /// Creates a new text payload.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl From<String> for TextPayload {
    fn from(content: String) -> Self {
        Self::new(content)
    }
}

impl From<&str> for TextPayload {
    fn from(content: &str) -> Self {
        Self::new(content)
    }
}

/// A JSON payload for structured data events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonPayload {
    pub data: serde_json::Value,
}

impl JsonPayload {
    /// Creates a new JSON payload from a serializable value.
    pub fn new<T: Serialize>(value: &T) -> Result<Self, serde_json::Error> {
        Ok(Self {
            data: serde_json::to_value(value)?,
        })
    }

    /// Creates a new JSON payload from a JSON value.
    pub fn from_value(data: serde_json::Value) -> Self {
        Self { data }
    }

    /// Deserializes the payload into a specific type.
    pub fn deserialize<T: for<'de> Deserialize<'de>>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.data.clone())
    }
}

/// An empty payload for events that don't need to carry data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct EmptyPayload;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_payload() {
        let payload = TextPayload::new("test message");
        assert_eq!(payload.content, "test message");

        let payload: TextPayload = "another test".into();
        assert_eq!(payload.content, "another test");
    }

    #[test]
    fn test_json_payload() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestData {
            value: i32,
            name: String,
        }

        let data = TestData {
            value: 42,
            name: "test".to_string(),
        };

        let payload = JsonPayload::new(&data).unwrap();
        let recovered: TestData = payload.deserialize().unwrap();

        assert_eq!(data, recovered);
    }

    #[test]
    fn test_empty_payload() {
        let payload = EmptyPayload;
        assert_eq!(payload, EmptyPayload::default());
    }

    #[test]
    fn test_event_payload_trait() {
        let text_payload = TextPayload::new("test");
        let json_payload = JsonPayload::from_value(serde_json::json!({"test": true}));
        let empty_payload = EmptyPayload;

        // Test trait object creation
        let payloads: Vec<&dyn EventPayload> = vec![&text_payload, &json_payload, &empty_payload];

        for payload in payloads {
            // Each payload should have a type name
            assert!(!payload.type_name().is_empty());
            // Each payload should be convertible to Any
            assert!(payload.as_any().is::<TextPayload>() || 
                   payload.as_any().is::<JsonPayload>() || 
                   payload.as_any().is::<EmptyPayload>());
        }
    }
}