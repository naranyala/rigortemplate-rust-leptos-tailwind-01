# Leptos CSR Dashboard Template

A high-performance, modern dashboard template built with **Leptos (CSR mode)**, **Trunk**, and **Tailwind CSS**. Provides a robust foundation for building single-page applications with Rust and WebAssembly.

## Quick Start

### Prerequisites

- **Rust**: [rustup.rs](https://rustup.rs/)
- **WASM Target**: `rustup target add wasm32-unknown-unknown`
- **Trunk**: `cargo install trunk`
- **Bun**: [bun.sh](https://bun.sh/) (used for the Tailwind CSS watcher)

### Running the Development Server

```bash
chmod +x run.sh
./run.sh
```

The script automatically detects an available port (starting at 8080) and serves the application.

## Project Architecture

### Core Stack
- **Frontend**: [Leptos 0.8](https://leptos.dev/) (Client-Side Rendering)
- **Bundler**: [Trunk](https://trunkrs.dev/)
- **Styling**: [Tailwind CSS v4](https://tailwindcss.com/) with `darkMode: "class"`
- **State Management**: Leptos Signals + Context API + Service Registry

### Directory Structure

```
src/
  lib.rs          -- Crate root, re-exports all modules
  main.rs         -- WASM entry point (mount_to_body)
  app.rs          -- Root component, page routing via Page enum
  error.rs        -- Logging helpers
  core/
    store.rs      -- GlobalStore, Theme, Page, Notification types
    services/     -- ServiceRegistry, TaskService (DI pattern)
    utils.rs      -- cn() clsx-style class concatenation utility
    error.rs      -- Core error types
  shared/
    hooks/        -- Reusable reactive primitives (use_toggle, use_window_size, etc.)
    utils/        -- General utility functions (date formatting, etc.)
  ui/
    components/   -- Btn, TheInput, BaseCard, BaseBadge, CodeBlock, Showcase, Modal, etc.
    layout/       -- MainLayout, Sidebar, TopNav (uses dark: variants)
    pages/        -- Home, Demo, DemoDetail, hooks/*, demos/*
```

### Theming

The app supports light/dark mode toggling via a top-nav button. The `dark` CSS class is toggled on the root element, and all components use Tailwind's `dark:` variant exclusively (no manual theme-signal checks in markup). Theme preference is stored in a `RwSignal<Theme>` within `GlobalStore`.

### Dependency Injection (Service Registry)

The `ServiceRegistry` implements a DI-like pattern. Services implementing the `Service` trait can be registered globally in `GlobalStore::new()` and retrieved by type from any component:

```rust
let store = use_context::<GlobalStore>().expect("GlobalStore not provided");
let task_service = store.services.get::<TaskService>().expect("TaskService not registered");
```

## Documentation

Detailed guides are in the `docs/` directory:
- [Architecture](docs/Architecture.md) -- System design, GlobalStore, Service Registry
- [Hooks](docs/Hooks.md) -- Available reactive hooks
- [Components](docs/Components.md) -- UI component library
- [State Management](docs/StateManagement.md) -- DI pattern and store
- [Routing](docs/Routing.md) -- Page/layout system
- [Development](docs/Development.md) -- Extending the template

## Features

### Hooks Library

Reusable hooks for common browser and state patterns:
- **State**: `use_toggle`, `use_counter_with_step`
- **Browser**: `use_window_size`, `use_mouse_position`, `use_media_query`, `use_online_status`, `use_dark_mode`
- **DOM**: `use_click_outside`, `use_keyboard_shortcut`, `use_element_size`, `use_element_visibility`

Each hook has its own dedicated demo page accessible from the sidebar.

### Component Showcase

Interactive library where each component is displayed with a live preview and syntax-highlighted source code via `CodeBlock` and `DemoDetail`.

## Testing

### Unit & Logic Tests

Run standard Rust tests for utilities and services:
```bash
cargo test
```

### WASM Browser Tests

Run tests requiring a browser environment:
```bash
wasm-pack test --chrome --headless
```

## Production Build

```bash
trunk build --release
```

Optimized static assets are generated in the `dist/` directory.
