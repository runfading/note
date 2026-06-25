use super::service;
use common::common::{ApiPageData, ApiPageQuery, AppResult, AppState, Empty};
use common::models::notebooks::{CreateNotebook, NotebookInfo};
use tauri::State;

#[tauri::command]
pub async fn page_notebooks(
    state: State<'_, AppState>,
    query: ApiPageQuery,
) -> AppResult<ApiPageData<NotebookInfo>> {
    service::page_notebooks(&state.db, query).await
}

#[tauri::command]
pub async fn create_notebook(state: State<'_, AppState>, create: CreateNotebook) -> AppResult<i64> {
    service::create_notebook(&state.db, create).await
}

#[tauri::command]
pub async fn remove_notebook(state: State<'_, AppState>, id: i64) -> AppResult<Empty> {
    service::remove_notebook(&state.db, id).await
}
