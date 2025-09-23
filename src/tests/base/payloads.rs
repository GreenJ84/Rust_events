extern crate alloc;
use crate::{EventEmitter, EventHandler, EventPayload};
use alloc::{string::String, sync::Arc, vec::Vec};

#[test]
fn u32_emit_successful() {
    let called = Arc::new(core::sync::atomic::AtomicU64::new(0));
    let called2 = called.clone();
    let cb: Arc<dyn Fn(&EventPayload<u32>) + Send + Sync> = Arc::new(move |payload| {
        assert_eq!(*payload.as_ref(), 42);
        called2.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    });

    let mut emitter = EventEmitter::<u32>::default();
    emitter.add("emit", None, cb).unwrap();
    emitter.emit("emit", Arc::new(42)).unwrap();
    assert_eq!(called.load(core::sync::atomic::Ordering::SeqCst), 1);
}

#[test]
fn bool_emit_successful() {
    let called = Arc::new(core::sync::atomic::AtomicU64::new(0));
    let called2 = called.clone();
    let cb: Arc<dyn Fn(&EventPayload<bool>) + Send + Sync> = Arc::new(move |payload| {
        assert_eq!(*payload.as_ref(), true);
        called2.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    });

    let mut emitter = EventEmitter::<bool>::default();
    emitter.add("emit", None, cb).unwrap();
    emitter.emit("emit", Arc::new(true)).unwrap();
    assert_eq!(called.load(core::sync::atomic::Ordering::SeqCst), 1);
}

#[test]
fn binary_emit_successful() {
    let called = Arc::new(core::sync::atomic::AtomicU64::new(0));
    let called2 = called.clone();
    let cb: Arc<dyn Fn(&EventPayload<Vec<u8>>) + Send + Sync> = Arc::new(move |payload| {
        assert_eq!(payload.as_ref(), b"Test");
        called2.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    });

    let mut emitter = EventEmitter::<Vec<u8>>::default();
    emitter.add("count", None, cb).unwrap();
    for _ in 0..10 {
        emitter
            .emit("count", Arc::new(Vec::from(b"Test".as_ref())))
            .unwrap();
    }
    assert_eq!(called.load(core::sync::atomic::Ordering::SeqCst), 10);
}

#[derive(PartialEq, Eq, Debug)]
struct TestCustomPayload {
    message: String,
    option: bool,
    val: u32,
}
#[test]
fn custom_emit_successful() {
    let called = Arc::new(core::sync::atomic::AtomicU64::new(0));
    let called2 = called.clone();
    let cb: Arc<dyn Fn(&EventPayload<TestCustomPayload>) + Send + Sync> =
        Arc::new(move |payload| {
            let received = payload.as_ref();
            assert_eq!(received.message, String::from("custom"));
            assert_eq!(
                received.option,
                received.val % 2 == 0 && received.val % 4 == 0
            );
            called2.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        });

    let mut emitter = EventEmitter::<TestCustomPayload>::default();
    emitter.add("count", None, cb).unwrap();
    for val in 0..10 {
        let payload = TestCustomPayload {
            message: "custom".to_string(),
            option: val % 2 == 0 && val % 4 == 0,
            val,
        };
        emitter.emit("count", Arc::new(payload)).unwrap();
    }
    assert_eq!(called.load(core::sync::atomic::Ordering::SeqCst), 10);
}
