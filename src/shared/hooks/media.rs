use leptos::prelude::*;
use web_sys::{window, MediaQueryListEvent};
use wasm_bindgen::JsCast;

pub fn use_media_query(query: &'static str) -> ReadSignal<bool> {
    let win = window().expect("window not available");
    let mql = win.match_media(query).expect("failed to match media query").expect("MediaQueryList not found");
    
    let initial_matches = mql.matches();
    let (matches, set_matches) = signal(initial_matches);
    
    let handle_change = move |ev: MediaQueryListEvent| {
        set_matches.set(ev.matches());
    };
    
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_change) as Box<dyn FnMut(_)>);
    
    mql.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
        .expect("failed to add media query listener");
    
    closure.forget();
    
    matches
}



