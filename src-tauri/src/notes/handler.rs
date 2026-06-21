use super::service;
use crate::common::{ApiPageData, AppResult, AppState, Empty};
use crate::notes::models::{CreateNote, NoteInfo, PageNotesQuery, SearchNotesQuery, UpdateNote};
use tauri::State;

#[tauri::command]
pub async fn page_notes(
    state: State<'_, AppState>,
    query: PageNotesQuery,
) -> AppResult<ApiPageData<NoteInfo>> {
    service::page_notes(&state.db, query).await
}

#[tauri::command]
pub async fn search_notes(
    state: State<'_, AppState>,
    query: SearchNotesQuery,
) -> AppResult<ApiPageData<NoteInfo>> {
    service::search_notes(&state.db, query).await
}

#[tauri::command]
pub async fn create_note(state: State<'_, AppState>, create: CreateNote) -> AppResult<i64> {
    service::create_note(&state.db, create).await
}

#[tauri::command]
pub async fn remove_note(state: State<'_, AppState>, id: i64) -> AppResult<Empty> {
    service::remove_note(&state.db, id).await
}

#[tauri::command]
pub async fn update_note(state: State<'_, AppState>, update: UpdateNote) -> AppResult<Empty> {
    service::update_note(&state.db, update).await
}

#[tauri::command]
pub async fn note_detail(state: State<'_, AppState>, id: i64) -> AppResult<Option<NoteInfo>> {
    service::note_detail(&state.db, id).await
}
