# Hooks Library

The `stdlib/hooks` module provides a collection of reactive primitives to encapsulate common browser and DOM logic.

## Available Hooks

### Window & Browser
- `use_window_size`: Tracks the current viewport dimensions.
- `use_mouse_position`: Tracks the current mouse coordinates.
- `use_online_status`: Monitors the browser's network connectivity.
- `use_dark_mode`: Tracks the system color scheme preference.
- `use_media_query`: Reacts to CSS media query changes.

### DOM & Interaction
- `use_click_outside`: Triggers a callback when a click occurs outside a specified element.
- `use_element_size`: Tracks the dimensions of a specific DOM element.
- `use_element_visibility`: Monitors if an element is visible in the viewport.
- `use_keyboard_shortcut`: Listens for specific key combinations.

### State & Logic
- `use_toggle`: Simple boolean toggle state.
- `use_counter`: Numeric counter with increment, decrement, and reset.
- `use_counter_with_step`: Counter with a customizable step value.

### Utilities
- `use_url_params`: Parses and tracks URL search parameters.
- `use_idle`: Detects user inactivity.
- `use_storage`: Reactive wrapper around LocalStorage.
