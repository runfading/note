use crate::handler::*;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;

pub mod handler;
pub mod service;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("notebooks")
        .invoke_handler(tauri::generate_handler![
            page_notebooks,
            create_notebook,
            remove_notebook,
        ])
        .build()
}
