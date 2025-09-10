use futures::future::BoxFuture;

use crate::{Callback, EventError, EventPayload};
use crate::listener::Listener;

/// This Trait defines event-driven functionality for structs that handle event listeners and event emissions. <br/>
/// It provides methods for managing listeners, emitting events, and handling asynchronous execution. <br/>
/// Implementors of this trait can be used as event-driven components in an application. <br/>
pub trait EventHandler<T: Send + Sync>: Send + Sync {
    /// Get a Vec of names for currently active (1+ listeners) events registered.
    ///
    /// # Errors
    /// This method should not error.
    fn event_names(&self) -> Vec<String>;

    /// Set the maximum number of listeners per event.
    ///
    /// # Errors
    /// This method should not error.
    fn set_max_listeners(&mut self, max: usize);

    /// Get the current maximum number of listeners per event.
    ///
    /// # Errors
    /// This method should not error.
    fn max_listeners(&self) -> usize;


    /// Add an infinite Listener to a specific event.
    ///
    /// # Errors
    /// Returns `EventError::OverloadedEvent` if adding would exceed max listeners.
    fn add_listener(&mut self, event_name: &str, callback: Callback<T>) -> Result<Listener<T>, EventError>;

    /// Add a finite Listener to a specific event with the number of emissions it is limited to receive.
    ///
    /// # Errors
    /// Returns `EventError::OverloadedEvent` if adding would exceed max listeners.
    fn add_limited_listener(&mut self, event_name: &str, callback: Callback<T>, limit: u64) -> Result<Listener<T>, EventError>;

    /// Add a single emission Listener to a specific event.
    ///
    /// # Errors
    /// Returns `EventError::OverloadedEvent` if adding would exceed max listeners.
    fn add_once(&mut self, event_name: &str, callback: Callback<T>) -> Result<Listener<T>, EventError>;


    /// Get the number of listeners that are registered to a specific event.
    ///
    /// # Errors
    /// Returns `EventError::EventNotFound` if the event has not been registered.
    fn listener_count(&self, event_name: &str) ->  usize;

    /// Get a boolean, whether a specific event has any registered listeners.
    /// # Errors
    /// Returns `EventError::EventNotFound` if the event has not been registered.
    fn has_listener(&self, event_name: &str) ->  bool {
        self.listener_count(event_name) > 0
    }


    /// Remove a specific active listener for an Event.
    ///
    /// # Errors
    /// Returns `EventError::ListenerNotFound` if the listener is not found for the event.
    fn remove_listener(&mut self, event_name: &str, callback: &Listener<T>) -> Result<(), EventError>;

    /// Remove all Listeners for an Event.
    ///
    /// # Errors
    /// Returns `EventError::EventNotFound` if the event has not been registered.
    fn remove_all_listeners(&mut self, event_name: &str) -> Result<(), EventError>;


    /// Synchronous emission of a specific Event.
    ///
    /// # Errors
    /// Returns `EventError::EventNotFound` if the event has not been registered.
    fn emit(&mut self, event_name: &str, payload: EventPayload<T>) -> Result<(), EventError>;

    /// Synchronous emission of a specific Event for the last time.
    ///
    /// # Errors
    /// Returns `EventError::EventNotFound` if the event has not been registered.
    fn emit_final(&mut self, event_name: &str, payload: EventPayload<T>) -> Result<(), EventError>;

    /// Asynchronous emission of a specific Event.
    ///
    /// # Parameters
    /// - `parallel`: If true, listeners are called in parallel (spawned as tasks); if false, listeners are called sequentially.
    ///
    /// # Errors
    /// Returns `EventError::EventNotFound` if the event has not been registered.
    fn emit_async<'a>(&'a mut self, event_name: &'a str, payload: EventPayload<T>, parallel: bool) -> BoxFuture<Result<(), EventError>>;

    /// Asynchronous emission of a specific Event for the last time.
    ///
    /// # Parameters
    /// - `parallel`: If true, listeners are called in parallel (spawned as tasks); if false, listeners are called sequentially.
    ///
    /// # Errors
    /// Returns `EventError::EventNotFound` if the event has not been registered.
    fn emit_final_async<'a>(&'a mut self, event_name: &'a str, payload: EventPayload<T>, parallel: bool) -> BoxFuture<Result<(), EventError>>;
}