#[cfg(feature = "threaded")]
mod threaded {
    mod event_emitter;
    mod listener;
    mod payloads;
}

#[cfg(not(feature = "threaded"))]
mod base {
    mod event_emitter;
    mod listener;
    mod payloads;
}
