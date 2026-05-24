use leptos::prelude::*;
use crate::services::task_service::TaskService;

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Home,
    Demo,
    Accordion,
    SlidingPanel,
    Hooks,
}

#[derive(Clone)]
pub struct GlobalStore {
    pub current_page: RwSignal<Page>,
    pub sidebar_open: RwSignal<bool>,
    pub task_service: std::sync::Arc<TaskService>,
}

impl GlobalStore {
    pub fn new() -> Self {
        Self {
            current_page: RwSignal::new(Page::Home),
            sidebar_open: RwSignal::new(false),
            task_service: std::sync::Arc::new(TaskService::new()),
        }
    }
}
