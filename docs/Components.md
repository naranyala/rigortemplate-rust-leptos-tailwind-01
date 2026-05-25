# Component Library

The `src/ui/components/` module contains a set of atomic and molecular UI components built with Tailwind CSS.

## Base Components (Atomic)

### `Btn`

A highly configurable button.
- **Props**:
  - `variant`: `ButtonVariant` (Primary, Secondary, Outline, Ghost). Defaults to Primary.
  - `on_click`: Optional click callback.
  - `class`: Optional extra CSS classes.
  - `children`: Content of the button.
- **Pattern**: Used for all user interactions.

### `TheInput`

An accessible input field.
- **Props**:
  - `label`: Text label for the input.
  - `placeholder`: Optional placeholder text.
  - `value`: `RwSignal<String>`.
  - `input_type`: Optional input type (defaults to "text").
- **Note**: Takes a single `RwSignal<String>` for bidirectional binding (no separate `set_value` prop).

### `BaseCard`

A container for grouping related content.
- **Props**:
  - `title`: Optional title for the card header.
  - `children`: Content of the card.

### `BaseBadge`

Small status indicators.
- **Props**:
  - `text`: The label text.
  - `color`: Tailwind CSS color class string (e.g., `"bg-indigo-500/20 text-indigo-300"`).

### `ErrorDisplay`

Standardized error message list.
- **Props**:
  - `errors`: `StoredValue<Vec<String>>` containing error messages.

### `Showcase`

Wraps component previews with a labeled header.
- **Props**:
  - `name`: Display name.
  - `description`: Optional description text.
  - `children`: Preview content.

### `CodeBlock`

Renders syntax-highlighted Rust source code.
- **Props**:
  - `source`: `&'static str` containing the source text.

### `DemoDetail`

Page-level wrapper for component demonstrations with live rendering and source code display.
- **Props**:
  - `label`: Page title.
  - `source`: Source code string.
  - `children`: Live component preview.

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

Global notification system that listens to `GlobalStore.notifications`. Call `store.notify()` from anywhere in the app to trigger a toast.

### `StatsCard`

KPI display card with trend indicators.
- **Props**:
  - `title`: Card title.
  - `value`: The primary metric.
  - `change`: Percentage or absolute change.
  - `trend`: `Trend` (Up, Down, Neutral).
  - `icon`: `AnyView` for the visual indicator.
  - `color`: Color class for the icon.

## Theming Convention

All components rely exclusively on Tailwind's `dark:` variant for dark mode support. For example, a card uses `dark:bg-slate-800/80 dark:border-slate-700/50 bg-white border-slate-200`. No component reads the theme signal directly in its markup.
