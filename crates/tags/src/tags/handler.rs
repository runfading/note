use super::service;
use common::common::{ApiPageData, AppResult, AppState, Empty};
use common::models::notes::NoteInfo;
use common::models::tags::{CreateTag, PageNotesByTagQuery, SearchTagsQuery, TagInfo};
use tauri::State;

#[tauri::command]
pub async fn recent_tags(state: State<'_, AppState>) -> AppResult<Vec<TagInfo>> {
    service::recent_tags(&state.db).await
}

#[tauri::command]
pub async fn create_tag(state: State<'_, AppState>, create: CreateTag) -> AppResult<i64> {
    service::create_tag(&state.db, create).await
}

#[tauri::command]
pub async fn search_tags(
    state: State<'_, AppState>,
    query: SearchTagsQuery,
) -> AppResult<Vec<TagInfo>> {
    service::search_tags(&state.db, query).await
}

#[tauri::command]
pub async fn page_notes_by_tag(
    state: State<'_, AppState>,
    id: i64,
    query: PageNotesByTagQuery,
) -> AppResult<ApiPageData<NoteInfo>> {
    service::page_notes_by_tag(&state.db, id, query).await
}

#[tauri::command]
pub async fn delete_tag(state: State<'_, AppState>, name: String) -> AppResult<Empty> {
    service::delete_tag(&state.db, name).await
}
