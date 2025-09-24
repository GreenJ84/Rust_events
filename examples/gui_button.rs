//! Example: GUI button click event
#![no_std]
extern crate alloc;
use alloc::{
    string::{String, ToString},
    sync::Arc,
};
use rs_events::{EventEmitter, EventHandler};

fn main() {
    let mut emitter = EventEmitter::<String>::default();
    emitter
        .add(
            "button_click",
            None,
            Arc::new(|payload| {
                // Handle button click event
                let _value = payload.as_ref();
            }),
        )
        .unwrap();

    // Simulate button click
    emitter
        .emit("button_click", Arc::new("OK".to_string()))
        .unwrap();
}
