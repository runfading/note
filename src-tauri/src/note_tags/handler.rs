use crate::common::{AppResult, AppState, Empty};
use crate::note_tags::models::{AddNoteTag, TagsUsedTimesQuery, TagsUsedTimesResponse};
use crate::note_tags::service;
use tauri::State;

#[tauri::command]
pub async fn add_note_tag(
    state: State<'_, AppState>,
    note_id: i64,
    tag: AddNoteTag,
) -> AppResult<Empty> {
    service::add_note_tag(&state.db, note_id, tag).await
}

#[tauri::command]
pub async fn delete_note_tag(
    state: State<'_, AppState>,
    note_id: i64,
    tag: String,
) -> AppResult<Empty> {
    service::delete_note_tag(&state.db, note_id, tag).await
}

#[tauri::command]
pub async fn query_tag_used_times(
    state: State<'_, AppState>,
    query: TagsUsedTimesQuery,
) -> AppResult<Vec<TagsUsedTimesResponse>> {
    service::query_tag_used_times(&state.db, query).await
}
