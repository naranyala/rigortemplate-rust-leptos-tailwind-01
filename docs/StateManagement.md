# State Management

This project utilizes a hybrid state management approach combining local signals, global context, and a service registry.

## Local State
For component-specific state, standard Leptos `signal` and `RwSignal` are used. This ensures minimal re-renders and high performance.

## Global Store
The `GlobalStore` is provided as a context at the root of the application (`App.rs`). It is the single source of truth for:
- **UI State**: Current page, theme, and sidebar status.
- **Notifications**: A reactive list of active notifications.
- **Registry**: The entry point for all global services.

## Service Registry (Dependency Injection)
To avoid "prop drilling" and prevent the `GlobalStore` from becoming a monolith, the project uses a `ServiceRegistry`.

### How it Works
1. **Service Trait**: Any struct that implements `Service` (which requires `Any + Send + Sync`) can be registered.
2. **Registration**: Services are instantiated and registered in `GlobalStore::new()`.
3. **Retrieval**: Components retrieve services via the registry using their type:
   ```rust
   let store = use_context::<GlobalStore>().expect("...");
   let task_service = store.services.get::<TaskService>().expect("...");
   ```

### Benefits
- **Decoupling**: Components depend on the service interface, not the store's internal structure.
- **Scalability**: New services can be added without changing the `GlobalStore` struct definition.
- **Testability**: Services can be easily mocked or replaced in tests.
