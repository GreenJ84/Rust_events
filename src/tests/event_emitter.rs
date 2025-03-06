use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

use crate::{EventManager, EventEmitter, EventHandler, EventPayload, EventError};

type TestStringPayload = String;
fn test_string_payload(data: &str) -> EventPayload<TestStringPayload> {
    Arc::new(data.to_string())
}

mod adding_listeners {
    use super::*;

    #[test]
    fn add_single_infinite_listener() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            emitter.add_listener("test", Arc::new(move |_| {})).unwrap();
        }

        assert_eq!(emitter.listener_count("test"), 1,"Listener was not added");
    }

    #[test]
    fn add_single_finite_listener() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            emitter.add_limited_listener("test", Arc::new(move |_| {}), 5).unwrap();
        }

        assert_eq!(emitter.listener_count("test"), 1,"Listener was not added");
    }

    #[test]
    fn add_single_once_listener() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            emitter.add_once("test", Arc::new(move |_| {})).unwrap();
        }

        assert_eq!(emitter.listener_count("test"), 1,"Listener was not added");
    }

    #[test]
    fn add_multiple_different_listeners() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());

        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            for idx in 0..5{
                match idx {
                    0 | 2 => {
                        emitter.add_once("once", Arc::new(move |_: &EventPayload<TestStringPayload>| {})).unwrap();
                    }
                    1 | 3=> {
                        emitter.add_limited_listener("limited", Arc::new(move |_: &EventPayload<TestStringPayload>| {}), 4).unwrap();
                    }
                    _ => {
                        emitter.add_listener("unlimited", Arc::new(move |_: &EventPayload<TestStringPayload>| {})).unwrap();
                    }
                }
            }
        }
        for event_idx in 0..3 {
            match event_idx {
                0 => {
                    assert_eq!(emitter.listener_count("once"), 2, "a ONCE Listener was not added")
                },
                1 => {
                    assert_eq!(emitter.listener_count("limited"), 2,"a LIMITED Listener was not added");
                },
                _ => {
                    assert_eq!(emitter.listener_count("unlimited"), 1,"a UNLIMITED Listener was not added")
                }
            }
        }
    }

    #[test]
    fn overloading_event_throws_error() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());

        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            for _ in 0..10{
                emitter.add_listener("test", Arc::new(move |_| {})).unwrap();
            }

            assert_eq!(
                emitter.add_listener("test", Arc::new(move |_| {})),
                Err(EventError::OverloadedEvent),
                "Emitter did not throw a Overload error"
            );
        }
    }
}


mod removing_listeners {
    use crate::Listener;
    use super::*;

    #[test]
    fn remove_single_listener() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            let listener = emitter.add_listener("test", Arc::new(|_| {}));
            assert!(listener.is_ok());
            assert_eq!(emitter.listener_count("test"), 1);

            assert!(emitter.remove_listener("test", &listener.unwrap()).is_ok());
            assert_eq!(emitter.listener_count("test"), 0);
        }
    }

    #[test]
    fn remove_single_finite_listener() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            let listener = emitter.add_limited_listener("test", Arc::new(|_| {}), 5);
            assert!(listener.is_ok());
            assert_eq!(emitter.listener_count("test"), 1);

            assert!(emitter.remove_listener("test", &listener.unwrap()).is_ok());
            assert_eq!(emitter.listener_count("test"), 0);
        }
    }

    #[test]
    fn remove_single_once_listener() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            let listener = emitter.add_once("test", Arc::new(|_| {}));
            assert!(listener.is_ok());
            assert_eq!(emitter.listener_count("test"), 1);

            assert!(emitter.remove_listener("test", &listener.unwrap()).is_ok());
            assert_eq!(emitter.listener_count("test"), 0);
        }
    }

    #[test]
    fn remove_all_listeners() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            for _ in 0..10 {
                emitter.add_listener("test", Arc::new(|_| {})).unwrap();
            }
            assert_eq!(emitter.listener_count("test"), 10);

            assert!(emitter.remove_all_listeners("test").is_ok());
            assert_eq!(emitter.listener_count("test"), 0);
        }
    }

    #[test]
    fn remove_invalid_listener_throws_error() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            emitter.add_listener("test", Arc::new(|_| {})).unwrap();
            assert_eq!(emitter.remove_listener("test", &Listener::<TestStringPayload>::new(Arc::new(|_| {}), None)), Err(EventError::ListenerNotFound));
        }
    }

    #[test]
    fn remove_from_invalid_event_throws_error() {
        let mut emitter = EventManager::new(EventEmitter::<TestStringPayload>::default());
        if let Some(emitter) = Arc::get_mut(&mut emitter) {
            emitter.add_listener("test", Arc::new(|_| {})).unwrap();
            assert_eq!(emitter.remove_listener("not_test",  &Listener::<TestStringPayload>::new(Arc::new(|_| {}), None)), Err(EventError::EventNotFound));
        }
    }
}


mod emitting_events {
    use super::*;

    #[test]
    fn emit_successful() {
        let mut emitter = EventManager::<TestStringPayload>::new(EventEmitter::default());
        let count = Arc::new(Mutex::new(0));
        let count_clone = Arc::clone(&count);
        {
            let emitter = Arc::get_mut(&mut emitter).unwrap();
            assert!(
                emitter.add_listener("count", Arc::new(move |payload| {
                    assert_eq!(payload.as_ref(), "Test");
                    *count_clone.lock().unwrap() += 1;
                })).is_ok(),
                "Failed to add event listener"
            );

            for _ in 0..10 {
                assert!(
                    emitter.emit("count", test_string_payload("Test")).is_ok(),
                    "Failed to emit event"
                );
            }
        }

        assert_eq!(*count.lock().unwrap(), 10, "Event callbacks unsuccessful");
    }

    #[test]
    fn limited_listener_emission_drop_off_successful() {
        let mut emitter = EventManager::new(EventEmitter::default());
        let count = Arc::new(Mutex::new(0));
        let count_clone = Arc::clone(&count);
        {
            let emitter = Arc::get_mut(&mut emitter).unwrap();
            assert!(
                emitter.add_limited_listener(
                    "count",
                    Arc::new(move |_| {
                        *count_clone.lock().unwrap() += 1;
                    }),
                    5
                ).is_ok(),
                "Failed to add event listener"
            );

            for _ in 0..5 {
                assert!(
                    emitter.emit("count", test_string_payload("Test")).is_ok(),
                    "Failed to emit event to limited listeners"
                );
            }
        }

        assert!(
            !emitter.has_listener("count"),
            "Failed to remove limited listener {}", emitter.listener_count("count")
        );
        assert_eq!(*count.lock().unwrap(), 5, "Event callbacks unsuccessful");
    }

    #[test]
    fn once_listener_emission_drop_off_successful() {
        let mut emitter = EventManager::new(EventEmitter::default());
        let count = Arc::new(Mutex::new(0));
        let count_clone = Arc::clone(&count);
        {
            let emitter = Arc::get_mut(&mut emitter).unwrap();
            assert!(
                emitter.add_once("count", Arc::new(move |_| {
                    *count_clone.lock().unwrap() += 1;
                })).is_ok(),
                "Failed to add once event listener"
            );

            assert!(
                emitter.emit("count", test_string_payload("Increment")).is_ok(),
                "Failed to emit event to once listeners"
            );
        }

        assert!(
            !emitter.has_listener("count"),
            "Failed to remove once listener: {}", emitter.listener_count("count")
        );
        assert_eq!(*count.lock().unwrap(), 1, "Event callbacks unsuccessful");
    }
}


mod emitting_async_events {
    use super::*;

    #[tokio::test]
    async fn async_emission_successful() {
        let mut emitter = EventManager::<TestStringPayload>::new(EventEmitter::default());
        let count = Arc::new(Mutex::new(0));
        let count_clone = Arc::clone(&count);
        {
            let emitter = Arc::get_mut(&mut emitter).unwrap();
            assert!(
                emitter.add_listener("async_event", Arc::new(move |payload| {
                    assert_eq!(payload.as_ref(), "Async Test");
                    *count_clone.lock().unwrap() += 1;
                })).is_ok(),
                "Failed to add event listener"
            );

            for _ in 0..10 {
                assert!(
                    emitter.emit_async("async_event", test_string_payload("Async Test"), false).is_ok()
                );
                sleep(Duration::from_millis(100)).await;
            }
        }

        assert_eq!(*count.lock().unwrap(), 10, "Async event callbacks unsuccessful");
    }

    #[tokio::test]
    async fn async_limited_listener_emission_drop_off_successful() {
        let mut emitter = EventManager::new(EventEmitter::default());
        let count = Arc::new(Mutex::new(0));
        let count_clone = Arc::clone(&count);
        {
            let emitter = Arc::get_mut(&mut emitter).unwrap();
            assert!(
                emitter.add_limited_listener(
                    "async_event",
                    Arc::new(move |_| {
                        *count_clone.lock().unwrap() += 1;
                    }),
                    5
                ).is_ok(),
                "Failed to add event listener"
            );

            for _ in 0..5{
                assert!(
                    emitter.emit_async("async_event", test_string_payload(""), false).is_ok()
                );
                sleep(Duration::from_millis(100)).await; // Multiple emit rely on same Mutex for test
            }
        }

        assert_eq!(*count.lock().unwrap(), 5, "Event callbacks unsuccessful");
        assert!(!emitter.has_listener("count"), "Failed to remove limited listener");
    }

    #[tokio::test]
    async fn async_once_listener_emission_drop_off_successful() {
        let mut emitter = EventManager::new(EventEmitter::default());
        let count = Arc::new(Mutex::new(0));
        let count_clone = Arc::clone(&count);
        {
            let emitter = Arc::get_mut(&mut emitter).unwrap();
            assert!(
                emitter.add_once(
                    "final_async_event",
                    Arc::new(move |_| {
                        *count_clone.lock().unwrap() += 1;
                    })).is_ok(),
                "Failed to add event listener"
            );

            assert!(
                emitter.emit_async("final_async_event", test_string_payload(""), false).is_ok()
            );
            sleep(Duration::from_millis(100)).await;
        }

        assert_eq!(*count.lock().unwrap(), 1, "Event callbacks unsuccessful");
        assert!(!emitter.has_listener("count"), "Failed to remove once listener");
    }

    // async fn off(){
    //     let mut emitter = EventManager::<TestStringPayload>::new(EventEmitter::default());
    //     {
    //         let emitter = Arc::get_mut(&mut emitter).unwrap();
    //         let callback_count = Arc::new(Mutex::new(0));
    //         let callback_count_clone = Arc::clone(&callback_count);
    //
    //         let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    //         let tx = Arc::new(Mutex::new(Some(tx)));
    //
    //         assert!(emitter.add_listener("async_event", Arc::new({
    //             let tx = Arc::clone(&tx);
    //             move |val| {
    //                 if let Some(tx) = tx.lock().unwrap().as_ref() {
    //                     let _ = tx.clone().send(val.clone());
    //                     *callback_count_clone.lock().unwrap() += 1;
    //                 }
    //             }
    //         })).is_ok());
    //
    //         for _ in 0..10 {
    //             emitter.emit_async("async_event", Arc::new("Hello".to_string())).await;
    //         }
    //         drop(tx);
    //
    //         sleep(Duration::from_secs(5)).await;
    //         assert_eq!(*callback_count.lock().unwrap(), 10, "Event callbacks not triggered");
    //
    //         let mut received = 0;
    //         while let Some(val) = rx.recv().await{
    //            println!("{:?}", val);
    //             received+=1;
    //             if received > 0 {
    //                 break;
    //             }
    //         }
    //     }
    // }
}


mod emitting_final_events{
    use super::*;

    #[test]
    fn emit_final_drops_infinite_listener() {
        let mut emitter = EventManager::<TestStringPayload>::new(EventEmitter::default());
        {
            let emitter = Arc::get_mut(&mut emitter).unwrap();
            assert!(
                emitter.add_listener("count_final", Arc::new(move |payload| {
                    assert_eq!(payload.as_ref(), "Test");
                })).is_ok(),
                "Failed to add event listener"
            );

            assert!(
                emitter.emit_final("count_final", test_string_payload("Test")).is_ok(),
                "Failed to emit final event"
            );
            assert!(
                emitter.emit_final("count_final", test_string_payload("Test"))
                    .is_err_and(|e| e.eq(&EventError::EventNotFound)),
                "Failed to emit final event"
            );
        }

    }

    #[test]
    fn emit_final_drops_limited_listener() {
        let mut emitter = EventManager::<TestStringPayload>::new(EventEmitter::default());
        {
            let emitter = Arc::get_mut(&mut emitter).unwrap();
            assert!(
                emitter.add_limited_listener("count_final", Arc::new(move |_| {}), 5).is_ok(),
                "Failed to add event listener"
            );

            assert!(
                emitter.emit_final("count_final", test_string_payload("Test")).is_ok(),
                "Failed to emit final event"
            );

            assert!(
                emitter.emit_final("count_final", test_string_payload("Test"))
                    .is_err_and(|e| e.eq(&EventError::EventNotFound)),
                "Failed to emit final event"
            );
        }
    }
}


mod emitting_async_final_events {
    use super::*;

    #[tokio::test]
    async fn emit_final_async_drops_infinite_listener() {
        let mut emitter = EventManager::new(EventEmitter::default());
        {
            let emitter = Arc::get_mut(&mut emitter).unwrap();
            assert!(emitter.add_listener("final_async_event", Arc::new(|_| {})).is_ok());

            assert!(
                emitter.emit_final_async("final_async_event", test_string_payload(""), false).is_ok()
            );
        }
        assert!(!emitter.has_listener("final_async_event"), "Final async emit did not remove listeners");
    }

    #[tokio::test]
    async fn emit_final_async_drops_limited_listener() {
        let mut emitter = EventManager::new(EventEmitter::default());
        {
            let emitter = Arc::get_mut(&mut emitter).unwrap();
            assert!(emitter.add_limited_listener("final_async_event", Arc::new(|_| {}), 5).is_ok());

            assert!(
                emitter.emit_final_async("final_async_event", test_string_payload(""), false).is_ok()
            );
        }
        assert!(!emitter.has_listener("final_async_event"), "Final async emit did not remove limited listeners");
    }
}
