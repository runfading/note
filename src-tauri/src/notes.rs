mod handler;
pub mod models;
pub mod service;

use crate::common::HandlerRegistrar;
use tauri::plugin::TauriPlugin;
use tauri::Wry;

pub fn routes() -> TauriPlugin<Wry> {
    tauri::plugin::Builder::new("notes")
        .invoke_handler(tauri::generate_handler![
            handler::page_notes,
            handler::update_note,
            handler::note_detail,
            handler::search_notes,
            handler::create_note,
            handler::remove_note,
        ])
        .build()
}

inventory::submit!(HandlerRegistrar { handler_fn: routes });
