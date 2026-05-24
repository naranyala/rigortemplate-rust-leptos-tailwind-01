# Architecture

This project is a high-performance Leptos template focused on reusability and scalability.

## Core Concepts

### Global State Management
The application uses a `GlobalStore` provided via Leptos context. It manages:
- Current page routing
- Theme state (Light/Dark)
- Global notifications
- Service Registry

### Dependency Injection (Service Registry)
The `ServiceRegistry` implements a DI-like pattern. Services that implement the `Service` trait can be registered globally. This decouples business logic from the UI components and allows for easier testing and maintenance.

### Layout System
The `MainLayout` component provides a flexible wrapper for the application. It supports multiple `LayoutType` configurations:
- `Default`: Includes both Sidebar and TopNav.
- `NoSidebar`: TopNav and Main content.
- `NoTopNav`: Sidebar and Main content.
- `Fullscreen`: Main content only.

## Directory Structure
- `src/app.rs`: Root component and layout orchestration.
- `src/store.rs`: Global state and store initialization.
- `src/services/`: Business logic and global services.
- `src/stdlib/`: Standard library of reusable hooks and components.
- `src/pages/`: Page-level components.
- `src/demos/`: Interactive examples of the stdlib.
