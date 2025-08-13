//! Event system for ZenTerm.
//!
//! This module provides a robust, flexible event bus system with support for:
//! - Pattern-based event routing with wildcard support
//! - Panic isolation and recovery
//! - Metrics collection and observability
//! - Optional tracing integration
//! - Both simple and result-returning handlers
//!
//! # Quick Start
//!
//! ```rust
//! use zenterm_core::event::prelude::*;
//!
//! let bus = EventBus::new();
//!
//! // Subscribe to events
//! let subscription = bus.subscribe("user.login", |payload| {
//!     println!("User logged in!");
//! });
//!
//! // Emit events
//! bus.emit("user.login", &EmptyPayload);
//! ```
//!
//! # Pattern Matching
//!
//! The event system supports sophisticated pattern matching:
//! - `user.login` - Exact match
//! - `user.*` - Single segment wildcard (matches `user.login`, not `user.login.success`)
//! - `user.**` - Multi-segment wildcard (matches `user.login`, `user.login.success`, etc.)
//! - `**` - Root wildcard (matches everything)
//!
//! # Advanced Usage
//!
//! ```rust
//! use zenterm_core::event::prelude::*;
//!
//! let bus = EventBus::builder()
//!     .catch_panics(true)
//!     .tracing(true)
//!     .build();
//!
//! // Handler that returns results
//! let sub = bus.subscribe_result("user.validate", |payload| {
//!     // Validation logic here
//!     Ok(())
//! });
//!
//! // Get detailed emission report
//! let report = bus.emit_and_report("user.validate", &EmptyPayload);
//! if !report.is_all_ok() {
//!     println!("Some handlers failed: {:?}", report);
//! }
//! ```

pub mod bus;
pub mod metrics;
pub mod payload;
pub mod prelude;
pub mod subscription;

// Re-export commonly used types at the module level
pub use bus::{EmitReport, EventBus, EventBusBuilder, HandlerResult};
pub use metrics::{MetricsSink, MetricsSinkRef, NoopMetricsSink};
pub use payload::{EmptyPayload, EventPayload, JsonPayload, TextPayload};
pub use subscription::Subscription;