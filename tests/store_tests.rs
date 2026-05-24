use wasm_bindgen_test::*;
use leptos::prelude::*;
use leptos_template::store::{GlobalStore, Theme};
use leptos_template::services::registry::{ServiceRegistry, Service};
use leptos_template::services::task_service::TaskService;

wasm_bindgen_test_configure!(run_in_browser);

struct MockService;
impl Service for MockService {}

#[wasm_bindgen_test]
fn test_service_registry_registration() {
    let registry = ServiceRegistry::new();
    registry.register(MockService);
    
    let service = registry.get::<MockService>();
    assert!(service.is_some());
}

#[wasm_bindgen_test]
fn test_service_registry_not_found() {
    let registry = ServiceRegistry::new();
    let service = registry.get::<MockService>();
    assert!(service.is_none());
}

#[wasm_bindgen_test]
fn test_service_registry_multiple_services() {
    struct MockService2;
    impl Service for MockService2 {}
    
    let registry = ServiceRegistry::new();
    registry.register(MockService);
    registry.register(MockService2);
    
    assert!(registry.get::<MockService>().is_some());
    assert!(registry.get::<MockService2>().is_some());
}

#[wasm_bindgen_test]
fn test_global_store_theme_toggle() {
    let store = GlobalStore::new();
    let initial_theme = store.theme.get();
    
    store.toggle_theme();
    let new_theme = store.theme.get();
    
    assert_ne!(initial_theme, new_theme);
    
    store.toggle_theme();
    assert_eq!(store.theme.get(), initial_theme);
}

#[wasm_bindgen_test]
fn test_global_store_notifications() {
    let store = GlobalStore::new();
    
    store.notify("Test Message", leptos_template::store::NotificationLevel::Info);
    assert_eq!(store.notifications.get().len(), 1);
    assert_eq!(store.notifications.get()[0].message, "Test Message");
    
    let id = store.notifications.get()[0].id;
    store.remove_notification(id);
    assert_eq!(store.notifications.get().len(), 0);
}


#[wasm_bindgen_test]
fn test_global_store_service_integration() {
    let store = GlobalStore::new();
    let ts = store.services.get::<TaskService>();
    assert!(ts.is_some());
}
