/// Errors that can occur in the event system.
///
/// - `OverloadedEvent`: Too many listeners for an event.
/// - `ListenerNotFound`: Tried to remove or emit to a listener that does not exist.
/// - `EventNotFound`: Tried to remove or emit to an event that does not exist.
/// - `Other`: Any other error (boxed).
#[derive(Debug)]
pub enum EventError {
    /// Adding Listener:
    /// - Trying to add more than `max_listeners` to an Event.
    OverloadedEvent,

    /// Removing Listener/Emitting Event:
    /// - Trying to access a specific `Listener` that cannot be found.
    ListenerNotFound,

    /// Removing Listener/Emitting Event:
    /// Trying to access a specific `Event` that cannot be found.
    EventNotFound,

    /// Any other possible Errors during Event Handling
    #[cfg(not(feature = "threaded"))]
    Other(&'static str, u16),

    /// Any other possible Errors during Event Handling
    #[cfg(feature = "threaded")]
    Other(Box<dyn std::error::Error + Send + Sync>),
}
impl PartialEq for EventError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EventError::ListenerNotFound, EventError::ListenerNotFound) |
            (EventError::EventNotFound, EventError::EventNotFound) |
            (EventError::OverloadedEvent, EventError::OverloadedEvent) => true,
            #[cfg(not(feature = "threaded"))]
            (EventError::Other(a1, a2), EventError::Other(b1, b2)) => a1 == b1 && a2 == b2,
            _ => false,
            #[cfg(feature = "threaded")]
            (EventError::Other(a), EventError::Other(b)) => a.to_string() == b.to_string()
        }
    }
}
impl Eq for EventError {}

impl core::fmt::Display for EventError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EventError::OverloadedEvent => write!(f, "Too many listeners for event"),
            EventError::ListenerNotFound => write!(f, "Listener not found"),
            EventError::EventNotFound => write!(f, "Event not found"),
            #[cfg(not(feature = "threaded"))]
            EventError::Other(msg, code) => write!(f, "Error: {} (code {})", msg, code),
            #[cfg(feature = "threaded")]
            EventError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

#[cfg(feature = "threaded")]
impl std::error::Error for EventError {}