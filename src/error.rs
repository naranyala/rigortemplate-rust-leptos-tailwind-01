use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Storage(String),
    Serialization(String),
    Network(String),
    Validation(String),
    NotFound(String),
    Context(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Storage(msg) => write!(f, "Storage error: {msg}"),
            AppError::Serialization(msg) => write!(f, "Serialization error: {msg}"),
            AppError::Network(msg) => write!(f, "Network error: {msg}"),
            AppError::Validation(msg) => write!(f, "Validation error: {msg}"),
            AppError::NotFound(msg) => write!(f, "Not found: {msg}"),
            AppError::Context(msg) => write!(f, "Context error: {msg}"),
            AppError::Internal(msg) => write!(f, "Internal error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

pub fn log_error(err: &AppError) {
    leptos::logging::error!("[AppError] {err}");
}

pub fn log_warn(msg: &str) {
    leptos::logging::warn!("[App] {msg}");
}

pub fn log_info(msg: &str) {
    leptos::logging::log!("[App] {msg}");
}
