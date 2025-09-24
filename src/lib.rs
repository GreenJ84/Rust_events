//! # Rust Events Crate
//!
//! This crate provides a flexible, modular event system for Rust applications.
//!
//! - **Listener**: Represents a struct that holds a tag (optional), callback, and lifetime (optional) which can be registered to an event.
//! - **EventEmitter**: Manages event registration and emission.
//! - **EventHandler**: Trait defining the event API.
//!
//! By default, the crate uses the `threaded` (multi-threaded, async) implementation.
//! All core types are exported from the `threaded` module.
//!
//! ## Usage Examples
//!
//! **Threaded (default)**
//! ```rust
//! use rs_events::{EventEmitter, EventPayload, EventHandler};
//! use std::sync::Arc;
//!
//! let mut emitter = EventEmitter::<String>::default();
//! emitter.add("event", None, Arc::new(|payload| {
//!     println!("Received: {}", payload.as_ref());
//! })).unwrap();
//! emitter.emit("event", Arc::new("Hello World".to_string())).unwrap();
//! ```
//!
//! **no_std/alloc**
//! Build with:
//! ```shell
//! cargo build --no-default-features
//! ```
//!
//! ```rust
//! extern crate alloc;
//! use alloc::sync::Arc;
//! use alloc::string::String;
//! use rs_events::{EventEmitter, EventPayload, EventHandler};
//!
//! let mut emitter = EventEmitter::<String>::default();
//! emitter.add("event", None, Arc::new(|payload| {
//!     // Handle event
//! })).unwrap();
//! emitter.emit("event", Arc::new(String::from("Hello no_std!"))).unwrap();
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

mod constants;
pub use crate::constants::*;

mod error;
pub use crate::error::*;

// Base (non-threaded) backend
#[cfg(not(feature = "threaded"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "threaded"))))]
mod base;

#[cfg(not(feature = "threaded"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "threaded"))))]
pub use base::{event_emitter::EventEmitter, event_handler::EventHandler, listener::Listener};

// Threaded backend
#[cfg(feature = "threaded")]
#[cfg_attr(docsrs, doc(cfg(feature = "threaded")))]
mod threaded;

#[cfg(feature = "threaded")]
#[cfg_attr(docsrs, doc(cfg(feature = "threaded")))]
pub use threaded::{event_emitter::EventEmitter, event_handler::EventHandler, listener::Listener};
#[cfg(test)]
mod tests;
