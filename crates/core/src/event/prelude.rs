//! Common imports for the event system.
//!
//! This module re-exports the most commonly used types and traits
//! from the event system for convenience.

pub use crate::event::{
    bus::{EmitReport, EventBus, EventBusBuilder, EventHandler, HandlerResult, ResultEventHandler},
    metrics::{MetricsSink, MetricsSinkRef, NoopMetricsSink},
    payload::{EmptyPayload, EventPayload, JsonPayload, TextPayload},
    subscription::Subscription,
};

// Re-export the event_emit macro
pub use crate::event_emit;