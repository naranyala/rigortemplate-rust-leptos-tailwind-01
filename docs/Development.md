# Development Guide

## Adding a New Hook

1. Create a new file in `src/shared/hooks/`.
2. Implement the hook function returning a `ReadSignal`, `RwSignal`, or a tuple of signals/functions.
3. Export the hook in `src/shared/hooks/mod.rs`.
4. Add a `Page` variant in `src/core/store.rs`.
5. Create a component in `src/ui/pages/hooks.rs`.
6. Wire the component in `src/app.rs` via the Page match block.
7. Add a sidebar entry in `src/ui/layout/sidebar.rs`.

## Adding a New Component

1. Create a new file in `src/ui/components/`.
2. Use `#[component]` and implement the view with Tailwind classes. Use `dark:` variants for dark mode.
3. Export the component in `src/ui/components/mod.rs`.
4. Create a demo in `src/ui/pages/demos/` if the component warrants an interactive showcase.
5. Wire the demo page via `src/app.rs` and `src/ui/pages/demo.rs`.

## Adding a New Service

1. Define a struct in `src/core/services/`.
2. Implement the `Service` trait for the struct.
3. Register the service in `src/core/store.rs` inside `GlobalStore::new()`.
4. Access the service in components:
   ```rust
   let store = use_context::<GlobalStore>().expect("GlobalStore not provided");
   let service = store.services.get::<YourService>().expect("...");
   ```

## Adding a New Page

1. Add a variant to the `Page` enum in `src/core/store.rs`.
2. Create the page component in `src/ui/pages/` (or add to an existing file).
3. Add a match arm in `src/app.rs` for the new page variant.
4. Optionally add a sidebar link in `src/ui/layout/sidebar.rs`.

## Theming Convention

All new components must use Tailwind's `dark:` variant for dark mode support. Never read the theme signal directly in component markup. Example:

```rust
<div class="dark:bg-slate-800 bg-white dark:text-slate-200 text-slate-900">
```

## Building and Running

- `cargo check` -- Verify types compile.
- `trunk serve` -- Run the application in development mode (opens on port 8080 by default).
- `trunk build --release` -- Production build to `dist/`.
