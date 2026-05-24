use leptos::prelude::*;
use web_sys::window;
use wasm_bindgen::JsCast;
use std::rc::Rc;

/// A simple timer hook that executes a callback after a delay.
pub fn use_timer(delay: u32, callback: impl Fn() + 'static) -> ReadSignal<i32> {
    let (id, set_id) = signal(0i32);
    
    let callback = Rc::new(callback);
    
    let callback_clone = callback.clone();
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        callback_clone();
    }) as Box<dyn FnMut()>);
    
    let win = window().expect("window not available");
    let handle = win.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        delay as i32,
    ).expect("failed to set timeout");
    
    set_id.set(handle);
    closure.forget();
    
    id
}
