use leptos::prelude::*;
use crate::core::services::task_service::TaskService;
use crate::core::services::registry::ServiceRegistry;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Page {
    Home,
    Demo,
    Accordion,
    SlidingPanel,
    Hooks,
    // New pages for demos
    Tabs,
    Stepper,
    Stats,
    Autocomplete,
    Upload,
    Modal,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Clone)]
pub struct Notification {
    pub id: uuid::Uuid,
    pub message: String,
    pub level: NotificationLevel,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct GlobalStore {
    pub current_page: RwSignal<Page>,
    pub sidebar_open: RwSignal<bool>,
    pub theme: RwSignal<Theme>,
    pub notifications: RwSignal<Vec<Notification>>,
    pub services: ServiceRegistry,
}

impl GlobalStore {
    pub fn new() -> Self {
        let services = ServiceRegistry::new();
        services.register(TaskService::new());
        
        Self {
            current_page: RwSignal::new(Page::Home),
            sidebar_open: RwSignal::new(false),
            theme: RwSignal::new(Theme::Dark),
            notifications: RwSignal::new(Vec::new()),
            services,
        }
    }

    pub fn notify(&self, message: &str, level: NotificationLevel) {
        self.notifications.update(|notifications| {
            notifications.push(Notification {
                id: uuid::Uuid::new_v4(),
                message: message.to_string(),
                level,
            });
        });
    }

    pub fn remove_notification(&self, id: uuid::Uuid) {
        self.notifications.update(|notifications| {
            notifications.retain(|n| n.id != id);
        });
    }

    pub fn toggle_theme(&self) {
        self.theme.update(|t| {
            *t = match t {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            };
        });
    }
}

pub fn use_store() -> GlobalStore {
    use_context::<GlobalStore>().expect("GlobalStore not provided in context")
}
