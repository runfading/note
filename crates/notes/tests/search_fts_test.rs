use common::models::notebooks::ActiveModel as NotebookModel;
use common::models::notes::{ActiveModel as NoteModel, Entity as NoteEntity, SearchNotesQuery};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm_migration::MigratorTrait;
use time::OffsetDateTime;

async fn setup_db() -> common::common::DbPool {
    let conn = sea_orm::Database::connect("sqlite::memory:").await.unwrap();
    db::migration::Migrator::up(&conn, None).await.unwrap();

    let now = OffsetDateTime::now_utc();
    let ts = sea_orm::prelude::TimeDateTime::new(now.date(), now.time());
    let nb = NotebookModel {
        name: Set("notebook-1".to_string()),
        created_at: Set(ts),
        updated_at: Set(ts),
        ..Default::default()
    }
    .insert(&conn)
    .await
    .unwrap();

    let mk_note = |title: &str, content: &str| NoteModel {
        notebook_id: Set(nb.id),
        title: Set(title.to_string()),
        content: Set(content.to_string()),
        created_at: Set(ts),
        updated_at: Set(ts),
        ..Default::default()
    };

    NoteEntity::insert(mk_note("Rust 笔记", "关于所有权和借用的内容"))
        .exec(&conn)
        .await
        .unwrap();
    NoteEntity::insert(mk_note("Go 笔记", "goroutine 和 channel"))
        .exec(&conn)
        .await
        .unwrap();
    NoteEntity::insert(mk_note("Python 入门", "list dict tuple"))
        .exec(&conn)
        .await
        .unwrap();

    conn
}

#[tokio::test]
async fn search_notes_fts_matches_chinese_substring() {
    let pool = setup_db().await;
    let result = notes::notes::service::search_notes(
        &pool,
        SearchNotesQuery {
            page_num: 1,
            page_size: 10,
            keyword: "所有权".to_string(),
        },
    )
    .await
    .unwrap();

    let data = result.data.unwrap();
    assert_eq!(data.total, 1, "should match the Rust note");
    assert_eq!(data.list[0].title, "Rust 笔记");
}

#[tokio::test]
async fn search_notes_short_query_falls_back_to_like() {
    let pool = setup_db().await;
    let result = notes::notes::service::search_notes(
        &pool,
        SearchNotesQuery {
            page_num: 1,
            page_size: 10,
            keyword: "笔".to_string(),
        },
    )
    .await
    .unwrap();

    let data = result.data.unwrap();
    assert_eq!(data.total, 2, "should match both notes with '笔' in title");
}

#[tokio::test]
async fn search_notes_no_match_returns_empty() {
    let pool = setup_db().await;
    let result = notes::notes::service::search_notes(
        &pool,
        SearchNotesQuery {
            page_num: 1,
            page_size: 10,
            keyword: "不存在的内容xyz".to_string(),
        },
    )
    .await
    .unwrap();

    let data = result.data.unwrap();
    assert_eq!(data.total, 0);
    assert!(data.list.is_empty());
}
