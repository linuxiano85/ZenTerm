# ZenTerm Event System

A robust, flexible event bus system designed for the ZenTerm terminal emulator. This system provides pattern-based event routing, panic isolation, metrics collection, and optional tracing support.

## Milestone 1 - Core Features ✅

The event system has successfully implemented all Milestone 1 features:

### ✅ Robustness & Safety
- **Panic Isolation**: Handlers are wrapped in `catch_unwind` by default to prevent crashes
- **Error Handling**: Support for handlers that return `Result<(), String>`
- **Graceful Degradation**: Failed handlers don't affect other handlers

### ✅ Pattern Matching
- **Exact Matching**: `user.login` matches only `user.login`
- **Single Wildcards**: `user.*` matches `user.login` but not `user.login.success`
- **Multi Wildcards**: `user.**` matches `user.login`, `user.login.success`, etc.
- **Root Wildcard**: `**` matches all events

### ✅ Observability
- **Metrics Collection**: Pluggable metrics system via `MetricsSink` trait
- **Execution Reports**: Detailed reports with success/error/panic counts
- **Optional Tracing**: Integration with the `tracing` crate (feature-gated)

### ✅ Builder Pattern
- **Configurable Construction**: `EventBus::builder()` with fluent API
- **Sensible Defaults**: Panic catching enabled, tracing disabled, no-op metrics
- **Customizable**: Custom metrics sinks, tracing control, panic handling

### ✅ Convenience Features
- **Emit Macro**: `event_emit!` with support for text, JSON, and custom payloads
- **Multiple Emit Methods**: `emit`, `emit_and_count`, `emit_and_report`
- **Backward Compatibility**: Deprecated methods maintained for compatibility

## Quick Start

```rust
use zenterm_core::event::prelude::*;

// Create an event bus
let bus = EventBus::new();

// Subscribe to events
let subscription = bus.subscribe("user.login", |payload| {
    if let Some(text) = payload.as_any().downcast_ref::<TextPayload>() {
        println!("User logged in: {}", text.content);
    }
});

// Emit events
event_emit!(bus, "user.login", text: "john_doe");

// Or use the direct API
let payload = TextPayload::new("jane_doe");
bus.emit("user.login", &payload);
```

## Advanced Usage

### Custom Configuration

```rust
use zenterm_core::event::prelude::*;

let bus = EventBus::builder()
    .catch_panics(true)          // Enable panic recovery (default)
    .tracing(true)               // Enable tracing spans (requires feature)
    .metrics(MyCustomSink::new()) // Custom metrics collection
    .build();
```

### Error Handling

```rust
// Subscribe with result-returning handler
let sub = bus.subscribe_result("data.validate", |payload| {
    // Validation logic
    if is_valid(payload) {
        Ok(())
    } else {
        Err("Validation failed".to_string())
    }
});

// Get detailed execution report
let report = bus.emit_and_report("data.validate", &payload);
if !report.is_all_ok() {
    eprintln!("Validation errors: {} errors, {} panics", 
              report.error_count, report.panic_count);
}
```

### Custom Metrics

```rust
use zenterm_core::event::prelude::*;

struct MyMetrics;

impl MetricsSink for MyMetrics {
    fn on_emit(&self, pattern: &str, handler_count: usize) {
        println!("Emitted '{}' to {} handlers", pattern, handler_count);
    }
    
    fn on_handler_result(&self, pattern: &str, handler_id: &str, result: &HandlerResult) {
        match result {
            HandlerResult::Success => println!("✓ {}/{}", pattern, handler_id),
            HandlerResult::Error(e) => println!("✗ {}/{}: {}", pattern, handler_id, e),
            HandlerResult::Panic(e) => println!("💥 {}/{}: {}", pattern, handler_id, e),
        }
    }
}

let bus = EventBus::builder().metrics(MyMetrics).build();
```

## Pattern Matching Examples

| Subscription Pattern | Event Pattern | Matches? | Reason |
|---------------------|---------------|----------|---------|
| `user.login` | `user.login` | ✅ | Exact match |
| `user.login` | `user.logout` | ❌ | Different event |
| `user.*` | `user.login` | ✅ | Single wildcard |
| `user.*` | `user.login.success` | ❌ | Too many segments |
| `user.**` | `user.login` | ✅ | Multi wildcard |
| `user.**` | `user.login.success` | ✅ | Multi wildcard |
| `**` | `anything.goes.here` | ✅ | Root wildcard |

## Features

The event system supports optional features via Cargo:

- `custom-payload` (default): Support for custom payload types
- `event-tracing`: Integration with the `tracing` crate for observability

```toml
[dependencies]
zenterm-core = { version = "0.1", features = ["event-tracing"] }
```

## Performance

The event system is designed for minimal overhead:

- **No-op Metrics**: Default metrics sink has zero cost
- **Feature Gating**: Tracing code is compiled out when feature is disabled
- **Efficient Matching**: Simple string-based pattern matching
- **Minimal Allocations**: Event handlers stored efficiently

## Security & Robustness

### Panic Isolation
By default, all handlers are wrapped in `catch_unwind` to prevent panics from crashing the application:

```rust
// This handler's panic won't crash the system
let sub = bus.subscribe("dangerous.operation", |_| {
    panic!("Something went wrong!");
});

// The panic is caught and reported
let report = bus.emit_and_report("dangerous.operation", &EmptyPayload);
assert_eq!(report.panic_count, 1);
```

### Error Reporting
Handlers can return errors which are collected and reported:

```rust
let sub = bus.subscribe_result("validate.input", |payload| {
    if validate(payload) {
        Ok(())
    } else {
        Err("Invalid input".to_string())
    }
});
```

## Future Roadmap

### Milestone 2 - Performance & Scalability
- [ ] Pattern precompilation for faster matching
- [ ] Benchmarking suite and performance optimizations
- [ ] Priority-based handler execution
- [ ] Backpressure handling for high-volume scenarios

### Milestone 3 - Advanced Features
- [ ] Typed event bus wrappers for compile-time safety
- [ ] Event replay and history mechanisms
- [ ] Federation support for distributed events
- [ ] Interceptor chains for cross-cutting concerns

### Milestone 4 - Enterprise Features
- [ ] Persistent event storage
- [ ] Event schema validation
- [ ] Advanced security and access control
- [ ] Monitoring and alerting integration

## Testing

The event system includes comprehensive tests covering:

- Basic subscription and emission
- Pattern matching (exact, single wildcard, multi wildcard, root wildcard)
- Error handling and panic recovery
- Metrics collection
- Builder pattern configuration
- Macro usage

Run tests with:

```bash
cd crates/core
cargo test event
```

## Contributing

When contributing to the event system:

1. Ensure all existing tests pass
2. Add tests for new functionality
3. Update documentation for API changes
4. Consider backward compatibility
5. Benchmark performance-critical changes

## License

This event system is part of the ZenTerm project and follows the same licensing terms.