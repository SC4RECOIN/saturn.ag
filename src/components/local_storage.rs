use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;

pub fn use_local_storage(key: String, initial_value: String) -> (String, Callback<(String,), ()>) {
    if let Err(e) = LocalStorage::set(&key, &initial_value) {
        log::error!("Failed to set initial value in LocalStorage: {}", e);
    }

    let stored_value: String = LocalStorage::get(key.clone()).unwrap_or(initial_value.clone());
    let (stored_value, set_stored_value) = signal(stored_value);

    let set_value = {
        let key = key.clone();
        Callback::from(move |value: String| {
            set_stored_value.set(value.clone());
            if let Err(e) = LocalStorage::set(&key, &initial_value) {
                log::error!("Failed to set value in LocalStorage: {}", e);
            }
        })
    };

    (stored_value.get_untracked(), set_value)
}
