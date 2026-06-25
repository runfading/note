use crate::note_tags::handler::*;
use crate::tags::handler::*;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;

pub mod note_tags;
pub mod tags;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("tags")
        .invoke_handler(tauri::generate_handler![
            search_tags,
            create_tag,
            recent_tags,
            page_notes_by_tag,
            delete_tag,
        ])
        .build()
}

pub fn init_note_tags<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("note-tags")
        .invoke_handler(tauri::generate_handler![
            query_tag_used_times,
            add_note_tag,
            delete_note_tag,
        ])
        .build()
}
