# rs_events

[![Crates.io](https://img.shields.io/crates/v/rs_events.svg)](https://crates.io/crates/rs_events)
[![Docs.rs](https://docs.rs/rs_events/badge.svg)](https://docs.rs/rs_events)
[![License](https://img.shields.io/crates/l/rs_events.svg)](./LICENSE)
[![Build Status](https://github.com/GreenJ84/Rust_events_crate/actions/workflows/ci.yml/badge.svg)](https://github.com/GreenJ84/Rust_events_crate/actions)


`rust_events` is a highly flexible, ergonomic, and portable event emitter crate for Rust.

It supports both high-performance threaded applications and minimal `no_std`/`alloc` environments, making it suitable for everything from servers to embedded systems.

---

## Quick Example

```rust
use rs_events::{EventEmitter, EventPayload};
use std::sync::Arc;

fn main() {
    let mut emitter = EventEmitter::<String>::default();
    emitter.add("event", None, Arc::new(|payload| {
        println!("Received: {}", payload.as_ref());
    })).unwrap();

    emitter.emit("event", Arc::new("Hello World".to_string())).unwrap();
}
```

---

## Features

- **Dual Environment Support:**
  - `threaded` (default): Uses `std`, `tokio`, and `dashmap` for high concurrency and async support.
  - `no_std`/`alloc`: Minimal, dependency-free build for embedded and constrained environments.
- **Listener Types:**
  - Unlimited: Listeners persist until explicitly removed.
  - Limited: Listeners auto-remove after a set number of calls.
  - Once: Listeners auto-remove after a single call.
- **Tagging & Lifetimes:**
  - Tag listeners for easy identification and management.
  - Provide lifetimes to manage the number of times a listener gets called.
  - Query and remove listeners.
- **Async & Sync Emission:**
  - Emit events synchronously or asynchronously (blocking or parallel).
  - Async emission uses `tokio` for efficient scheduling.
- **Robust Error Handling:**
  - Comprehensive error types for missing events, listener overload, and more.
- **Feature-Gated Dependencies:**
  - Only include heavy dependencies when needed, keeping builds minimal for embedded use.

---

## Design Rationale

- **Ergonomics:**
  - Simple, type-safe API for adding, removing, and emitting events.
  - Arc-based payloads for cheap cloning and thread safety.
- **Performance:**
  - Uses `dashmap` for lock-free, highly concurrent event storage in threaded mode (enabled by the `threaded` feature). This allows multiple threads to add, remove, and emit events without blocking, making it ideal for servers and async applications.
  - In no_std/alloc mode, falls back to `BTreeMap` for event storage. `BTreeMap` is chosen for its minimal dependency footprint and efficient ordered storage, suitable for embedded and constrained environments where concurrency is not available.
  - This dual approach ensures optimal performance and minimal resource usage for both desktop/server and embedded/portable targets.
  - Minimal allocations and fast event dispatch in both modes.
- **Portability:**
  - Compile with `--no-default-features` for no_std/alloc.
  - All core logic is feature-gated for easy adaptation.

---

## Getting Started

Add to your Cargo.toml:

```toml
[dependencies]
rust_events = "1.0.0"
```

### Threaded (default)

Build with:

```shell
cargo build
```

Example:

```rust
use rust_events::{EventEmitter, EventPayload};
use std::sync::Arc;

fn main() {
    let mut emitter = EventEmitter::<String>::default();
    emitter.add("event", None, Arc::new(|payload| {
        println!("Received: {}", payload.as_ref());
    })).unwrap();
    emitter.emit("event", Arc::new("Hello World".to_string())).unwrap();
}
```

### no_std/alloc

Build with:

```shell
cargo build --no-default-features
```

Use `alloc::sync::Arc` and `alloc::string::String` in your code. See crate docs for details.

Example:

```rust
extern crate alloc;
use alloc::sync::Arc;
use alloc::string::String;
use rust_events::{EventEmitter, EventPayload};

let mut emitter = EventEmitter::<String>::default();
emitter.add("event", None, Arc::new(|payload| {
  // Handle event
})).unwrap();
emitter.emit("event", Arc::new(String::from("Hello no_std!"))).unwrap();
```

## Usage

### Adding Listeners with Tags and Lifetimes

```rust
let mut emitter = EventEmitter::<u32>::default();
let listener = emitter.add_limited("count", Some("tag1".to_string()), Arc::new(|payload| {
    println!("Count: {}", payload.as_ref());
}), 5).unwrap();
assert_eq!(listener.tag(), Some(&"tag1".to_string()));
assert_eq!(listener.lifetime(), Some(5));
```

### Async Emission (Blocking and Parallel - Threaded only)

```rust
#[tokio::main]
async fn main() {
    let mut emitter = EventEmitter::<String>::default();
    emitter.add("event", None, Arc::new(|payload| {
        println!("Received: {}", payload.as_ref());
    })).unwrap();
    // Blocking mode
    emitter.emit_async("event", Arc::new("Hello Async".to_string()), false).await.unwrap();
    // Parallel mode
    emitter.emit_async("event", Arc::new("Hello Async".to_string()), true).await.unwrap();
}
```

### Removing Listeners

```rust
let mut emitter = EventEmitter::<String>::default();
let listener = emitter.add_once("event", Some("tag1".to_string()), Arc::new(|_| {})).unwrap();
emitter.remove_listener("event", &listener).unwrap();
```

### Final Emission and Listener Drop-off

```rust
let mut emitter = EventEmitter::<String>::default();
emitter.add("event", None, Arc::new(|_| {})).unwrap();
let falloff = emitter.emit_final("event", Arc::new("Final".to_string())).unwrap();
assert_eq!(falloff.len(), 1); // Listener removed after final emit
```

---

## Feature Flags

- `threaded` (default): Enables std/threaded support and dependencies (`tokio`, `dashmap`, `futures`).
- Build with `--no-default-features` for no_std/alloc.

## Documentation

- [API Docs](https://docs.rs/rust_events)

## License

MIT OR Apache-2.0

## Contributing

We welcome contributions of all kinds—bug reports, feature requests, documentation improvements, and code enhancements. Please:

- Open issues for bugs, questions, or feature ideas.
- Submit pull requests for code or documentation improvements.
- Follow the coding style and guidelines outlined in [CONTRIBUTING.md](CONTRIBUTING.md).

See [CONTRIBUTING.md](CONTRIBUTING.md) for full details on how to get started, code standards, and the review process.

## Author

Jesse Greenough
