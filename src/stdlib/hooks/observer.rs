use leptos::prelude::*;
use web_sys::{window, Element};
use wasm_bindgen::JsCast;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ElementSize {
    pub width: f64,
    pub height: f64,
}

/// Tracks the size of a DOM element.
pub fn use_element_size(target: StoredNode<leptos::html::Div>) -> ReadSignal<ElementSize> {
    let size = signal(ElementSize { width: 0.0, height: 0.0 });

    let target_node = target.clone();
    let size_clone = size.clone();

    let observer_callback = move |entries: &js_sys::Array| {
        if let Some(entry) = entries.get(0).and_then(|e| e.dyn_ref::<web_sys::ResizeObserverEntry>()) {
            let rect = entry.content_rect();
            size_clone.set(ElementSize {
                width: rect.width(),
                height: rect.height(),
            });
        }
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(observer_callback) as Box<dyn FnMut(&js_sys::Array)>);
    
    let observer = web_sys::ResizeObserver::new(closure.as_ref().unchecked_ref()).expect("failed to create ResizeObserver");
    
    let target_el = target_node.get().expect("target element not found");
    observer.observe(&target_el);

    // Cleanup
    let observer_arc = std::sync::Arc::new(observer);
    let observer_clone = observer_arc.clone();
    on_cleanup(move || {
        let _ = observer_clone.unobserve(&target_el);
    });

    closure.forget();

    size
}

/// Tracks if an element is visible in the viewport.
pub fn use_element_visibility(target: StoredNode<leptos::html::Div>) -> ReadSignal<bool> {
    let is_visible = signal(false);

    let is_visible_clone = is_visible.clone();
    let observer_callback = move |entries: &js_sys::Array| {
        if let Some(entry) = entries.get(0).and_then(|e| e.dyn_ref::<web_sys::IntersectionObserverEntry>()) {
            is_visible_clone.set(entry.is_intersecting());
        }
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(observer_callback) as Box<dyn FnMut(&js_sys::Array)>);
    
    let options = web_sys::IntersectionObserverInit::new();
    let observer = web_sys::IntersectionObserver::new(closure.as_ref().unchecked_ref(), &options).expect("failed to create IntersectionObserver");
    
    let target_el = target.get().expect("target element not found");
    observer.observe(&target_el);

    let observer_arc = std::sync::Arc::new(observer);
    let observer_clone = observer_arc.clone();
    on_cleanup(move || {
        let _ = observer_clone.unobserve(&target_el);
    });

    closure.forget();

    is_visible
}
