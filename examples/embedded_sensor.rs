//! Example: Embedded sensor event handler (no_std/alloc)
#![no_std]
extern crate alloc;
use alloc::sync::Arc;
use rs_events::{EventEmitter, EventHandler};

fn main() {
    let mut emitter = EventEmitter::<u32>::default();
    emitter
        .add(
            "sensor_triggered",
            None,
            Arc::new(|payload| {
                // handle sensor value
                let _value = payload.as_ref();
                // In real embedded, replace with logging or hardware action
                // e.g., blink LED, send data, etc.
            }),
        )
        .unwrap();

    // Simulate sensor event
    emitter.emit("sensor_triggered", Arc::new(42)).unwrap();
}
