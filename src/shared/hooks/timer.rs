use leptos::prelude::*;
use web_sys::window;
use wasm_bindgen::JsCast;

/// A simple timer hook that executes a callback after a delay.
pub fn use_timer(delay: u32, callback: impl Fn() + 'static) -> ReadSignal<i32> {
    let (id, set_id) = signal(0i32);

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);
    let callback_fn = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();

    let win = window().expect("window not available");
    let handle = win.set_timeout_with_callback_and_timeout_and_arguments_0(
        &callback_fn,
        delay as i32,
    ).expect("failed to set timeout");

    set_id.set(handle);

    on_cleanup(move || {
        win.clear_timeout_with_handle(handle);
    });
    closure.forget();

    id
}
