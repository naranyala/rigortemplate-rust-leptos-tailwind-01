use leptos::prelude::*;
use web_sys::{window, MouseEvent};
use wasm_bindgen::JsCast;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MousePosition {
    pub x: i32,
    pub y: i32,
}

pub fn use_mouse_position() -> ReadSignal<MousePosition> {
    let win = window().expect("window not available");
    
    let initial_pos = MousePosition { x: 0, y: 0 };
    let pos = signal(initial_pos);

    let handle_mousemove = move |ev: MouseEvent| {
        pos.set(MousePosition {
            x: ev.client_x(),
            y: ev.client_y(),
        });
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_mousemove) as Box<dyn FnMut(_)>);
    
    win.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
        .expect("failed to add mousemove listener");

    closure.forget();

    pos
}
