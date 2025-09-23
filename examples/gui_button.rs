//! Example: GUI button click event
extern crate alloc;
use alloc::sync::Arc;
use rs_events::{EventEmitter, EventHandler};

fn main() {
    let mut emitter = EventEmitter::<String>::default();
    emitter
        .add(
            "button_click",
            None,
            Arc::new(|payload| {
                println!("Button clicked: {}", payload.as_ref());
            }),
        )
        .unwrap();

    // Simulate button click
    emitter
        .emit("button_click", Arc::new("OK".to_string()))
        .unwrap();
}
