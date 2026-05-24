use leptos::prelude::*;
use web_sys::window;
use wasm_bindgen::JsCast;

/// Syncs a signal with a URL query parameter.
/// Returns a ReadSignal that updates when the URL param changes.
pub fn use_url_param(key: &'static str) -> ReadSignal<Option<String>> {
    let win = window().expect("window not available");
    let location = win.location();
    
    let get_param = move || {
        let search = location.search().unwrap_or_default();
        let params = web_sys::UrlSearchParams::new_with_str(&search).ok()?;
        params.get(key)
    };

    let value = signal(get_param());

    // We need to listen for popstate events to update the signal when the URL changes
    let handle_popstate = move |_: web_sys::Event| {
        value.set(get_param());
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_popstate) as Box<dyn FnMut(_)>);
    
    win.add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .expect("failed to add popstate listener");

    closure.forget();

    value
}

/// Syncs a signal with the URL hash.
pub fn use_url_hash() -> ReadSignal<String> {
    let win = window().expect("window not available");
    let location = win.location();
    
    let get_hash = move || {
        location.hash().unwrap_or_default().trim_start_matches('#').to_string()
    };

    let value = signal(get_hash());

    let handle_hashchange = move |_: web_sys::Event| {
        value.set(get_hash());
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_hashchange) as Box<dyn FnMut(_)>);
    
    win.add_event_listener_with_callback("hashchange", closure.as_ref().unchecked_ref())
        .expect("failed to add hashchange listener");

    closure.forget();

    value
}
