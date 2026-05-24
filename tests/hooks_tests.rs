use wasm_bindgen_test::*;
use leptos::prelude::*;
use leptos_template::stdlib::hooks::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_use_toggle() {
    let (val, toggle) = use_toggle(false);
    assert_eq!(val.get(), false);
    toggle();
    assert_eq!(val.get(), true);
    toggle();
    assert_eq!(val.get(), false);
}

#[wasm_bindgen_test]
fn test_use_counter_with_step() {
    let (count, inc, dec, reset) = use_counter_with_step(10, 5);
    assert_eq!(count.get(), 10);
    inc();
    assert_eq!(count.get(), 15);
    dec();
    assert_eq!(count.get(), 10);
    reset(0);
    assert_eq!(count.get(), 0);
}

#[wasm_bindgen_test]
fn test_use_local_storage() {
    let key = "test_storage_key".to_string();
    let (val, set_val) = use_local_storage(key.clone());
    
    set_val.set(100);
    assert_eq!(val.get(), 100);
}
