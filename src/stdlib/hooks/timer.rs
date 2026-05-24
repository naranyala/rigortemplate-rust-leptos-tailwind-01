use leptos::prelude::*;
use gloo_timers::callback::{Interval, Timeout};
use std::rc::Rc;
use std::cell::RefCell;

pub fn use_interval<F>(delay: u32, mut callback: F) 
where 
    F: FnMut() + 'static 
{
    let callback = Rc::new(RefCell::new(callback));
    
    let interval = Interval::new(delay, move || {
        (callback.borrow_mut())();
    });

    // Keep the interval alive by forgetting it, but we should ideally provide a way to clear it.
    // In a hook, we can wrap it in an Option and use on_cleanup.
    let interval_handle = Rc::new(RefCell::new(Some(interval)));
    let handle_clone = interval_handle.clone();
    
    on_cleanup(move || {
        *handle_clone.borrow_mut() = None;
    });
}

pub fn use_debounce<T, F>(value: ReadSignal<T>, delay: u32, mut callback: F)
where
    T: Clone + PartialEq + 'static,
    F: FnMut(T) + 'static,
{
    let callback = Rc::new(RefCell::new(callback));
    let timeout_handle = Rc::new(RefCell::new(None::<Timeout>));

    Effect::new(move |_| {
        let val = value.get();
        let timeout_clone = timeout_handle.clone();
        let cb_clone = callback.clone();

        // Clear existing timeout
        *timeout_clone.borrow_mut() = None;

        // Set new timeout
        *timeout_clone.borrow_mut() = Some(Timeout::new(delay, move || {
            (cb_clone.borrow_mut())(val.clone());
        }));
    });
}
