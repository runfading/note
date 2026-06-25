use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;

pub mod notes;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("notes")
        .invoke_handler(tauri::generate_handler![
            notes::handler::page_notes,
            notes::handler::update_note,
            notes::handler::note_detail,
            notes::handler::search_notes,
            notes::handler::create_note,
            notes::handler::remove_note,
        ])
        .build()
}
