mod handler;
mod models;
pub mod service;

use crate::common::HandlerRegistrar;
use tauri::plugin::TauriPlugin;
use tauri::Wry;

pub fn routes() -> TauriPlugin<Wry> {
    tauri::plugin::Builder::new("notebooks")
        .invoke_handler(tauri::generate_handler![
            handler::page_notebooks,
            handler::create_notebook,
            handler::remove_notebook,
        ])
        .build()
}

inventory::submit!(HandlerRegistrar { handler_fn: routes });
