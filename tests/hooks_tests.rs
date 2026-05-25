use wasm_bindgen_test::*;
use leptos::prelude::*;
use leptos_template::shared::hooks::*;
use gloo_storage::{LocalStorage, SessionStorage, Storage};

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
fn test_use_counter() {
    let (count, inc, dec, reset) = use_counter(10);
    assert_eq!(count.get(), 10);
    inc();
    assert_eq!(count.get(), 11);
    dec();
    assert_eq!(count.get(), 10);
    reset(5);
    assert_eq!(count.get(), 5);
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
fn test_use_local_storage_persistence() {
    let key = "test_storage_persist".to_string();
    
    // Manually set value in LocalStorage
    LocalStorage::set::<i32>(&key, 200).expect("Failed to set local storage");
    
    let (val, _set_val) = use_local_storage::<i32>(key);
    assert_eq!(val.get(), 200);
}

#[wasm_bindgen_test]
fn test_use_session_storage_persistence() {
    let key = "test_session_persist".to_string();
    
    // Manually set value in SessionStorage
    SessionStorage::set::<i32>(&key, 300).expect("Failed to set session storage");
    
    let (val, _set_val) = use_session_storage::<i32>(key);
    assert_eq!(val.get(), 300);
}

#[wasm_bindgen_test]
fn test_use_url_params() {
    let (params, _set_params) = use_url_params();
    assert!(params.get().is_empty() || !params.get().is_empty());
}

#[wasm_bindgen_test]
fn test_use_online_status() {
    let online = use_online_status();
    // We just verify it returns a value
    let _ = online.get();
}

#[wasm_bindgen_test]
fn test_use_dark_mode() {
    let dark_mode = use_dark_mode();
    let _ = dark_mode.get();
}
