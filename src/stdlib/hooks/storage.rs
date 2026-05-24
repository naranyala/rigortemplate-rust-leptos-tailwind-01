use leptos::prelude::*;
use serde::{Serialize, de::DeserializeOwned};
use gloo_storage::{LocalStorage, Storage};
use crate::error::{self, AppError};

pub fn use_local_storage<T>(key: String) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Serialize + DeserializeOwned + Clone + std::default::Default + Send + Sync + 'static,
{
    let initial_value = match LocalStorage::get::<T>(&key) {
        Ok(val) => val,
        Err(e) => {
            error::log_warn(&format!("use_local_storage(\"{key}\"): {e}. Using default."));
            T::default()
        }
    };
    let (value, set_value) = signal(initial_value);

    Effect::new(move |_| {
        let current = value.get();
        if let Err(e) = LocalStorage::set(&key, &current) {
            error::log_error(&AppError::Storage(format!(
                "use_local_storage(\"{key}\"): failed to persist: {e}"
            )));
        }
    });

    (value, set_value)
}
