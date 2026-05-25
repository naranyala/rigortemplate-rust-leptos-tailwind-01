# Routing and Layout

The application implements a custom routing and layout system using a `Page` enum and a reactive signal, avoiding the complexity of a full router library.

## Page Routing

Routing is driven by the `Page` enum in `src/core/store.rs`.

1. **State**: The `GlobalStore` holds a `current_page: RwSignal<Page>`.
2. **Navigation**: Sidebar links and buttons update this signal (e.g., `page.set(Page::Demo)`).
3. **Rendering**: The root `App` component in `src/app.rs` uses a `match` expression on `current_page.get()` to determine which page component to render.

### Page Enum Variants

The `Page` enum includes:
- **Dashboard**: `Home`, `Demo`
- **Demo patterns**: `Accordion`, `SlidingPanel`, `Tabs`, `Stepper`, `Stats`, `Autocomplete`, `Upload`, `Modal`
- **Individual hooks**: `HookToggle`, `HookCounter`, `HookWindowSize`, `HookMousePosition`, `HookMediaQuery`, `HookOnlineStatus`, `HookClickOutside`, `HookKeyboardShortcut`
- **Component demos**: `Badge`, `Button`, `Card`, `Input`, `NotificationToast`

## Layout System

The `MainLayout` component provides a structural wrapper configurable per page.

### Layout Configurations

- **Default**: Standard dashboard view with left Sidebar and Top Navigation.
- **NoSidebar**: Focused views needing more horizontal space but still showing top nav.
- **NoTopNav**: Views where the sidebar is the primary navigation.
- **Fullscreen**: Landing pages, login screens, or immersive demos.

### Implementation

`MainLayout` uses the `LayoutType` enum to conditionally render `Sidebar` and `TopNav`, wrapping children in a scrollable main content area. The sidebar toggle is accessible on mobile via a hamburger button in `TopNav`.

### Theme Toggle

The TopNav includes a theme toggle button that switches the `dark` CSS class on the root `<div>`, triggering Tailwind's `dark:` variant rules across all components.
