mod handler;
pub mod models;
pub mod service;

use crate::common::HandlerRegistrar;
use tauri::plugin::TauriPlugin;
use tauri::Wry;

pub fn routes() -> TauriPlugin<Wry> {
    tauri::plugin::Builder::new("tags")
        .invoke_handler(tauri::generate_handler![
            handler::search_tags,
            handler::create_tag,
            handler::recent_tags,
            handler::page_notes_by_tag,
            handler::delete_tag,
        ])
        .build()
}

inventory::submit!(HandlerRegistrar { handler_fn: routes });
