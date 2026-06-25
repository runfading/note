use common::common::AppState;
use common::config::{init_config, SETTINGS};
use common::logging::init_logging;
use db::init::init_db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    init_config().expect("failed to initialize config");
    let setting = SETTINGS.get().expect("config not initialized");
    let _guard = init_logging(&setting.log);
    let pool = init_db(&setting.database)
        .await
        .expect("failed to connect database");

    tauri::Builder::default()
        .manage(AppState { db: pool })
        .plugin(tauri_plugin_opener::init())
        .plugin(notes::init())
        .plugin(tags::init())
        .plugin(tags::init_note_tags())
        .plugin(notebooks::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
