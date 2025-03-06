//! Rust event emitter
//!
//! 'event_emitter' is a collection of utilities that facilitates communication between
//! different parts of an application, primarily in asynchronous environments. It
//! operates on the publish-subscribe principle, where an emitter object dispatches
//! events, and listener functions react to those events. This allows for decoupled
//! components, enhancing modularity and maintainability.

mod event_handler;
mod event_emitter;
mod listener;
#[cfg(test)]
mod tests;

use std::sync::Arc;

pub use listener::Listener;
pub use event_handler::EventHandler;
pub use event_emitter::{
    EventManager,
    EventEmitter
};

/// Type alias for a thread-safe Arc pointer. <br/>
/// Generic Wrapper for an Event Payload type. <br/>
/// (*Requires Send + Sync for use in Listener/Event Emitter*)
///
/// # Example
///
/// ```
/// use std::sync::Arc;
/// use event_emitter::{Callback, EventPayload};
///
/// let payload: EventPayload<String> = Arc::new(String::from("Emitting value"));
/// ```
pub type EventPayload<T> = Arc<T>;

/// Type alias for a thread-safe Arc pointer that wraps a function. <br/>
/// Function must take a reference to a [`EventPayload<T>`](EventPayload) with no return.
///
/// # Example
///
/// ```
/// use std::sync::Arc;
/// use event_emitter::{Callback, EventPayload};
///
/// let callback: Callback<String> = Arc::new(move |payload: &EventPayload<String>| {
///     // Use payload reference here
///     println!("Received event: {}", payload);
/// });
/// ```
pub type Callback<T> = Arc<dyn Fn(&EventPayload<T>) + Send + Sync>;

#[derive(Debug)]
/// Event Error enum for all custom and unknown error possibilities
pub enum EventError {
    /// Adding Listener:
    /// - Trying to add more than `max_listeners`to an Event.
    OverloadedEvent,
    /// Removing Listener/ Emitting Event:
    /// - Trying to access a specific `Listener` that cannot be found.
    ListenerNotFound,
    /// Removing Listener/ Emitting Event:
    /// Trying to access a specific `Event` that cannot be found.
    EventNotFound,
    /// Any other possible Errors during Event Handling
    Other(Box<dyn std::error::Error + Send + Sync>),
}
impl PartialEq for EventError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EventError::ListenerNotFound, EventError::ListenerNotFound) |
            (EventError::EventNotFound, EventError::EventNotFound) |
            (EventError::OverloadedEvent, EventError::OverloadedEvent) => {
                true
            },
            (EventError::Other(a), EventError::Other(b)) => a.to_string() == b.to_string(),
            _=> false
        }
    }
}
impl Eq for EventError {}