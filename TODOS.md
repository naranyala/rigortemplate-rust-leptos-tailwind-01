# Project Roadmap & TODOs

## UI & UX Improvements
- [x] Implement a global Theme Provider for light/dark mode switching.
- [ ] Add transition animations between page navigations.
- [ ] Implement a responsive mobile-first navigation menu.
- [ ] Add a loading skeleton state for async operations.
- [ ] Improve the "Sliding Panel" demo to support dynamic content injection.

## Hooks Library Expansion
- [ ] `use_fetch`: A wrapper around `reqwest` with loading/error states.
- [x] `use_intersection_observer`: A more generic version of the current visibility hook.
- [x] `use_session_storage`: Mirror of `use_local_storage` for session scope.
- [x] `use_url_params`: Comprehensive URL parameter management.
- [x] `use_idle`: Detect user inactivity.

## Component Library Growth
- [ ] Implement a data table with sorting and filtering.
- [x] Add a Modal/Dialog system with accessibility (focus trapping).
- [ ] Create a Tooltip/Popover component.
- [ ] Implement a complex Form system with validation (using `validator` crate).
- [x] Add a Toast/Notification system.
- [x] Implement a Tabs system for content switching.
- [ ] Create a Stepper/Wizard component for multi-step processes.
- [x] Add a set of KPI/Stats cards with trend indicators.
- [ ] Build a Search-with-Autocomplete demo.
- [ ] Implement a File Upload zone with drag-and-drop support.

## Engineering & Performance
- [ ] Implement a more robust Error Boundary system.
- [ ] Optimize WASM binary size (e.g., using `wasm-opt`).
- [ ] Move from `closure.forget()` to a managed closure registry to prevent memory leaks in hooks.
- [ ] Implement full End-to-End (E2E) tests with Playwright.
- [ ] Add CI/CD pipeline for automated testing and deployment.
- [x] Implement global ServiceRegistry for dependency injection.
- [x] Implement reusable MainLayout system.

## Documentation
- [x] Write detailed API documentation for the `stdlib/hooks` module.
- [x] Create a "Getting Started" guide for adding new components to the showcase.
- [x] Document the custom syntax highlighting logic.
- [x] Create Architecture and Development guides.
