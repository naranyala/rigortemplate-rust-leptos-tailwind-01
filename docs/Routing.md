# Routing and Layout

The application implements a custom routing and layout system that avoids the complexity of full-scale routing libraries for simple dashboard needs.

## Page Routing
Routing is driven by the `Page` enum in `src/store.rs`.

1. **State**: The `GlobalStore` holds a `current_page: RwSignal<Page>`.
2. **Dispatch**: Navigation is performed by updating this signal (e.g., `page.set(Page::Demo)`).
3. **Rendering**: The root `App` component uses a `match` expression on `current_page.get()` to determine which page component to render.

## Layout System
The `MainLayout` component provides a structural wrapper that can be configured per page or globally.

### Layout Configurations
- **Default**: The standard dashboard view with a left Sidebar and a Top Navigation bar.
- **NoSidebar**: Used for focused views that require more horizontal space but still need top-level navigation.
- **NoTopNav**: Used for views where the sidebar is the primary navigation and the top bar is unnecessary.
- **Fullscreen**: Used for landing pages, login screens, or immersive demos.

### Implementation
The `MainLayout` component uses the `LayoutType` enum to conditionally render the `Sidebar` and `TopNav` components, wrapping the children in a scrollable main content area.

## Page-Level Layouts
While the `App` currently uses `LayoutType::Default` globally, it can be easily modified to provide different layouts based on the current page:

```rust
let layout = match current_page.get() {
    Page::Home => LayoutType::Default,
    Page::ModalDemo => LayoutType::Fullscreen,
    _ => LayoutType::Default,
};

view! {
    <MainLayout layout_type=layout>
        {render_page()}
    </MainLayout>
}
```
