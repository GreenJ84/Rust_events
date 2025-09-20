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
//! For embedded or single-threaded use, see the `base` module (not exported by default).

mod constants;
mod error;
mod base;
mod threaded;

#[cfg(test)]
mod tests;