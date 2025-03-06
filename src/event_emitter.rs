use std::sync::Arc;
use dashmap::DashMap;

use crate::event_handler::EventHandler;
use crate::{Callback, EventError, EventPayload};
use crate::listener::Listener;

/// Type alias for a cross-thread safe Arc referenced `EventEmitter<T>` struct
pub type EventManager<T> = Arc<EventEmitter<T>>;

/// A struct intended to handle the implementations of the EventHandler trait
///
/// This implementation utilizes DashMap, a concurrent associative array/ hashmap as the underlying storage.
#[derive(Clone)]
pub struct EventEmitter<T> where T: Send + Sync + 'static  {
    max_listeners: usize,
    events: Arc<DashMap<String, Vec<Listener<T>>>>,
}
impl<T: Send + Sync+ 'static> EventEmitter<T>{
    /// Creates a new `EventEmitter<T>` from a passed max listeners value.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::events::{EventEmitter, EventHandler};
    ///
    /// let emitter: EventEmitter<String> = EventEmitter::new(20);
    /// assert_eq!(emitter.max_listeners(), 20);
    /// ```
    pub fn new(max_listeners: usize) -> Self {
        Self {
            max_listeners,
            events: Arc::new(DashMap::new()),
        }
    }
}
impl<T: Send + Sync> Default for EventEmitter<T>{
    /// Creates a new `EventEmitter<T>` with a default max listeners of 10.
    ///
    /// # Example
    ///
    /// ```
    /// use events::{EventEmitter, EventHandler};
    ///
    /// let emitter: EventEmitter<String> = EventEmitter::default();
    /// assert_eq!(emitter.max_listeners(), 10);
    /// ```
    fn default() -> Self {
        Self {
            max_listeners: 10,
            events: Arc::new(DashMap::new()),
        }
    }
}
impl<T: Send + Sync> EventEmitter<T> {
    /// Get the underlying DashMap Arc reference
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use dashmap::DashMap;
    /// use events::{EventEmitter, EventHandler, Listener};
    ///
    /// let mut emitter: EventEmitter<String> = EventEmitter::default();
    ///  if let Some(events_map) = Arc::get_mut(emitter.events_mut()){
    /// }
    /// ```
   pub fn events_mut(&mut self) -> &mut Arc<DashMap<String, Vec<Listener<T>>>> {
        &mut self.events
    }
}
impl<T: Send+ Sync> EventHandler<T> for EventEmitter<T> {
    /// Get a Vec containing the key of each active event entry register in the underlying DashMap.
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use events::{EventEmitter, EventHandler};
    ///
    /// let mut emitter: EventEmitter<String> = EventEmitter::default();
    /// emitter.add_listener("event_one", Arc::new(|_|{})).expect("Failed to add");
    /// emitter.add_listener("event_two", Arc::new(|_|{})).expect("Failed to add");
    /// emitter.add_listener("event_three", Arc::new(|_|{})).expect("Failed to add");
    /// ```
    fn event_names(&self) -> Vec<String> {
        self.events.iter().map(|entry| entry.key().clone()).collect()
    }
    /// Set the maximum number of listeners allowed for a specific event
    ///
    /// # Example
    ///
    /// ```
   /// use events::{EventEmitter, EventHandler};
    ///
    /// let mut emitter: EventEmitter<String> = EventEmitter::default();
    /// emitter.set_max_listeners(4);
    /// ```
    fn set_max_listeners(&mut self, max: usize) { self.max_listeners = max; }

    /// Get the current maximum listeners limit for events within the Event Emitter
    ///
    /// # Example
    ///
    /// ```
   /// use events::{EventEmitter, EventHandler};
    ///
    /// let mut emitter: EventEmitter<String> = EventEmitter::default();
    /// assert_eq!(emitter.max_listeners(), 10); // 10 is max_listeners default
    /// ```
    fn max_listeners(&self) -> usize { self.max_listeners }


    fn add_listener(&mut self, event_name: &str, callback: Callback<T>) -> Result<Listener<T>, EventError> {
        self.add_limited_listener(event_name, callback, 0)
    }

    fn add_limited_listener(&mut self, event_name: &str, callback: Callback<T>, limit: u64) -> Result<Listener<T>, EventError> {
        let mut entry = self.events.entry(event_name.to_string()).or_default();
        if entry.len() < self.max_listeners {
            let listener = Listener::new(
                callback,
                if limit > 0 { Some(limit) } else { None }
            );
            entry.push(listener.clone());
            return Ok(listener);
        }
        Err(EventError::OverloadedEvent)
    }

    fn add_once(&mut self, event_name: &str, callback: Callback<T>) -> Result<Listener<T>, EventError> {
        self.add_limited_listener(event_name, callback, 1)
    }


    fn listener_count(&self, event_name: &str) -> usize {
        self.events
            .get(event_name)
            .map(|entry| entry.len())
            .unwrap_or(0)
    }


    fn remove_listener(&mut self, event_name: &str, callback: &Listener<T>) -> Result<(), EventError> {
        if let Some(mut entry) = self.events.get_mut(event_name) {
            let original_len = entry.len();
            entry.retain(|listener| !listener.eq(callback));

            return if entry.len() < original_len {
                Ok(())
            } else {
                Err(EventError::ListenerNotFound)
            };
        }
        Err(EventError::EventNotFound)
    }

    fn remove_all_listeners(&mut self, event_name: &str) -> Result<(), EventError> {
        if self.events.remove(event_name).is_some() {
            Ok(())
        } else {
            Err(EventError::EventNotFound)
        }
    }

    fn emit(&mut self, event_name: &str, payload: EventPayload<T>) -> Result<(), EventError> {
        if let Some(mut entry) = self.events_mut().get_mut(event_name) {
            for listener in entry.iter_mut().rev() {
                listener.call(&payload);
            }
            entry.retain(|listener| !listener.at_limit());
            return Ok(());
        }
        Err(EventError::EventNotFound)
    }

    fn emit_final(&mut self, event_name: &str, payload: EventPayload<T>) -> Result<(), EventError> {
        if self.events_mut().contains_key(event_name){
            for listener in self.events.get_mut(event_name).unwrap().iter_mut() {
                listener.call(&payload);
            }
            self.events_mut().remove(event_name);
            return Ok(())
        }
        Err(EventError::EventNotFound)
    }

    /// Concurrent Async: (parallel == false)
    /// - Tasks do not block each other but share the same CPU core
    ///
    /// Parallel Async: (parallel == true)
    ///  - Tasks run in parallel on different CPU cores
    fn emit_async<'a>(&'a mut self, event_name: &'a str, payload: EventPayload<T>, parallel: bool) -> Result<(), EventError> {
        if let Some(mut entry) = self.events.get_mut(event_name) {
            for listener in entry.iter_mut().rev() {
                if parallel {
                    listener.background_call(&payload);
                } else {
                    listener.blocking_call(&payload);
                }
            }

            entry.retain(|listener| !listener.at_limit());
            return Ok(())
        }
        Err(EventError::EventNotFound)
    }

    fn emit_final_async<'a>(&'a mut self, event_name: &'a str, payload: EventPayload<T>, parallel: bool) -> Result<(), EventError> {
        if self.events_mut().contains_key(event_name) {
            for listener in self.events_mut().get_mut(event_name).unwrap().iter_mut() {
                if parallel {
                    listener.background_call(&payload);
                } else {
                    listener.blocking_call(&payload);
                }
            }

            self.events_mut().remove(event_name);
            return Ok(());
        }
        Err(EventError::EventNotFound)
    }
}