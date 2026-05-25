use leptos::prelude::*;
use web_sys::window;
use wasm_bindgen::JsCast;

/// Tracks if the browser is online or offline.
pub fn use_online_status() -> ReadSignal<bool> {
    let win = window().expect("window not available");
    let initial_status = win.navigator().on_line();
    let (online, set_online) = signal(initial_status);

    let handle_change = move |_: web_sys::Event| {
        set_online.set(window().expect("window not available").navigator().on_line());
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_change) as Box<dyn FnMut(_)>);
    let callback = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();

    win.add_event_listener_with_callback("online", &callback).ok();
    win.add_event_listener_with_callback("offline", &callback).ok();

    on_cleanup(move || {
        let _ = win.remove_event_listener_with_callback("online", &callback);
        let _ = win.remove_event_listener_with_callback("offline", &callback);
    });
    closure.forget();

    online
}

/// Detects and tracks the system's color scheme (light/dark).
pub fn use_dark_mode() -> ReadSignal<bool> {
    let win = window().expect("window not available");
    let mql = win.match_media("(prefers-color-scheme: dark)").expect("failed to match media query").expect("MediaQueryList not found");

    let initial_dark = mql.matches();
    let (is_dark, set_is_dark) = signal(initial_dark);

    let mql_clone = mql.clone();
    let handle_change_fixed = move |_: web_sys::Event| {
        set_is_dark.set(mql_clone.matches());
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_change_fixed) as Box<dyn FnMut(_)>);
    let callback = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();

    mql.add_event_listener_with_callback("change", &callback).ok();

    on_cleanup(move || {
        let _ = mql.remove_event_listener_with_callback("change", &callback);
    });
    closure.forget();

    is_dark
}
