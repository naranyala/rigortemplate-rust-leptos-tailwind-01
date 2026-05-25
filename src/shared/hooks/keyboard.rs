use web_sys::{window, KeyboardEvent};
use wasm_bindgen::JsCast;
use leptos::prelude::*;


pub struct Shortcut {
    pub key: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
}

pub fn use_keyboard_shortcut<F>(shortcut: Shortcut, callback: F)
where
    F: Fn() + 'static,
{
    let win = window().expect("window not available");
    let callback = std::rc::Rc::new(std::cell::RefCell::new(callback));

    let handle_keydown = move |ev: KeyboardEvent| {
        if ev.key() == shortcut.key
            && ev.ctrl_key() == shortcut.ctrl
            && ev.alt_key() == shortcut.alt
            && ev.shift_key() == shortcut.shift
            && ev.meta_key() == shortcut.meta
        {
            (callback.borrow_mut())();
        }
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_keydown) as Box<dyn FnMut(_)>);
    let callback_fn = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();

    win.add_event_listener_with_callback("keydown", &callback_fn)
        .expect("failed to add keydown listener");

    on_cleanup(move || {
        let _ = win.remove_event_listener_with_callback("keydown", &callback_fn);
    });
    closure.forget();
}
