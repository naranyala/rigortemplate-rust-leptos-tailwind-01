# Development Guide

## Adding a New Hook
1. Create a new file in `src/stdlib/hooks/`.
2. Implement the hook function returning a `ReadSignal` or a tuple of signals/functions.
3. Export the hook in `src/stdlib/hooks/mod.rs`.
4. Add a demonstration in `src/pages/hooks.rs`.

## Adding a New Component
1. Create a new file in `src/stdlib/components/`.
2. Use `#[component]` and implement the view.
3. Export the component in `src/stdlib/components/mod.rs`.
4. Create a demo in `src/demos/` and link it via `src/app.rs` and `src/pages/demo.rs`.

## Adding a New Service
1. Define a struct in `src/services/`.
2. Implement the `Service` trait for the struct.
3. Register the service in `src/store.rs` inside `GlobalStore::new()`.
4. Access the service in components using `store.services.get::<YourService>()`.

## Building and Running
- Run `cargo check` to verify types.
- Use `trunk serve` to run the application in development mode.
