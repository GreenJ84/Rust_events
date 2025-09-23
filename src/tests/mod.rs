#[cfg(not(feature = "no_std"))]
mod threaded {
    mod listener;
    mod event_emitter;
    mod payloads;
}

#[cfg(feature = "no_std")]
mod base{
    mod listener;
    mod event_emitter;
    mod payloads;
}
