//! Event bus implementation.
//!
//! This module provides the core event bus functionality, including event emission,
//! subscription management, panic recovery, and metrics collection.

use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, RwLock};

use crate::event::{
    metrics::{MetricsSink, MetricsSinkRef, NoopMetricsSink},
    payload::EventPayload,
    subscription::Subscription,
};

/// Result of handler execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandlerResult {
    /// Handler completed successfully
    Success,
    /// Handler returned an error
    Error(String),
    /// Handler panicked and was recovered
    Panic(String),
}

impl HandlerResult {
    /// Returns true if the result represents a successful execution.
    pub fn is_success(&self) -> bool {
        matches!(self, HandlerResult::Success)
    }

    /// Returns true if the result represents an error.
    pub fn is_error(&self) -> bool {
        matches!(self, HandlerResult::Error(_))
    }

    /// Returns true if the result represents a panic.
    pub fn is_panic(&self) -> bool {
        matches!(self, HandlerResult::Panic(_))
    }
}

/// Report of event emission results.
#[derive(Debug, Clone)]
pub struct EmitReport {
    /// Number of handlers that executed successfully
    pub success_count: usize,
    /// Number of handlers that returned errors
    pub error_count: usize,
    /// Number of handlers that panicked
    pub panic_count: usize,
    /// Total number of handlers that were invoked
    pub total_count: usize,
}

impl EmitReport {
    /// Creates a new empty emit report.
    pub fn new() -> Self {
        Self {
            success_count: 0,
            error_count: 0,
            panic_count: 0,
            total_count: 0,
        }
    }

    /// Returns true if all handlers executed successfully.
    pub fn is_all_ok(&self) -> bool {
        self.error_count == 0 && self.panic_count == 0
    }

    /// Adds a handler result to this report.
    pub fn add_result(&mut self, result: &HandlerResult) {
        self.total_count += 1;
        match result {
            HandlerResult::Success => self.success_count += 1,
            HandlerResult::Error(_) => self.error_count += 1,
            HandlerResult::Panic(_) => self.panic_count += 1,
        }
    }
}

impl Default for EmitReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Type alias for event handlers that don't return results.
pub type EventHandler = Box<dyn Fn(&dyn EventPayload) + Send + Sync>;

/// Type alias for event handlers that return results.
pub type ResultEventHandler = Box<dyn Fn(&dyn EventPayload) -> Result<(), String> + Send + Sync>;

/// Storage for a subscription and its handler.
struct HandlerEntry {
    subscription: Subscription,
    handler: HandlerType,
}

/// Enum to store different types of handlers.
enum HandlerType {
    Simple(EventHandler),
    Result(ResultEventHandler),
}

/// Builder for configuring an EventBus.
pub struct EventBusBuilder {
    catch_panics: bool,
    tracing_enabled: bool,
    metrics_sink: MetricsSinkRef,
}

impl Default for EventBusBuilder {
    fn default() -> Self {
        Self {
            catch_panics: true,
            tracing_enabled: false,
            metrics_sink: Arc::new(NoopMetricsSink),
        }
    }
}

impl EventBusBuilder {
    /// Creates a new event bus builder with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether to catch panics in handlers (default: true).
    pub fn catch_panics(mut self, enabled: bool) -> Self {
        self.catch_panics = enabled;
        self
    }

    /// Sets whether to enable tracing (default: false).
    /// 
    /// Note: Requires the "event-tracing" feature to be enabled.
    pub fn tracing(mut self, enabled: bool) -> Self {
        self.tracing_enabled = enabled;
        self
    }

    /// Sets a custom metrics sink for collecting event bus metrics.
    pub fn metrics<T: MetricsSink + 'static>(mut self, sink: T) -> Self {
        self.metrics_sink = Arc::new(sink);
        self
    }

    /// Builds the event bus with the configured settings.
    pub fn build(self) -> EventBus {
        EventBus {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            catch_panics: self.catch_panics,
            tracing_enabled: self.tracing_enabled,
            metrics_sink: self.metrics_sink,
        }
    }
}

/// The main event bus for handling event emission and subscription.
pub struct EventBus {
    handlers: Arc<RwLock<HashMap<String, HandlerEntry>>>,
    catch_panics: bool,
    tracing_enabled: bool,
    metrics_sink: MetricsSinkRef,
}

impl EventBus {
    /// Creates a new event bus with default settings.
    pub fn new() -> Self {
        EventBusBuilder::default().build()
    }

    /// Creates a new event bus builder for customization.
    pub fn builder() -> EventBusBuilder {
        EventBusBuilder::new()
    }

    /// Subscribes to events matching the given pattern with a simple handler.
    ///
    /// # Arguments
    /// * `pattern` - The event pattern to subscribe to
    /// * `handler` - The handler function to call when matching events are emitted
    ///
    /// # Returns
    /// A subscription that can be used to unsubscribe later
    pub fn subscribe<F>(&self, pattern: impl Into<String>, handler: F) -> Subscription
    where
        F: Fn(&dyn EventPayload) + Send + Sync + 'static,
    {
        let subscription = Subscription::new(pattern);
        let handler_entry = HandlerEntry {
            subscription: subscription.clone(),
            handler: HandlerType::Simple(Box::new(handler)),
        };

        let mut handlers = self.handlers.write().unwrap();
        handlers.insert(subscription.id().to_string(), handler_entry);

        subscription
    }

    /// Subscribes to events matching the given pattern with a result-returning handler.
    ///
    /// # Arguments
    /// * `pattern` - The event pattern to subscribe to
    /// * `handler` - The handler function that returns a Result
    ///
    /// # Returns
    /// A subscription that can be used to unsubscribe later
    pub fn subscribe_result<F>(&self, pattern: impl Into<String>, handler: F) -> Subscription
    where
        F: Fn(&dyn EventPayload) -> Result<(), String> + Send + Sync + 'static,
    {
        let subscription = Subscription::new(pattern);
        let handler_entry = HandlerEntry {
            subscription: subscription.clone(),
            handler: HandlerType::Result(Box::new(handler)),
        };

        let mut handlers = self.handlers.write().unwrap();
        handlers.insert(subscription.id().to_string(), handler_entry);

        subscription
    }

    /// Unsubscribes from events using the given subscription.
    ///
    /// # Arguments
    /// * `subscription` - The subscription to remove
    ///
    /// # Returns
    /// `true` if the subscription was found and removed, `false` otherwise
    pub fn unsubscribe(&self, subscription: &Subscription) -> bool {
        let mut handlers = self.handlers.write().unwrap();
        handlers.remove(subscription.id()).is_some()
    }

    /// Emits an event to all matching subscribers.
    ///
    /// This is the basic emit method that doesn't return detailed results.
    ///
    /// # Arguments
    /// * `pattern` - The event pattern
    /// * `payload` - The event payload
    pub fn emit(&self, pattern: &str, payload: &dyn EventPayload) {
        let _ = self.emit_and_report(pattern, payload);
    }

    /// Emits an event and returns the count of handlers that processed it.
    ///
    /// # Arguments
    /// * `pattern` - The event pattern
    /// * `payload` - The event payload
    ///
    /// # Returns
    /// The number of handlers that processed the event
    pub fn emit_and_count(&self, pattern: &str, payload: &dyn EventPayload) -> usize {
        self.emit_and_report(pattern, payload).total_count
    }

    /// Emits an event and returns a detailed report of the results.
    ///
    /// # Arguments
    /// * `pattern` - The event pattern
    /// * `payload` - The event payload
    ///
    /// # Returns
    /// A detailed report of handler execution results
    pub fn emit_and_report(&self, pattern: &str, payload: &dyn EventPayload) -> EmitReport {
        let handlers = self.handlers.read().unwrap();
        let matching_handlers: Vec<_> = handlers
            .values()
            .filter(|entry| entry.subscription.matches(pattern))
            .collect();

        let handler_count = matching_handlers.len();
        self.metrics_sink.on_emit(pattern, handler_count);

        let mut report = EmitReport::new();

        // Add tracing span if enabled
        #[cfg(feature = "event-tracing")]
        let _span = if self.tracing_enabled {
            Some(tracing::info_span!("event_emit", pattern = pattern, handler_count = handler_count).entered())
        } else {
            None
        };

        for entry in matching_handlers {
            let result = self.execute_handler(&entry.handler, payload);
            self.metrics_sink.on_handler_result(pattern, entry.subscription.id(), &result);
            report.add_result(&result);
        }

        report
    }

    /// Executes a single handler with panic recovery if enabled.
    fn execute_handler(&self, handler: &HandlerType, payload: &dyn EventPayload) -> HandlerResult {
        if self.catch_panics {
            match catch_unwind(AssertUnwindSafe(|| self.call_handler(handler, payload))) {
                Ok(result) => result,
                Err(panic) => {
                    let error_msg = if let Some(s) = panic.downcast_ref::<String>() {
                        s.clone()
                    } else if let Some(s) = panic.downcast_ref::<&str>() {
                        s.to_string()
                    } else {
                        "Unknown panic".to_string()
                    };
                    HandlerResult::Panic(error_msg)
                }
            }
        } else {
            self.call_handler(handler, payload)
        }
    }

    /// Calls the actual handler function.
    fn call_handler(&self, handler: &HandlerType, payload: &dyn EventPayload) -> HandlerResult {
        match handler {
            HandlerType::Simple(h) => {
                h(payload);
                HandlerResult::Success
            }
            HandlerType::Result(h) => match h(payload) {
                Ok(()) => HandlerResult::Success,
                Err(e) => HandlerResult::Error(e),
            },
        }
    }

    /// Emits an event synchronously in sequential order.
    ///
    /// This is an alias for `emit_and_report` for backward compatibility.
    ///
    /// # Arguments
    /// * `pattern` - The event pattern
    /// * `payload` - The event payload
    ///
    /// # Returns
    /// A detailed report of handler execution results
    pub fn emit_sync_sequential(&self, pattern: &str, payload: &dyn EventPayload) -> EmitReport {
        self.emit_and_report(pattern, payload)
    }

    /// Deprecated alias for `emit_and_report`.
    ///
    /// # Deprecated
    /// Use `emit_and_report` instead.
    #[deprecated(note = "Use emit_and_report instead")]
    pub fn emit_wait(&self, pattern: &str, payload: &dyn EventPayload) -> EmitReport {
        self.emit_and_report(pattern, payload)
    }

    /// Returns the number of active subscriptions.
    pub fn subscription_count(&self) -> usize {
        self.handlers.read().unwrap().len()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience macro for emitting events.
///
/// Supports several forms:
/// - `event_emit!(bus, "pattern")` - Emit with empty payload
/// - `event_emit!(bus, "pattern", text: "message")` - Emit with text payload
/// - `event_emit!(bus, "pattern", json: value)` - Emit with JSON payload
/// - `event_emit!(bus, "pattern", payload)` - Emit with custom payload
#[macro_export]
macro_rules! event_emit {
    ($bus:expr, $pattern:expr) => {
        $bus.emit($pattern, &$crate::event::payload::EmptyPayload)
    };
    ($bus:expr, $pattern:expr, text: $text:expr) => {
        $bus.emit($pattern, &$crate::event::payload::TextPayload::new($text))
    };
    ($bus:expr, $pattern:expr, json: $value:expr) => {
        match $crate::event::payload::JsonPayload::new(&$value) {
            Ok(payload) => $bus.emit($pattern, &payload),
            Err(e) => eprintln!("Failed to create JSON payload: {}", e),
        }
    };
    ($bus:expr, $pattern:expr, $payload:expr) => {
        $bus.emit($pattern, &$payload)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::payload::{EmptyPayload, TextPayload};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_event_bus_creation() {
        let bus = EventBus::new();
        assert_eq!(bus.subscription_count(), 0);
    }

    #[test]
    fn test_subscribe_and_emit() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let _sub = bus.subscribe("test.event", move |_payload| {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });

        assert_eq!(bus.subscription_count(), 1);

        let payload = TextPayload::new("test message");
        bus.emit("test.event", &payload);

        assert_eq!(counter.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_subscribe_result() {
        let bus = EventBus::new();
        
        let _sub = bus.subscribe_result("test.result", |_payload| {
            Ok(())
        });

        let report = bus.emit_and_report("test.result", &EmptyPayload);
        assert_eq!(report.success_count, 1);
        assert_eq!(report.error_count, 0);
        assert_eq!(report.panic_count, 0);
    }

    #[test]
    fn test_subscribe_result_error() {
        let bus = EventBus::new();
        
        let _sub = bus.subscribe_result("test.error", |_payload| {
            Err("Handler error".to_string())
        });

        let report = bus.emit_and_report("test.error", &EmptyPayload);
        assert_eq!(report.success_count, 0);
        assert_eq!(report.error_count, 1);
        assert_eq!(report.panic_count, 0);
        assert!(!report.is_all_ok());
    }

    #[test]
    fn test_panic_recovery() {
        let bus = EventBus::builder().catch_panics(true).build();
        
        let _sub = bus.subscribe("test.panic", |_payload| {
            panic!("Handler panic");
        });

        let report = bus.emit_and_report("test.panic", &EmptyPayload);
        assert_eq!(report.success_count, 0);
        assert_eq!(report.error_count, 0);
        assert_eq!(report.panic_count, 1);
        assert!(!report.is_all_ok());
    }

    #[test]
    fn test_wildcard_patterns() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let _sub = bus.subscribe("user.*", move |_payload| {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });

        bus.emit("user.login", &EmptyPayload);
        bus.emit("user.logout", &EmptyPayload);
        bus.emit("admin.login", &EmptyPayload);

        assert_eq!(counter.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn test_multi_wildcard_patterns() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let _sub = bus.subscribe("user.**", move |_payload| {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });

        bus.emit("user.login", &EmptyPayload);
        bus.emit("user.profile.update", &EmptyPayload);
        bus.emit("admin.login", &EmptyPayload);

        assert_eq!(counter.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn test_root_wildcard() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let _sub = bus.subscribe("**", move |_payload| {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });

        bus.emit("user.login", &EmptyPayload);
        bus.emit("admin.logout", &EmptyPayload);
        bus.emit("system.shutdown", &EmptyPayload);

        assert_eq!(counter.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn test_unsubscribe() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let sub = bus.subscribe("test.event", move |_payload| {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });

        assert_eq!(bus.subscription_count(), 1);

        bus.emit("test.event", &EmptyPayload);
        assert_eq!(counter.load(Ordering::Relaxed), 1);

        assert!(bus.unsubscribe(&sub));
        assert_eq!(bus.subscription_count(), 0);

        bus.emit("test.event", &EmptyPayload);
        assert_eq!(counter.load(Ordering::Relaxed), 1); // Should not increment
    }

    #[test]
    fn test_emit_and_count() {
        let bus = EventBus::new();

        let _sub1 = bus.subscribe("test.event", |_| {});
        let _sub2 = bus.subscribe("test.*", |_| {});

        let count = bus.emit_and_count("test.event", &EmptyPayload);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_emit_report() {
        let bus = EventBus::new();

        let _sub1 = bus.subscribe("test.event", |_| {});
        let _sub2 = bus.subscribe_result("test.event", |_| Ok(()));
        let _sub3 = bus.subscribe_result("test.event", |_| Err("error".to_string()));

        let report = bus.emit_and_report("test.event", &EmptyPayload);
        assert_eq!(report.total_count, 3);
        assert_eq!(report.success_count, 2);
        assert_eq!(report.error_count, 1);
        assert_eq!(report.panic_count, 0);
        assert!(!report.is_all_ok());
    }

    #[test]
    fn test_event_emit_macro() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let _sub = bus.subscribe("**", move |_| {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });

        // Test different macro forms
        event_emit!(bus, "test.empty");
        event_emit!(bus, "test.text", text: "message");
        event_emit!(bus, "test.custom", EmptyPayload);

        assert_eq!(counter.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn test_builder_pattern() {
        let bus = EventBus::builder()
            .catch_panics(false)
            .tracing(true)
            .build();

        // Just verify it builds without errors
        assert_eq!(bus.subscription_count(), 0);
    }
}