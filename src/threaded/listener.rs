use std::fmt::{Debug, Formatter};
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use tokio::task::JoinHandle;

use crate::{Callback, EventPayload};

/// A handle for an event listener callback.
///
/// `Listener<T>` wraps a callback and an optional lifetime counter, allowing for one-shot, limited, or unlimited event listeners.
///
/// # Thread Safety
///
/// All methods are thread-safe. Lifetime is managed atomically.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use std::sync::Arc;
/// use events::{Listener, EventPayload};
/// let listener = Listener::new(Arc::new(|payload: &EventPayload<String>| {
///     println!("Got: {}", payload);
/// }), None);
/// ```
pub struct Listener<T> {
    callback: Callback<T>,
    lifetime: Option<Arc<AtomicU64>>,
}

impl<T: Send + Sync + 'static> Listener<T> {
    /// Create a new listener with an optional lifetime.
    ///
    /// If `lifetime` is `None` or `Some(0)`, the listener is unlimited.
    /// If `lifetime` is `Some(n)`, the listener will be called at most `n` times.
    ///
    /// # Examples
    ///
    /// Unlimited:
    /// ```
    /// # use std::sync::Arc;
    /// # use events::{Listener, EventPayload};
    /// let listener = Listener::new(Arc::new(|_: &EventPayload<String>| {}), None);
    /// ```
    ///
    /// Limited:
    /// ```
    /// # use std::sync::Arc;
    /// # use events::{Listener, EventPayload};
    /// let listener = Listener::new(Arc::new(|_: &EventPayload<String>| {}), Some(3));
    /// ```
    pub fn new(callback: Callback<T>, lifetime: Option<u64>) -> Self {
        match lifetime {
            Some(0) | None => Self { callback, lifetime: None },
            Some(limit) => Self { callback, lifetime: Some(Arc::new(AtomicU64::new(limit))) },
        }
    }

    /// Returns whether the listener has reached its call limit.
    ///
    /// # Returns
    /// * `true` if the listener was created with a limited lifetime and has been called the maximum number of times.
    /// * `false` if the listener is unlimited or has remaining calls.
    ///
    /// # Example
    /// ```
    /// # use std::sync::Arc;
    /// # use events::{Listener, EventPayload};
    /// let mut listener = Listener::new(Arc::new(|_: &EventPayload<String>| {}), Some(1));
    /// assert!(!listener.at_limit());
    /// listener.call(&Arc::new("payload".to_string()));
    /// assert!(listener.at_limit());
    /// ```
    #[inline]
    pub fn at_limit(&self) -> bool {
        match self.lifetime {
            None => false,
            Some(ref lifetime) =>  lifetime.load(Ordering::SeqCst) == 0
        }
    }


    /// Returns the number of remaining calls for this listener, if it has a limited lifetime.
    ///
    /// # Returns
    /// * `Some(n)` if the listener was created with a limited lifetime, where `n` is the number of calls left before it is at limit.
    /// * `None` if the listener is unlimited.
    ///
    /// # Example
    /// ```
    /// # use std::sync::Arc;
    /// # use events::{Listener, EventPayload};
    /// let mut listener = Listener::new(Arc::new(|_: &EventPayload<String>| {}), Some(2));
    /// assert_eq!(listener.remaining_calls(), Some(2));
    /// listener.call(&Arc::new("payload".to_string()));
    /// assert_eq!(listener.remaining_calls(), Some(1));
    /// ```
    pub fn remaining_calls(&self) -> Option<u64> {
        self.lifetime.as_ref().map(|l| l.load(Ordering::SeqCst))
    }

    /// Synchronously invoke the callback with the given payload.
    ///
    /// If the listener has a limited lifetime, it is decremented. If at limit, the callback is not invoked.
    ///
    /// # Panics
    /// Panics if the atomic counter is poisoned (should not happen).
    ///
    /// # Example
    /// ```
    /// # use std::sync::Arc;
    /// # use events::{Listener, EventPayload};
    /// let mut listener = Listener::new(Arc::new(|_: &EventPayload<String>| {}), Some(1));
    /// listener.call(&Arc::new("payload".to_string()));
    /// ```
    #[inline]
    pub fn call(&mut self, payload: &EventPayload<T>) {
        if let Some(ref lifetime) = self.lifetime {
            if lifetime.fetch_update(
                Ordering::SeqCst,
                Ordering::SeqCst,
                |x| if x > 0 { Some(x - 1) } else { None }
            ).is_err() { return; }
        }
        (self.callback)(payload);
    }

    /// Asynchronously invoke the callback in a Tokio task.
    ///
    /// If the listener has a limited lifetime, it is decremented. If at limit, the callback is not invoked.
    ///
    /// # Returns
    /// `Some(JoinHandle)` if the callback was invoked, `None` if at limit.
    ///
    /// # Example
    /// ```
    /// # use std::sync::Arc;
    /// # use events::{Listener, EventPayload};
    /// let mut listener = Listener::new(Arc::new(|_: &EventPayload<String>| {}), Some(1));
    /// let handle = listener.background_call(&Arc::new("payload".to_string()));
    /// assert!(handle.is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn background_call(&mut self, payload: &EventPayload<T>) -> Option<JoinHandle<()>> {
        if let Some(ref lifetime) = self.lifetime {
            if lifetime.fetch_update(
                Ordering::SeqCst,
                Ordering::SeqCst,
                |x| if x > 0 { Some(x - 1) } else { None }
            ).is_err() { return None; }
        }
        let callback = Arc::clone(&self.callback);
        let payload = Arc::clone(&payload);
        Some(tokio::spawn(async move {
            callback(&payload);
        }))
    }

    /// Asynchronously invoke the callback in a blocking thread pool (for CPU-heavy work).
    ///
    /// If the listener has a limited lifetime, it is decremented. If at limit, the callback is not invoked.
    ///
    /// # Returns
    /// `Some(JoinHandle)` if the callback was invoked, `None` if at limit.
    ///
    /// # Example
    /// ```
    /// # use std::sync::Arc;
    /// # use events::{Listener, EventPayload};
    /// let mut listener = Listener::new(Arc::new(|_: &EventPayload<String>| {}), Some(1));
    /// let handle = listener.blocking_call(&Arc::new("payload".to_string()));
    /// assert!(handle.is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn blocking_call(&mut self, payload: &EventPayload<T>) -> Option<JoinHandle<()>> {
        if let Some(ref lifetime) = self.lifetime {
            if lifetime.fetch_update(
                Ordering::SeqCst,
                Ordering::SeqCst,
                |x| if x > 0 { Some(x - 1) } else { None }
            ).is_err() { return None; }
        }
        let callback = Arc::clone(&self.callback);
        let payload = Arc::clone(&payload);
        Some(tokio::task::spawn_blocking(move || {
            callback(&payload)
        }))
    }
}
impl<T: Send + Sync + 'static> Clone for Listener<T> {
    fn clone(&self) -> Self {
        Self {
            callback: Arc::clone(&self.callback),
            lifetime: self.lifetime.as_ref().map(Arc::clone),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.callback = Arc::clone(&source.callback);
        self.lifetime = source.lifetime.as_ref().map(Arc::clone);
    }
}
impl<T: Send + Sync + 'static> Default for Listener<T> {
    /// Returns a default listener with a no-op callback and a single call limit.
    ///
    /// # Example
    /// ```
    /// use events::Listener;
    /// let _ = Listener::<String>::default();
    /// ```
    fn default() -> Self {
        Self::new(Arc::new(|_: &EventPayload<T>| {}), Some(1))
    }
}
impl<T> Debug for Listener<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Listener")
            .field("lifetime", &self.lifetime.as_ref().map(|a| a.load(Ordering::SeqCst)))
            .finish()
    }
}
impl<T> PartialEq for Listener<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.callback, &other.callback)
    }
}
impl<T> Eq for Listener<T> {}
