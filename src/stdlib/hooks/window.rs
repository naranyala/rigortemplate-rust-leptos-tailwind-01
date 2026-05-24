use leptos::prelude::*;
use web_sys::{window, Event};
use wasm_bindgen::JsCast;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

pub fn use_window_size() -> ReadSignal<WindowSize> {
    let win = window().expect("window not available");
    
    let initial_size = WindowSize {
        width: win.inner_width().unwrap_or(0).unwrap_or(0) as u32,
        height: win.inner_height().unwrap_or(0).unwrap_or(0) as u32,
    };

    let size = signal(initial_size);

    let handle_resize = move |_: Event| {
        if let Some(win) = window() {
            size.set(WindowSize {
                width: win.inner_width().unwrap_or(0).unwrap_or(0) as u32,
                height: win.inner_height().unwrap_or(0).unwrap_or(0) as u32,
            });
        }
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_resize) as Box<dyn FnMut(_)>);
    
    win.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .expect("failed to add resize listener");

    closure.forget();

    size
}
