//! Metrics collection for the event system.
//!
//! This module provides traits and implementations for collecting metrics
//! about event bus operations, including handler execution results and panics.

use std::sync::Arc;

use crate::event::HandlerResult;

/// Trait for collecting metrics from the event system.
///
/// Implementations can track various aspects of event bus operation,
/// such as event emission counts, handler execution results, and panic recovery.
pub trait MetricsSink: Send + Sync {
    /// Called when an event is emitted.
    ///
    /// # Arguments
    /// * `pattern` - The event pattern that was emitted
    /// * `handler_count` - Number of handlers that will process this event
    fn on_emit(&self, _pattern: &str, _handler_count: usize) {}

    /// Called when a handler completes execution.
    ///
    /// # Arguments
    /// * `pattern` - The event pattern being handled
    /// * `handler_id` - Unique identifier for the handler
    /// * `result` - The result of handler execution
    fn on_handler_result(&self, _pattern: &str, _handler_id: &str, _result: &HandlerResult) {}

    /// Called when a handler panics and is recovered.
    ///
    /// # Arguments
    /// * `pattern` - The event pattern being handled when panic occurred
    /// * `handler_id` - Unique identifier for the handler that panicked
    /// * `error` - String representation of the panic
    fn on_panic(&self, _pattern: &str, _handler_id: &str, _error: &str) {}
}

/// No-operation implementation of MetricsSink.
///
/// This is the default metrics sink that performs no actual metric collection.
/// It's used when metrics are disabled or no custom sink is provided.
#[derive(Debug, Clone, Default)]
pub struct NoopMetricsSink;

impl MetricsSink for NoopMetricsSink {}

/// Type alias for a thread-safe reference to a metrics sink.
pub type MetricsSinkRef = Arc<dyn MetricsSink>;