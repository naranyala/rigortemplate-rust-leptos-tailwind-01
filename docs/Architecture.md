# Architecture

This project is a Leptos CSR template focused on reusability and scalability.

## Core Concepts

### Global State Management

The application uses a `GlobalStore` provided via Leptos context. It manages:
- Current page routing (via `Page` enum)
- Theme state (Light/Dark, toggling the `dark` CSS class on the root element)
- Global notifications
- Service Registry for dependency injection

### Dependency Injection (Service Registry)

The `ServiceRegistry` implements a DI-like pattern. Services implementing the `Service` trait can be registered globally. This decouples business logic from UI components and allows for easier testing and maintenance.

Services are registered in `GlobalStore::new()` and retrieved by type:
```rust
let task_service = store.services.get::<TaskService>().expect("...");
```

### Layout System

The `MainLayout` component provides a flexible wrapper. It supports multiple `LayoutType` configurations:
- `Default`: Includes both Sidebar and TopNav.
- `NoSidebar`: TopNav and main content.
- `NoTopNav`: Sidebar and main content.
- `Fullscreen`: Main content only.

### Theming Strategy

Theming uses Tailwind's built-in `dark:` variant system with `darkMode: "class"` in `tailwind.config.js`. The `MainLayout` adds/removes the `dark` class on the root `<div>` based on `GlobalStore.theme`. All components use `dark:` classes exclusively -- no manual theme-signal checks appear in view markup.

## The Reactive Cycle

The application follows a unidirectional data flow:
1. **Service**: Manages raw data and business logic (e.g., `TaskService`) using `RwSignal`.
2. **Store**: Holds the `ServiceRegistry` and global UI state, provided as context.
3. **Hooks**: Encapsulate logic for browser or state behaviors, often interacting with services or the store.
4. **Component**: Consumes signals from hooks or services and renders the view.

## Source Layout

```
src/
  lib.rs          -- Crate root
  main.rs         -- WASM entry point
  app.rs          -- Root component and page routing
  core/
    store.rs      -- GlobalStore, Theme, Page, Notification
    services/     -- ServiceRegistry, TaskService
    utils.rs      -- cn() clsx-style utility
    error.rs      -- Error types
  shared/
    hooks/        -- Reactive primitives (use_toggle, use_window_size, etc.)
    utils/        -- Date formatting and general utilities
  ui/
    components/   -- Btn, TheInput, BaseCard, BaseBadge, CodeBlock, Showcase, etc.
    layout/       -- MainLayout, Sidebar, TopNav
    pages/        -- Home, Demo, DemoDetail, hooks/*, demos/*
```
