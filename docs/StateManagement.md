# State Management

This project uses a hybrid state management approach combining local signals, global context, and a service registry.

## Local State

For component-specific state, standard Leptos `RwSignal` and derived signals are used. This ensures minimal re-renders and high performance.

```rust
let count = RwSignal::new(0);
// read: count.get()
// write: count.set(1) or count.update(|n| *n += 1)
```

## Global Store

The `GlobalStore` (defined in `src/core/store.rs`) is provided as a Leptos context at the root of the application. It is the single source of truth for:
- **UI State**: Current page (`RwSignal<Page>`), theme (`RwSignal<Theme>`), sidebar open state (`RwSignal<bool>`).
- **Notifications**: A reactive list of active `Notification` structs.
- **Registry**: The entry point for all global services via `ServiceRegistry`.

### Theme

The `Theme` enum has two variants: `Light` and `Dark`. The `MainLayout` toggles the `dark` CSS class on the root element based on `theme.get()`. All components use Tailwind's `dark:` variant for styling -- no manual theme checks in component markup.

```rust
store.toggle_theme(); // switches between Light and Dark
```

## Service Registry (Dependency Injection)

To avoid prop drilling and keep `GlobalStore` focused, the project uses a `ServiceRegistry`.

### How It Works

1. **Service Trait**: Any struct implementing `Service` (requires `Any + Send + Sync`) can be registered.
2. **Registration**: Services are instantiated and registered in `GlobalStore::new()`.
3. **Retrieval**: Components retrieve services by type:
   ```rust
   let store = use_context::<GlobalStore>().expect("GlobalStore not provided");
   let task_service = store.services.get::<TaskService>().expect("TaskService not registered");
   ```

### Benefits

- **Decoupling**: Components depend on the service interface, not the store's internal structure.
- **Scalability**: New services can be added without changing `GlobalStore`'s struct definition.
- **Testability**: Services can be mocked or replaced in tests.
