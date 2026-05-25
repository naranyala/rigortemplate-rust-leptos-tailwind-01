use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use crate::shared::hooks::StoredNode;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ElementSize {
    pub width: f64,
    pub height: f64,
}

/// Tracks the size of a DOM element.
pub fn use_element_size<T>(target: StoredNode<T>) -> ReadSignal<ElementSize>
where
    T: leptos::html::ElementType,
    T::Output: JsCast + Clone,
{
    let (size, set_size) = signal(ElementSize { width: 0.0, height: 0.0 });

    let target_node = target.clone();
    let set_size_clone = set_size.clone();

    let observer_callback = move |entries: &js_sys::Array| {
        let entry_jsvalue = entries.get(0);
        if !entry_jsvalue.is_undefined() {
            if let Some(entry) = entry_jsvalue.dyn_ref::<web_sys::ResizeObserverEntry>() {
                let rect_js = js_sys::Reflect::get(entry, &JsValue::from_str("contentRect")).unwrap();
                let rect: web_sys::DomRect = rect_js.unchecked_into();
                set_size_clone.set(ElementSize {
                    width: rect.width(),
                    height: rect.height(),
                });
            }
        }
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(observer_callback) as Box<dyn FnMut(&js_sys::Array)>);
    let callback = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();

    let observer = web_sys::ResizeObserver::new(&callback).expect("failed to create ResizeObserver");

    let target_el = target_node.get().expect("target element not found");
    let el = target_el.unchecked_ref::<web_sys::Element>().clone();
    observer.observe(&el);

    on_cleanup(move || {
        observer.disconnect();
    });
    closure.forget();

    size
}

/// Tracks if an element is visible in the viewport.
pub fn use_element_visibility<T>(target: StoredNode<T>) -> ReadSignal<bool>
where
    T: leptos::html::ElementType,
    T::Output: JsCast + Clone,
{
    let (is_visible, set_is_visible) = signal(false);

    let observer_callback = move |entries: &js_sys::Array| {
        let entry_jsvalue = entries.get(0);
        if !entry_jsvalue.is_undefined() {
            if let Some(entry) = entry_jsvalue.dyn_ref::<web_sys::IntersectionObserverEntry>() {
                set_is_visible.set(entry.is_intersecting());
            }
        }
    };

    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(observer_callback) as Box<dyn FnMut(&js_sys::Array)>);
    let callback = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();

    let observer = web_sys::IntersectionObserver::new(&callback).expect("failed to create IntersectionObserver");

    let target_el = target.get().expect("target element not found");
    let el = target_el.unchecked_ref::<web_sys::Element>().clone();
    observer.observe(&el);

    on_cleanup(move || {
        observer.disconnect();
    });
    closure.forget();

    is_visible
}
