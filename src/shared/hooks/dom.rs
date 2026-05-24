use leptos::prelude::*;
use web_sys::{window, Node, MouseEvent};
use wasm_bindgen::JsCast;
use crate::shared::hooks::StoredNode;

pub fn use_click_outside<F>(target: StoredNode<leptos::html::Div>, callback: F)
where
    F: Fn() + 'static,
{
    let win = window().expect("window not available");
    
    let callback = std::rc::Rc::new(std::cell::RefCell::new(callback));
    
    let handle_click = move |ev: MouseEvent| {
        let target_element = target.get().expect("target element not found");
        let path = ev.composed_path();
        let mut contains = false;
        
            for node in path.iter() {
                let node_as_node = node.unchecked_into::<Node>();
                if AsRef::<Node>::as_ref(&node_as_node) == AsRef::<Node>::as_ref(&target_element) {
                    contains = true;
                    break;
                }
            }

        
        if !contains {
            (callback.borrow_mut())();
        }
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handle_click) as Box<dyn FnMut(_)>);
    
    win.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .expect("failed to add click listener");

    closure.forget();
}
