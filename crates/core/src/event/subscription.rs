//! Event subscription management.
//!
//! This module handles subscriptions to events, including pattern matching
//! and handler storage.

use std::sync::atomic::{AtomicUsize, Ordering};

/// Unique identifier for subscriptions.
static SUBSCRIPTION_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Represents a subscription to events matching a specific pattern.
///
/// A subscription holds information about an event handler registration,
/// including the pattern it matches and its unique identifier.
#[derive(Debug, Clone)]
pub struct Subscription {
    /// Unique identifier for this subscription
    id: String,
    /// The pattern this subscription matches
    pattern: String,
}

impl Subscription {
    /// Creates a new subscription with the given pattern.
    ///
    /// # Arguments
    /// * `pattern` - The event pattern to match
    ///
    /// # Returns
    /// A new subscription with a unique identifier
    pub fn new(pattern: impl Into<String>) -> Self {
        let id = SUBSCRIPTION_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            id: format!("sub_{}", id),
            pattern: pattern.into(),
        }
    }

    /// Returns the unique identifier of this subscription.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the pattern this subscription matches.
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// Checks if this subscription matches the given event pattern.
    ///
    /// Supports basic wildcard matching:
    /// - `*` matches any single segment
    /// - `**` matches zero or more segments
    /// - Exact string matching for non-wildcard patterns
    ///
    /// # Arguments
    /// * `event_pattern` - The pattern of the emitted event
    ///
    /// # Returns
    /// `true` if this subscription should receive the event
    pub fn matches(&self, event_pattern: &str) -> bool {
        pattern_matches(&self.pattern, event_pattern)
    }
}

/// Performs pattern matching between subscription and event patterns.
///
/// This function implements a simple pattern matching algorithm that supports:
/// - Exact matches
/// - Single segment wildcards (`*`)
/// - Multi-segment wildcards (`**`)
///
/// # Arguments
/// * `subscription_pattern` - The pattern from the subscription
/// * `event_pattern` - The pattern from the emitted event
///
/// # Returns
/// `true` if the patterns match
pub fn pattern_matches(subscription_pattern: &str, event_pattern: &str) -> bool {
    // Handle special case of root wildcard
    if subscription_pattern == "**" {
        return true;
    }

    // Exact match
    if subscription_pattern == event_pattern {
        return true;
    }

    // If no wildcards, no match possible
    if !subscription_pattern.contains('*') {
        return false;
    }

    let sub_parts: Vec<&str> = subscription_pattern.split('.').collect();
    let event_parts: Vec<&str> = event_pattern.split('.').collect();

    matches_parts(&sub_parts, &event_parts)
}

/// Recursive helper function for pattern matching.
fn matches_parts(sub_parts: &[&str], event_parts: &[&str]) -> bool {
    match (sub_parts.first(), event_parts.first()) {
        // Both empty - match
        (None, None) => true,
        // One empty, other not - no match unless subscription has **
        (None, Some(_)) => false,
        (Some(&"**"), _) => {
            // ** can match zero or more segments
            // Try matching with consuming nothing from event
            if matches_parts(&sub_parts[1..], event_parts) {
                return true;
            }
            // Try matching with consuming one segment from event
            if !event_parts.is_empty() && matches_parts(sub_parts, &event_parts[1..]) {
                return true;
            }
            false
        }
        (Some(&"*"), Some(_)) => {
            // Single wildcard matches exactly one segment
            matches_parts(&sub_parts[1..], &event_parts[1..])
        }
        (Some(&"*"), None) => false, // * requires at least one segment
        (Some(sub_part), Some(event_part)) => {
            // Exact match required for non-wildcard parts
            if sub_part == event_part {
                matches_parts(&sub_parts[1..], &event_parts[1..])
            } else {
                false
            }
        }
        (Some(_), None) => false, // Subscription expects more segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscription_creation() {
        let sub = Subscription::new("test.pattern");
        assert_eq!(sub.pattern(), "test.pattern");
        assert!(!sub.id().is_empty());
        assert!(sub.id().starts_with("sub_"));
    }

    #[test]
    fn test_subscription_ids_are_unique() {
        let sub1 = Subscription::new("pattern1");
        let sub2 = Subscription::new("pattern2");
        assert_ne!(sub1.id(), sub2.id());
    }

    #[test]
    fn test_exact_pattern_matching() {
        let sub = Subscription::new("user.login");
        assert!(sub.matches("user.login"));
        assert!(!sub.matches("user.logout"));
        assert!(!sub.matches("admin.login"));
    }

    #[test]
    fn test_single_wildcard_matching() {
        let sub = Subscription::new("user.*");
        assert!(sub.matches("user.login"));
        assert!(sub.matches("user.logout"));
        assert!(!sub.matches("admin.login"));
        assert!(!sub.matches("user.login.success"));
    }

    #[test]
    fn test_multi_wildcard_matching() {
        let sub = Subscription::new("user.**");
        assert!(sub.matches("user.login"));
        assert!(sub.matches("user.login.success"));
        assert!(sub.matches("user.profile.update.complete"));
        assert!(!sub.matches("admin.login"));
    }

    #[test]
    fn test_root_wildcard_matching() {
        let sub = Subscription::new("**");
        assert!(sub.matches("user.login"));
        assert!(sub.matches("admin.logout"));
        assert!(sub.matches("system.shutdown"));
        assert!(sub.matches("a.very.long.pattern.here"));
    }

    #[test]
    fn test_mixed_wildcard_matching() {
        let sub = Subscription::new("*.login.**");
        assert!(sub.matches("user.login.success"));
        assert!(sub.matches("admin.login.failed"));
        assert!(!sub.matches("user.logout.complete"));
        assert!(!sub.matches("login.success"));
    }

    #[test]
    fn test_pattern_matches_function() {
        // Exact matches
        assert!(pattern_matches("test", "test"));
        assert!(!pattern_matches("test", "other"));

        // Single wildcard
        assert!(pattern_matches("*", "anything"));
        assert!(pattern_matches("user.*", "user.login"));
        assert!(!pattern_matches("user.*", "user.login.success"));

        // Multi wildcard
        assert!(pattern_matches("**", "anything.goes.here"));
        assert!(pattern_matches("user.**", "user"));
        assert!(pattern_matches("user.**", "user.login"));
        assert!(pattern_matches("user.**", "user.login.success.complete"));
    }
}