use leptos::prelude::*;
use web_sys::window;
use wasm_bindgen::JsCast;
use std::collections::HashMap;

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

    let (value, set_value) = signal(get_param());

    // We need to listen for popstate events to update the signal when the URL changes
    let handle_popstate = move |_: web_sys::Event| {
        set_value.set(get_param());
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_popstate) as Box<dyn FnMut(_)>);
    
    win.add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .expect("failed to add popstate listener");

    closure.forget();

    value
}

/// Syncs a signal with all URL query parameters.
/// Returns a tuple of (ReadSignal of all params, Callback to update a param).
pub fn use_url_params() -> (ReadSignal<HashMap<String, String>>, Callback<(String, Option<String>)>) {
    let win = window().expect("window not available");
    let location = win.location();

    let get_all_params = move || {
        let search = location.search().unwrap_or_default();
        let params = web_sys::UrlSearchParams::new_with_str(&search).unwrap();
        let mut map = HashMap::new();
        
        let entries = params.entries();
        while let Ok(entry_jsvalue) = entries.next() {
            if let Some(pair) = entry_jsvalue.dyn_ref::<js_sys::Array>() {
                if pair.length() == 2 {
                    if let (Some(k), Some(v)) = (pair.get(0).as_string(), pair.get(1).as_string()) {
                        map.insert(k, v);
                    }
                }
            }
        }
        map
    };

    let (params_signal, set_params_signal) = signal(get_all_params());

    // We need to clone the closure because it's used in multiple places
    let get_all_params_clone = get_all_params.clone();
    let handle_popstate = move |_: web_sys::Event| {
        set_params_signal.set(get_all_params_clone());
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_popstate) as Box<dyn FnMut(_)>);
    let _ = win.add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref());
    closure.forget();

    let update_param = move |(key, value): (String, Option<String>)| {
        let win = window().expect("window not available");
        let location = win.location();
        let search = location.search().unwrap_or_default();
        let params = web_sys::UrlSearchParams::new_with_str(&search).unwrap();
        
        if let Some(val) = value {
            params.set(&key, &val);
        } else {
            params.delete(&key);
        }
        
        let new_search = params.to_string();
        let current_url = location.href().unwrap();
        let base_url = current_url.split('?').next().unwrap_or("").to_string();
        let new_url = format!("{}?{}", base_url, new_search);
        
        let _ = win.history().expect("history not available").push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url));
        
        set_params_signal.set(get_all_params());
    };

    (params_signal, Callback::new(update_param))
}

/// Syncs a signal with the URL hash.
pub fn use_url_hash() -> ReadSignal<String> {
    let win = window().expect("window not available");
    let location = win.location();
    
    let get_hash = move || {
        location.hash().unwrap_or_default().trim_start_matches('#').to_string()
    };

    let (value, set_value) = signal(get_hash());

    let handle_hashchange = move |_: web_sys::Event| {
        set_value.set(get_hash());
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_hashchange) as Box<dyn FnMut(_)>);
    
    win.add_event_listener_with_callback("hashchange", closure.as_ref().unchecked_ref())
        .expect("failed to add hashchange listener");

    closure.forget();

    value
}
