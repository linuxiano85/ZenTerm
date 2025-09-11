use crossbeam_channel::{Receiver, Sender, unbounded};
use log::debug;

/// Event bus for inter-component communication using crossbeam channels
#[derive(Debug, Clone)]
pub struct EventBus {
    sender: Sender<AppEvent>,
    receiver: Receiver<AppEvent>,
}

/// Events that can be sent through the event bus
#[derive(Debug, Clone)]
pub enum AppEvent {
    GpuLimitChanged(u8),
    ThemeToggled(bool), // true = dark mode
    VoiceToggled(bool), // true = enabled
    WizardOpened,
    WizardClosed,
    ConfigSaveRequested,
    LogMessage(String),
    QuitRequested,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self { sender, receiver }
    }
    
    /// Get a sender for publishing events
    pub fn sender(&self) -> Sender<AppEvent> {
        self.sender.clone()
    }
    
    /// Get a receiver for consuming events
    pub fn receiver(&self) -> Receiver<AppEvent> {
        self.receiver.clone()
    }
    
    /// Send an event to the bus
    pub fn send(&self, event: AppEvent) -> Result<(), crossbeam_channel::SendError<AppEvent>> {
        debug!("Sending event: {:?}", event);
        self.sender.send(event)
    }
    
    /// Try to receive an event from the bus (non-blocking)
    pub fn try_recv(&self) -> Result<AppEvent, crossbeam_channel::TryRecvError> {
        self.receiver.try_recv()
    }
    
    /// Receive an event from the bus (blocking)
    pub fn recv(&self) -> Result<AppEvent, crossbeam_channel::RecvError> {
        self.receiver.recv()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_bus_basic_send_receive() {
        let event_bus = EventBus::new();
        let sender = event_bus.sender();
        
        // Send an event
        let test_event = AppEvent::GpuLimitChanged(50);
        sender.send(test_event.clone()).unwrap();
        
        // Receive the event
        let received_event = event_bus.try_recv().unwrap();
        
        // Verify it's the same event (we can't use PartialEq directly, so match)
        match (test_event, received_event) {
            (AppEvent::GpuLimitChanged(a), AppEvent::GpuLimitChanged(b)) => assert_eq!(a, b),
            _ => panic!("Events don't match"),
        }
    }
    
    #[test]
    fn test_event_bus_multiple_events() {
        let event_bus = EventBus::new();
        let sender = event_bus.sender();
        
        // Send multiple events
        sender.send(AppEvent::GpuLimitChanged(25)).unwrap();
        sender.send(AppEvent::ThemeToggled(true)).unwrap();
        sender.send(AppEvent::VoiceToggled(false)).unwrap();
        
        // Receive events in order
        match event_bus.try_recv().unwrap() {
            AppEvent::GpuLimitChanged(25) => {},
            _ => panic!("First event should be GpuLimitChanged(25)"),
        }
        
        match event_bus.try_recv().unwrap() {
            AppEvent::ThemeToggled(true) => {},
            _ => panic!("Second event should be ThemeToggled(true)"),
        }
        
        match event_bus.try_recv().unwrap() {
            AppEvent::VoiceToggled(false) => {},
            _ => panic!("Third event should be VoiceToggled(false)"),
        }
        
        // No more events should be available
        assert!(event_bus.try_recv().is_err());
    }
    
    #[test]
    fn test_event_bus_clone_sender() {
        let event_bus = EventBus::new();
        let sender1 = event_bus.sender();
        let sender2 = sender1.clone();
        
        // Both senders should work
        sender1.send(AppEvent::LogMessage("From sender1".to_string())).unwrap();
        sender2.send(AppEvent::LogMessage("From sender2".to_string())).unwrap();
        
        // Should receive both messages
        let msg1 = event_bus.try_recv().unwrap();
        let msg2 = event_bus.try_recv().unwrap();
        
        match (msg1, msg2) {
            (AppEvent::LogMessage(m1), AppEvent::LogMessage(m2)) => {
                assert!(m1 == "From sender1" || m1 == "From sender2");
                assert!(m2 == "From sender1" || m2 == "From sender2");
                assert_ne!(m1, m2);
            },
            _ => panic!("Should receive LogMessage events"),
        }
    }
}