use leptos::prelude::*;

pub const SOURCE: &str = include_str!("state.rs");

/// A simple boolean toggle hook.
/// Returns a tuple containing a read signal for the value and a function to toggle it.
pub fn use_toggle(initial: bool) -> (ReadSignal<bool>, impl Fn()) {
    let (value, set_value) = signal(initial);
    
    let toggle = move || {
        set_value.update(|v| *v = !*v);
    };

    (value, toggle)
}

/// A counter hook that increments/decrements by 1.
/// For a configurable step, use `use_counter_with_step`.
pub fn use_counter(initial: i32) -> (ReadSignal<i32>, impl Fn(), impl Fn(), impl Fn(i32)) {
    use_counter_with_step(initial, 1)
}

/// A more flexible counter hook that allows defining the step.
pub fn use_counter_with_step<T>(initial: T, step: T) -> (ReadSignal<T>, impl Fn(), impl Fn(), impl Fn(T))
where
    T: std::ops::AddAssign + std::ops::SubAssign + Copy + Send + Sync + 'static,
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
