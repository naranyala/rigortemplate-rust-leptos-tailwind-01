use leptos::prelude::*;
use web_sys::MouseEvent;
use wasm_bindgen::JsCast;

pub const SOURCE: &str = include_str!("mouse.rs");

#[derive(Clone, Copy, Debug)]
pub struct MousePosition {
    pub x: u32,
    pub y: u32,
}

pub fn use_mouse_position() -> ReadSignal<MousePosition> {
    let win = window();

    let initial_pos = MousePosition { x: 0, y: 0 };

    let (read_pos, write_pos) = signal(initial_pos);

    let handle_mousemove = move |ev: MouseEvent| {
        write_pos.set(MousePosition {
            x: ev.client_x() as u32,
            y: ev.client_y() as u32,
        });
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_mousemove) as Box<dyn FnMut(_)>);
    let callback = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();

    win.add_event_listener_with_callback("mousemove", &callback)
        .expect("failed to add mousemove listener");

    on_cleanup(move || {
        let _ = win.remove_event_listener_with_callback("mousemove", &callback);
    });
    closure.forget();

    read_pos
}

