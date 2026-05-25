use wasm_bindgen_test::*;
use leptos::prelude::*;
use leptos_template::core::services::task_service::TaskService;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_add_task() {
    let service = TaskService::new();
    service.add_task("Test Task".to_string());
    
    assert_eq!(service.tasks.get().len(), 1);
    assert_eq!(service.tasks.get()[0].text, "Test Task");
}

#[wasm_bindgen_test]
fn test_toggle_task() {
    let service = TaskService::new();
    service.add_task("Toggle Me".to_string());
    let id = service.tasks.get()[0].id.clone();
    
    service.toggle_task(id.clone());
    assert!(service.tasks.get()[0].completed);
    
    service.toggle_task(id);
    assert!(!service.tasks.get()[0].completed);
}

#[wasm_bindgen_test]
fn test_toggle_non_existent_task() {
    let service = TaskService::new();
    service.add_task("Task 1".to_string());
    let fake_id = "non-existent".to_string();
    
    service.toggle_task(fake_id);
    assert_eq!(service.tasks.get().len(), 1);
    assert!(!service.tasks.get()[0].completed);
}

#[wasm_bindgen_test]
fn test_remove_task() {
    let service = TaskService::new();
    service.add_task("Task 1".to_string());
    service.add_task("Task 2".to_string());
    let id = service.tasks.get()[0].id.clone();
    
    service.remove_task(id);
    assert_eq!(service.tasks.get().len(), 1);
    assert_eq!(service.tasks.get()[0].text, "Task 2");
}

#[wasm_bindgen_test]
fn test_remove_non_existent_task() {
    let service = TaskService::new();
    service.add_task("Task 1".to_string());
    let fake_id = "non-existent".to_string();
    
    service.remove_task(fake_id);
    assert_eq!(service.tasks.get().len(), 1);
}

#[wasm_bindgen_test]
fn test_progress_calculation() {
    let service = TaskService::new();
    
    // Edge case: Empty list
    assert_eq!(service.get_progress(), 0.0);
    
    service.add_task("T1".to_string());
    service.add_task("T2".to_string());
    
    assert_eq!(service.get_progress(), 0.0);
    
    let id = service.tasks.get()[0].id.clone();
    service.toggle_task(id);
    
    assert_eq!(service.get_progress(), 50.0);
    
    let id2 = service.tasks.get()[1].id.clone();
    service.toggle_task(id2);
    
    assert_eq!(service.get_progress(), 100.0);
}
