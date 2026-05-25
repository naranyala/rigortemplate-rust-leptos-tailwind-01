# Project Roadmap and TODOs

## UI and UX Improvements
- [x] Implement a global Theme Provider for light/dark mode switching using Tailwind `dark:` variants
- [ ] Add transition animations between page navigations
- [ ] Implement a responsive mobile-first navigation menu (sidebar toggle exists for mobile)
- [ ] Add a loading skeleton state for async operations
- [ ] Improve the "Sliding Panel" demo to support dynamic content injection

## Hooks Library Expansion
- [ ] `use_fetch`: A wrapper around `reqwest` with loading/error states
- [x] `use_intersection_observer`: A more generic version of the current visibility hook
- [x] `use_session_storage`: Mirror of `use_local_storage` for session scope
- [x] `use_url_params`: Comprehensive URL parameter management
- [x] `use_idle`: Detect user inactivity

## Component Library Growth
- [ ] Implement a data table with sorting and filtering
- [x] Add a Modal/Dialog system with accessibility (focus trapping)
- [ ] Create a Tooltip/Popover component
- [ ] Implement a complex Form system with validation (using `validator` crate)
- [x] Add a Toast/Notification system
- [x] Implement a Tabs system for content switching
- [ ] Create a Stepper/Wizard component for multi-step processes
- [x] Add a set of KPI/Stats cards with trend indicators
- [ ] Build a Search-with-Autocomplete demo
- [ ] Implement a File Upload zone with drag-and-drop support

## Engineering and Performance
- [ ] Implement a more robust Error Boundary system
- [ ] Optimize WASM binary size (e.g., using `wasm-opt`)
- [ ] Move from `closure.forget()` to a managed closure registry to prevent memory leaks in hooks
- [ ] Implement full End-to-End (E2E) tests with Playwright
- [ ] Add CI/CD pipeline for automated testing and deployment
- [x] Implement global ServiceRegistry for dependency injection
- [x] Implement reusable MainLayout system
- [x] Replace manual theme checks with Tailwind `dark:` variants across all components

## Documentation
- [x] Write detailed API documentation for the hooks module
- [x] Create a "Getting Started" guide for adding new components to the showcase
- [x] Document the custom syntax highlighting logic
- [x] Create Architecture and Development guides
