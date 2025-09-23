#[cfg(not(feature = "threaded"))]
extern crate alloc;
#[cfg(not(feature = "threaded"))]
use alloc::sync::Arc;

#[cfg(feature = "threaded")]
use std::sync::Arc;

/// Type alias for an event payload pointer.
///
/// Uses `Arc<T>` for both threaded and embedded builds.
///
/// # Example
/// ```
/// #[cfg(not(feature = "threaded"))]
/// extern crate alloc;
/// #[cfg(not(feature = "threaded"))]
/// use alloc::sync::Arc;
///
/// #[cfg(feature = "threaded")]
/// use std::sync::Arc;
///
/// use events::{Callback, EventPayload};
/// let payload: EventPayload<String> = Arc::new(String::from("Emitting value"));
/// ```
pub type EventPayload<T> = Arc<T>;


/// Type alias for a callback pointer.
///
/// - Allows any closure (no thread-safety required).
///
/// # Example (embedded/no_std)
/// ```
/// extern crate alloc;
/// use alloc::sync::Arc;
/// use events::{Callback, EventPayload};
/// let callback: Callback<String> = Arc::new(move |payload: &EventPayload<String>| {
///     println!("Received event: {}", payload);
/// });
/// ```
#[cfg(not(feature = "threaded"))]
pub type Callback<T> = Arc<dyn Fn(&EventPayload<T>)>;

/// Type alias for a callback pointer.
///
/// - Requires `Send + Sync` for thread safety.
///
/// # Example (threaded)
/// ```
/// use std::sync::Arc;
/// use events::{Callback, EventPayload};
/// let callback: Callback<String> = Arc::new(move |payload: &EventPayload<String>| {
///     println!("Received event: {}", payload);
/// });
#[cfg(feature = "threaded")]
pub type Callback<T> = Arc<dyn Fn(&EventPayload<T>) + Send + Sync>;
