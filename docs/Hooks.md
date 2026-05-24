# Hooks Library

The `stdlib/hooks` module provides a collection of reactive primitives to encapsulate common browser and DOM logic.

## Window & Browser Hooks

### `use_window_size`
Tracks the current viewport dimensions.
- **Returns**: `ReadSignal<WindowSize>`
- **Usage**:
  ```rust
  let size = use_window_size();
  view! { <div>{move || format!("{}x{}", size.get().width, size.get().height)}</div> }
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

## DOM & Interaction Hooks

### `use_click_outside`
Triggers a callback when a click occurs outside a specified element.
- **Arguments**: `target: StoredNode<Div>`, `callback: F`
- **Usage**: Use with a `NodeRef` attached to the target element.

### `use_element_size`
Tracks the dimensions of a specific DOM element using `ResizeObserver`.
- **Arguments**: `target: StoredNode<T>`
- **Returns**: `ReadSignal<ElementSize>`

### `use_element_visibility`
Monitors if an element is visible in the viewport using `IntersectionObserver`.
- **Arguments**: `target: StoredNode<T>`
- **Returns**: `ReadSignal<bool>`

### `use_keyboard_shortcut`
Listens for specific key combinations.
- **Arguments**: `shortcut: Shortcut`, `callback: F`

## State & Logic Hooks

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

## Utility Hooks

### `use_url_params`
Parses and tracks URL search parameters.
- **Returns**: `(ReadSignal<HashMap<String, String>>, Callback<(String, Option<String>)>)`

### `use_idle`
Detects user inactivity based on event listeners.
- **Returns**: `ReadSignal<bool>`

### `use_storage`
Reactive wrapper around `LocalStorage`.
- **Arguments**: `key: String`
- **Returns**: `(ReadSignal<T>, WriteSignal<T>)`
