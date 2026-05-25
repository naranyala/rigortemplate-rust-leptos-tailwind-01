use leptos::prelude::*;
use web_sys::window;
use wasm_bindgen::JsCast;
use std::sync::{Arc, Mutex};

/// Detects user inactivity.
/// Returns a ReadSignal that is true when the user is considered idle.
pub fn use_idle(timeout_ms: u32) -> ReadSignal<bool> {
    let win = window().expect("window not available");
    let (is_idle, set_idle) = signal(false);

    let timer_handle: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(None));

    let start_timer = {
        let win = win.clone();
        let set_idle = set_idle.clone();
        let handle = timer_handle.clone();
        let timeout = timeout_ms;
        move || {
            if let Ok(mut guard) = handle.lock() {
                if let Some(old) = guard.take() {
                    win.clear_timeout_with_handle(old);
                }
            }
            set_idle.set(false);
            let set_idle_inner = set_idle.clone();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                set_idle_inner.set(true);
            }) as Box<dyn FnMut()>);
            let callback = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
            let new_handle = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                &callback,
                timeout as i32,
            ).expect("failed to set timeout");
            if let Ok(mut guard) = handle.lock() {
                *guard = Some(new_handle);
            }
            closure.forget();
        }
    };

    let events = ["mousemove", "keydown", "mousedown", "touchstart", "wheel"];
    let mut callbacks: Vec<(String, js_sys::Function)> = Vec::new();

    for event in events {
        let start = start_timer.clone();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
            start();
        }) as Box<dyn FnMut(web_sys::Event)>);
        let callback = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
        let event_owned = event.to_string();

        win.add_event_listener_with_callback(event, &callback)
            .expect("failed to add event listener");

        callbacks.push((event_owned, callback));
        closure.forget();
    }

    start_timer();

    on_cleanup(move || {
        if let Ok(mut guard) = timer_handle.lock() {
            if let Some(handle) = guard.take() {
                win.clear_timeout_with_handle(handle);
            }
        }
        for (event, callback) in callbacks {
            let _ = win.remove_event_listener_with_callback(&event, &callback);
        }
    });

    is_idle
}
