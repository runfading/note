use crate::common::HandlerRegistrar;
use tauri::plugin::TauriPlugin;
use tauri::Wry;

mod handler;
pub mod models;
pub mod service;

pub fn routes() -> TauriPlugin<Wry> {
    tauri::plugin::Builder::new("note_tags")
        .invoke_handler(tauri::generate_handler![
            handler::query_tag_used_times,
            handler::add_note_tag,
            handler::delete_note_tag,
        ])
        .build()
}

inventory::submit!(HandlerRegistrar { handler_fn: routes });
