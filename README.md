<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos CSR Dashboard Template

A high-performance, modern dashboard template built with **Leptos (CSR mode)**, **Trunk**, and **Tailwind CSS**. This template provides a robust foundation for building single-page applications (SPAs) with Rust and WebAssembly.

## Quick Start

### Prerequisites

Ensure you have the following installed:
- **Rust**: [rustup.rs](https://rustup.rs/)
- **WASM Target**: `rustup target add wasm32-unknown-unknown`
- **Trunk**: `cargo install trunk`
- **Bun**: [bun.sh](https://bun.sh/) (used for the Tailwind CSS watcher)

### Running the Development Server

The project includes a convenience script to handle Tailwind CSS and the Trunk dev server:

```bash
chmod +x run.sh
./run.sh
```

The script will automatically detect an available port (starting at 8080) and serve the application.

## Project Architecture

### Core Stack
- **Frontend**: [Leptos 0.8](https://leptos.dev/) (Client-Side Rendering)
- **Bundler**: [Trunk](https://trunkrs.dev/)
- **Styling**: [Tailwind CSS v4](https://tailwindcss.com/)
- **State Management**: Leptos Signals & Context API

### Directory Structure
- `src/app.rs`: Root application component and layout orchestration.
- `src/store.rs`: Global state and service registry.
- `src/pages/`: Main views (Home, Demo, Hooks).
- `src/components/`: Shared UI components (Sidebar, TopNav, Layout).
- `src/stdlib/`: The "Standard Library" of the project:
    - `components/`: Atomic UI components (Buttons, Inputs, Cards).
    - `hooks/`: Reusable reactive primitives (use_window_size, use_local_storage, etc.).
    - `utils/`: General utility functions and syntax highlighting.
- `src/services/`: Global business logic services.
- `src/demos/`: Source code for the interactive demos.

## Documentation

Detailed guides are available in the `docs/` directory:
- [Architecture](docs/Architecture.md): System design, GlobalStore, and Service Registry.
- [Hooks](docs/Hooks.md): Comprehensive list of available reactive hooks.
- [Components](docs/Components.md): Documentation of the UI component library.
- [State Management](docs/StateManagement.md): In-depth look at the DI pattern and store.
- [Routing](docs/Routing.md): How the page and layout system works.
- [Development](docs/Development.md): Guide for extending the template.

## Features

### Hooks Library
A rich collection of reusable hooks for common browser and state patterns:
- **State**: `use_toggle`, `use_counter_with_step`, `use_local_storage`.
- **Browser**: `use_window_size`, `use_mouse_position`, `use_media_query`, `use_online_status`, `use_dark_mode`.
- **DOM**: `use_click_outside`, `use_keyboard_shortcut`, `use_element_size`, `use_element_visibility`.

### Component Showcase
An interactive library where each component is displayed with:
- A live, interactive preview.
- A syntax-highlighted source code block.

## Testing

### Unit & Logic Tests
Run standard Rust tests for utilities and services:
```bash
cargo test
```

### WASM Browser Tests
Run tests that require a browser environment (e.g., hooks, storage):
```bash
wasm-pack test --chrome --headless
```

## Production Build

To compile the project for production:
```bash
trunk build --release
```
The optimized static assets will be generated in the `dist/` directory, ready to be served by any static file host.
