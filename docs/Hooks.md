# Hooks Library

The `src/shared/hooks/` module provides a collection of reactive primitives for common browser and DOM logic.

## Window and Browser Hooks

### `use_window_size`

Tracks the current viewport dimensions.
- **Returns**: `ReadSignal<WindowSize>`
- **Usage**:
  ```rust
  let size = use_window_size();
  view! { <div>{move || format!("{} x {}", size.get().width, size.get().height)}</div> }
  ```

### `use_mouse_position`

Tracks the current mouse coordinates relative to the viewport.
- **Returns**: `ReadSignal<MousePosition>`

### `use_online_status`

Monitors the browser's network connectivity.
- **Returns**: `ReadSignal<bool>`

### `use_dark_mode`

Tracks the system color scheme preference.
- **Returns**: `ReadSignal<bool>`

### `use_media_query`

Reacts to CSS media query changes.
- **Arguments**: `query: &'static str`
- **Returns**: `ReadSignal<bool>`

## DOM and Interaction Hooks

### `use_click_outside`

Triggers a callback when a click occurs outside a specified element.
- **Arguments**: `target: NodeRef<html::Div>`, `callback: F`
- **Usage**: Attach a `NodeRef` to the target element.

### `use_element_size`

Tracks the dimensions of a specific DOM element using `ResizeObserver`.
- **Arguments**: `target: NodeRef<T>`
- **Returns**: `ReadSignal<ElementSize>`

### `use_element_visibility`

Monitors if an element is visible in the viewport using `IntersectionObserver`.
- **Arguments**: `target: NodeRef<T>`
- **Returns**: `ReadSignal<bool>`

### `use_keyboard_shortcut`

Listens for specific key combinations.
- **Arguments**: `shortcut: Shortcut`, `callback: F`
- **Shortcut fields**: `key`, `ctrl`, `alt`, `shift`, `meta`

## State and Logic Hooks

### `use_toggle`

Simple boolean toggle state.
- **Returns**: `(ReadSignal<bool>, impl Fn())`

### `use_counter`

Numeric counter with basic controls.
- **Returns**: `(ReadSignal<T>, inc, dec, reset)`

### `use_counter_with_step`

Counter with a customizable step value.
- **Arguments**: `initial: T`, `step: T`
- **Returns**: `(ReadSignal<T>, inc, dec, reset)`

## Sidebar Navigation

Each hook has a dedicated sidebar item and demo page, wired via the `Page` enum in `src/core/store.rs`. The sidebar links are defined in `src/ui/layout/sidebar.rs` under the "Hooks" section. Individual hook pages use the `DemoDetail` wrapper to display a live rendering and source code.
