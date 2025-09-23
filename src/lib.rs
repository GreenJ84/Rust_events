#![cfg_attr(not(feature = "threaded"), no_std)]

//! # Rust Events Crate
//!
//! This crate provides a flexible, modular event system for Rust applications.
//!
//! - **Listener**: Represents a callback that can be registered to an event.
//! - **EventEmitter**: Manages event registration and emission.
//! - **EventHandler**: Trait defining the event API.
//!
//! By default, the crate uses the `threaded` (multi-threaded, async) implementation.
//! All core types are exported from the `threaded` module.
//!
//! For embedded or single-threaded use, see the `base` module (not exported by default). Use the base crate with the `no_std` feature enabled.

mod constants;
pub use crate::constants::*;

mod error;
pub use crate::error::*;

#[cfg(feature = "no_std")]
mod base;
#[cfg(feature = "no_std")]
pub use base::{
    listener::Listener,
    event_emitter::EventEmitter,
    event_handler::EventHandler
};

#[cfg(not(feature = "no_std"))]
mod threaded;
#[cfg(not(feature = "no_std"))]
pub use threaded::{
    listener::Listener,
    event_emitter::EventEmitter,
    event_handler::EventHandler
};

#[cfg(test)]
mod tests;