//! Example: Threaded event-driven server (std)
use rs_events::{EventEmitter, EventHandler};
use std::sync::Arc;

fn main() {
    let mut emitter = EventEmitter::<String>::default();
    emitter
        .add(
            "client_connected",
            None,
            Arc::new(|payload| {
                println!("Client connected: {}", payload.as_ref());
            }),
        )
        .unwrap();
    emitter
        .add(
            "message_received",
            None,
            Arc::new(|payload| {
                println!("Message: {}", payload.as_ref());
            }),
        )
        .unwrap();

    // Simulate server events
    emitter
        .emit("client_connected", Arc::new("Alice".to_string()))
        .unwrap();
    emitter
        .emit("message_received", Arc::new("Hello from Alice".to_string()))
        .unwrap();
}
