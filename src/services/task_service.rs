use leptos::prelude::*;
use serde::{Serialize, Deserialize};
use gloo_storage::{LocalStorage, Storage};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::error::{self, AppError};
use crate::services::registry::Service;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: String,
    pub text: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

pub struct TaskService {
    pub tasks: RwSignal<Vec<Task>>,
}

impl Service for TaskService {}

const STORAGE_KEY: &str = "leptos_tasks";

impl TaskService {
    pub fn new() -> Self {
        let initial_tasks = match LocalStorage::get::<Vec<Task>>(STORAGE_KEY) {
            Ok(tasks) => tasks,
            Err(e) => {
                error::log_warn(&format!("Could not load tasks from storage: {e}. Starting fresh."));
                Vec::new()
            }
        };
        let tasks = RwSignal::new(initial_tasks);

        Effect::new(move |_| {
            let current = tasks.get();
            if let Err(e) = LocalStorage::set(STORAGE_KEY, &current) {
                error::log_error(&AppError::Storage(format!("Failed to persist tasks: {e}")));
            }
        });

        Self { tasks }
    }

    pub fn add_task(&self, text: String) {
        self.tasks.update(|t| {
            t.push(Task {
                id: Uuid::new_v4().to_string(),
                text,
                completed: false,
                created_at: Utc::now(),
            });
        });
    }

    pub fn toggle_task(&self, id: String) {
        self.tasks.update(|t| {
            match t.iter_mut().find(|task| task.id == id) {
                Some(task) => task.completed = !task.completed,
                None => error::log_warn(&format!("toggle_task: task {id} not found")),
            }
        });
    }

    pub fn remove_task(&self, id: String) {
        let prev_len = self.tasks.with(|t| t.len());
        self.tasks.update(|t| {
            t.retain(|task| task.id != id);
        });
        let removed = self.tasks.with(|t| prev_len - t.len());
        if removed == 0 {
            error::log_warn(&format!("remove_task: task {id} not found"));
        }
    }

    pub fn get_progress(&self) -> f32 {
        let t = self.tasks.get();
        if t.is_empty() {
            0.0
        } else {
            (t.iter().filter(|task| task.completed).count() as f32 / t.len() as f32) * 100.0
        }
    }
}
