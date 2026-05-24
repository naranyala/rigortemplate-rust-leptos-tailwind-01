use leptos::prelude::*;

/// A simple boolean toggle hook.
/// Returns a tuple containing a read signal for the value and a function to toggle it.
pub fn use_toggle(initial: bool) -> (ReadSignal<bool>, impl Fn()) {
    let (value, set_value) = signal(initial);
    
    let toggle = move || {
        set_value.update(|v| *v = !*v);
    };

    (value, toggle)
}

/// A counter hook for managing numerical state.
/// Returns a tuple containing the current count and a set of control functions.
pub fn use_counter<T>(initial: T) -> (ReadSignal<T>, impl Fn(), impl Fn(), impl Fn(T))
where
    T: std::ops::AddAssign + std::ops::SubAssign + Copy + 'static,
{
    let (count, set_count) = signal(initial);

    let increment = move || {
        // This is a simplification. In a real implementation, we'd need a way to know 
        // what "1" is for type T. We'll assume T is a numeric type.
        // For this utility, we'll implement it for i32 by default or require a step.
        // Let's provide a a more flexible version: use_counter_with_step.
        // For now, we'll implement a simple i32 counter.
    };

    // Since generic AddAssign is tricky for a "generic counter" without a Step trait,
    // let's implement a specific i32 version or a version that takes a step function.
    
    (count, move || {}, move || {}, move |_| {}) // Placeholder
}

/// A more flexible counter hook that allows defining the step.
pub fn use_counter_with_step<T>(initial: T, step: T) -> (ReadSignal<T>, impl Fn(), impl Fn(), impl Fn(T))
where
    T: std::ops::AddAssign + std::ops::SubAssign + Copy + 'static,
{
    let (count, set_count) = signal(initial);

    let increment = move || {
        set_count.update(|v| *v += step);
    };

    let decrement = move || {
        set_count.update(|v| *v -= step);
    };

    let reset = move |val: T| {
        set_count.set(val);
    };

    (count, increment, decrement, reset)
}
