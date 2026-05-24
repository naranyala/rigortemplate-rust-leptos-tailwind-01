# Component Library

The `stdlib/components` module contains a set of atomic and molecular UI components built with Tailwind CSS.

## Base Components (Atomic)

### `BaseButton`
A highly configurable button.
- **Props**:
    - `variant`: `ButtonVariant` (Primary, Secondary, Outline, Ghost, Error).
    - `children`: Content of the button.
- **Pattern**: Used for all user interactions.

### `BaseInput`
An accessible input field.
- **Props**:
    - `label`: Text label for the input.
    - `placeholder`: Placeholder text.
    - `value`: `ReadSignal<String>`.
    - `set_value`: `WriteSignal<String>`.

### `BaseCard`
A container for grouping related content.
- **Props**:
    - `title`: Optional title for the card header.
    - `children`: Content of the card.

### `BaseBadge`
Small status indicators.
- **Props**:
    - `text`: The label text.
    - `color`: Tailwind CSS color class.

## Complex Components (Molecular)

### `Tabs`
A system for switching between different content panes.
- **Props**:
    - `items`: `Vec<Arc<TabItem>>` containing IDs, labels, and content closures.
    - `default_tab`: The ID of the tab to open by default.

### `Modal`
An accessible dialog overlay.
- **Props**:
    - `is_open`: `Signal<bool>` controlling visibility.
    - `on_close`: `Callback<()>` to handle closing.
    - `title`: Header title.
    - `children`: Modal body content.

### `NotificationToast`
Global notification system.
- **Implementation**: Listens to `GlobalStore.notifications`.
- **Usage**: Call `store.notify()` from anywhere in the app.

### `StatsCard`
KPI display card with trend indicators.
- **Props**:
    - `title`: Card title.
    - `value`: The primary metric.
    - `change`: Percentage or absolute change.
    - `trend`: `Trend` (Up, Down, Neutral).
    - `icon`: `AnyView` for the visual indicator.
    - `color`: Color class for the icon.

## Utility Components
- `Showcase`: Wraps a component with a label and a source code block.
- `CodeBlock`: Renders syntax-highlighted Rust code.
- `ErrorDisplay`: Standardized error message component.
