use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

use crate::{Callback, EventPayload};

/// A Struct that is used to enact upon an emit Event
pub struct Listener<T> {
    callback: Callback<T>,
    lifetime: Option<Arc<Mutex<u64>>>,
}
impl<T: Send + Sync + 'static> Listener<T> {
    /// Constructs a new `Listener<T>` from a given a `Callback<T>` and lifetime `Option<u64>`
    ///
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use event_emitter::{Callback, EventPayload, Listener};
    ///
    /// let listener = Listener::new(
    ///     Arc::new(move |payload: &EventPayload<String>| {
    ///         // Handle payload
    ///     }),
    ///     Some(10)
    /// );
    /// ```
    pub fn new(callback: Callback<T>, lifetime: Option<u64>) -> Self {
        if let Some(limit) = lifetime {
            return Self { callback, lifetime: Some(Arc::new(Mutex::new(limit))) };
        }
        Self { callback, lifetime: None }
    }

    /// Synchronously run the callback for a `Listener<T>` with an emitted event payload. <br/>
    /// If the listener has a lifetime, it will be decreased by 1 for the current emission.
    ///
    /// Best for Synchronous order dependant tasks.
    ///
    /// # Example
    ///
    /// ```
    /// use event_emitter::{Callback, EventPayload, Listener};
    ///
    /// let mut listener = Listener::default();
    /// listener.call(&EventPayload::new(String::new()));
    /// ```
    pub fn call(&mut self, payload: &EventPayload<T>) {
        if let Some(ref lifetime) = self.lifetime {
            let mut count = lifetime.lock().unwrap();
            *count -= 1;
        }
        (self.callback)(payload);
    }

    /// Asynchronously run the callback for a `Listener<T>` ana emitted event payload within a concurrent task pool. <br/>
    /// If the listener has a lifetime, it will be decreased by 1 for the current emission.
    ///
    /// Best for Background Processing and I/O heavy tasks
    ///
    /// # Example
    ///
    /// ```
    /// use event_emitter::{Callback, EventPayload, Listener};
    ///
    /// let mut listener = Listener::default();
    /// # tokio_test::block_on(async {
    /// listener.background_call(&EventPayload::new(String::new()));
    /// # })
    /// ```
    pub fn background_call(&mut self, payload: &EventPayload<T>) {
        if let Some(ref lifetime) = self.lifetime {
            let mut count = lifetime.lock().unwrap();
            *count -= 1;
        }

        tokio::spawn({
            let callback = Arc::clone(&self.callback);
            let payload = Arc::clone(&payload);
            async move {
                callback(&payload);
            }
        });
    }

    /// Asynchronously run the callback for a `Listener<T>` ana emitted event payload within a blocking thread pool. <br/>
    /// If the listener has a lifetime, it will be decreased by 1 for the current emission.
    ///
    /// Best for CPU heavy Operations
    ///
    /// # Example
    ///
    /// ```
    /// use event_emitter::{EventPayload, Listener};
    ///
    /// let mut listener = Listener::default();
    /// # tokio_test::block_on(async {
    /// listener.blocking_call(&EventPayload::new(String::new()));
    /// # })
    /// ```
    pub fn blocking_call(&mut self, payload: &EventPayload<T>) {
        if let Some(ref lifetime) = self.lifetime {
            let mut count = lifetime.lock().unwrap();
            *count -= 1;
        }

        tokio::task::spawn_blocking({
            let callback = Arc::clone(&self.callback);
            let payload = Arc::clone(&payload);
            move || { callback(&payload) }
        });
    }

    /// Check whether the `Listener<T>` has fulfilled its lifetime.
    pub fn at_limit(&self) -> bool {
        if let Some(ref lifetime) = self.lifetime {
            let count = lifetime.lock().unwrap();
            return *count == 0;
        }
        false
    }

    /// Check whether a `Callback<T>` is the same Arc reference as in the `Listener<T>`.
    pub fn eq_callback(&self, callback: Callback<T>) -> bool{
        Arc::ptr_eq(&self.callback, &callback)
    }
}
impl<T:  Send + Sync + 'static> Clone for Listener<T> {
    fn clone(&self) -> Self {
        Self {
            callback: Arc::clone(&self.callback),
            lifetime: if let Some(limit) = &self.lifetime {
                Some(Arc::clone(limit))
            } else { None },
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.callback = Arc::clone(&source.callback);
        self.lifetime = if let Some(limit) = &source.lifetime {
            Some(Arc::clone(limit))
        } else { None };
    }
}
impl<T: Send + Sync + 'static> Default for Listener<T> {
    /// Returns the default value for the `Listener<T>` struct
    ///
    /// # Example
    ///
    /// ```
    /// use event_emitter::EventEmitter;
    ///
    /// // Type annotation required
    /// let emitter = EventEmitter::<String>::default();
    /// ```
    fn default() -> Self {
        Self::new(Arc::new(move |_: &EventPayload<T>| { println!("You wasted an Event, congrats!") }), Some(1))
    }
}
impl<T> Debug for Listener<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Listener (limit: {:?})", self.lifetime)
    }
}
impl<T> PartialEq<Self> for Listener<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.callback, &other.callback)
    }
}
impl<T> Eq for Listener<T>{}
