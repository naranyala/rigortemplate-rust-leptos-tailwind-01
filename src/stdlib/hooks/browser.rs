use leptos::prelude::*;
use web_sys::window;

/// Tracks if the browser is online or offline.
pub fn use_online_status() -> ReadSignal<bool> {
    let win = window().expect("window not available");
    let initial_status = win.navigator().on_line();
    let online = signal(initial_status);

    let handle_change = move |_: web_sys::Event| {
        online.set(window().expect("window not available").navigator().on_line());
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_change) as Box<dyn FnMut(_)>);
    
    win.add_event_listener_with_callback("online", closure.as_ref().unchecked_ref()).ok();
    win.add_event_listener_with_callback("offline", closure.as_ref().unchecked_ref()).ok();

    closure.forget();

    online
}

/// Detects and tracks the system's color scheme (light/dark).
pub fn use_dark_mode() -> ReadSignal<bool> {
    let win = window().expect("window not available");
    let mql = win.match_media("(prefers-color-scheme: dark)").expect("failed to match media query");
    
    let initial_dark = mql.matches();
    let is_dark = signal(initial_dark);

    let handle_change = move |ev: web_sys::MediaQueryListEvent| {
        is_dark.set(ev.matches());
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_change) as Box<dyn FnMut(_)>);
    
    mql.add_listener(closure.as_ref().unchecked_ref()).ok();

    closure.forget();

    is_dark
}
