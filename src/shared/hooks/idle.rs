use leptos::prelude::*;
use web_sys::window;
use wasm_bindgen::JsCast;

/// Detects user inactivity.
/// Returns a ReadSignal that is true when the user is considered idle.
pub fn use_idle(timeout_ms: u32) -> ReadSignal<bool> {
    let (is_idle, set_idle) = signal(false);
    
    let set_idle_clone = set_idle.clone();
    let timeout_ms_clone = timeout_ms;
    
    // Use i32 for the timer handle as required by web_sys
    let (timer_handle, set_timer_handle) = signal(0i32);

    let reset_timer = move || {
        let win = window().expect("window not available");
        
        // Clear existing timer
        win.clear_timeout_with_handle(timer_handle.get());
        
        set_idle_clone.set(false);
        
        // Set new timer
        let set_idle_inner = set_idle_clone.clone();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            set_idle_inner.set(true);
        }) as Box<dyn FnMut()>);
        
        let handle = win.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            timeout_ms_clone as i32,
        ).expect("failed to set timeout");
        
        set_timer_handle.set(handle);
        closure.forget();
    };

    // Event listeners to reset the timer
    let events = ["mousemove", "keydown", "mousedown", "touchstart", "wheel"];
    let win = window().expect("window not available");
    
    for event in events {
        let reset_clone = reset_timer.clone();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
            reset_clone();
        }) as Box<dyn FnMut(web_sys::Event)>);
        
        win.add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
            .expect("failed to add event listener");
        closure.forget();
    }

    // Initial timer start
    reset_timer();

    is_idle
}
