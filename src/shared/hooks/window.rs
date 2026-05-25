use leptos::prelude::*;
use web_sys::Event;
use wasm_bindgen::JsCast;

pub const SOURCE: &str = include_str!("window.rs");

#[derive(Clone, Copy, Debug)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

pub fn use_window_size() -> ReadSignal<WindowSize> {
    let win = window();

    let initial_size = WindowSize {
        width: win.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(0.0) as u32,
        height: win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0) as u32,
    };

    let (read_size, write_size) = signal(initial_size);

    let handle_resize = move |_: Event| {
        let win = window();
        write_size.set(WindowSize {
            width: win.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(0.0) as u32,
            height: win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0) as u32,
        });
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_resize) as Box<dyn FnMut(_)>);
    let callback = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();

    win.add_event_listener_with_callback("resize", &callback)
        .expect("failed to add resize listener");

    on_cleanup(move || {
        let _ = win.remove_event_listener_with_callback("resize", &callback);
    });
    closure.forget();

    read_size
}
