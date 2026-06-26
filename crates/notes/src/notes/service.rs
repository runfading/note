use common::common::{ApiPageData, ApiResponse, AppResult, DbPool, Empty};
use common::models::notes::{
    ActiveModel as NoteModel, Column as NoteColumn, Entity as NoteEntity, *,
};
use sea_orm::prelude::TimeDateTime;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbBackend, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set, Statement,
};
use tags::note_tags::service::notes_with_tags;
use time::OffsetDateTime;

pub async fn page_notes(pool: &DbPool, query: PageNotesQuery) -> AppResult<ApiPageData<NoteInfo>> {
    let page_num = query.page_num.max(1);
    let page_size = query.page_size.max(1);
    let mut finder = NoteEntity::find().order_by_desc(NoteColumn::UpdatedAt);
    if let Some(notebook_id) = query.notebook_id {
        finder = finder.filter(NoteColumn::NotebookId.eq(notebook_id));
    }

    let paginator = finder.paginate(pool, page_size);
    let total = paginator.num_items().await?;
    let data = paginator.fetch_page(page_num - 1).await?;

    // 数据组装
    let list = notes_with_tags(pool, data)
        .await?
        .into_iter()
        .map(|mut note| {
            note.content = None;
            note
        })
        .collect();

    Ok(ApiResponse::ok(ApiPageData {
        list: list,
        page_num,
        page_size,
        total,
    }))
}

pub async fn search_notes(
    pool: &DbPool,
    query: SearchNotesQuery,
) -> AppResult<ApiPageData<NoteInfo>> {
    let page_num = query.page_num.max(1);
    let page_size = query.page_size.max(1);

    let keyword_chars = query.keyword.chars().count();
    let note_ids: Vec<i64> = if keyword_chars < 3 {
        NoteEntity::find()
            .filter(
                Condition::any()
                    .add(NoteColumn::Title.contains(query.keyword.as_str()))
                    .add(NoteColumn::Content.contains(query.keyword.as_str())),
            )
            .order_by_desc(NoteColumn::UpdatedAt)
            .all(pool)
            .await?
            .into_iter()
            .map(|n| n.id)
            .collect()
    } else {
        let sql = r#"SELECT n.id FROM notes n
                     JOIN notes_fts f ON f.rowid = n.id
                     WHERE notes_fts MATCH $1
                     ORDER BY n.updated_at DESC"#;
        let rows = pool
            .query_all_raw(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                sql,
                [query.keyword.clone().into()],
            ))
            .await?;
        rows.into_iter()
            .map(|r| r.try_get_by_index::<i64>(0).unwrap_or(0))
            .collect()
    };

    let total = note_ids.len() as u64;
    let start = ((page_num - 1) * page_size) as usize;
    let page_ids: Vec<i64> = note_ids
        .into_iter()
        .skip(start)
        .take(page_size as usize)
        .collect();

    let list = if page_ids.is_empty() {
        Vec::new()
    } else {
        let notes = NoteEntity::find()
            .filter(NoteColumn::Id.is_in(page_ids))
            .all(pool)
            .await?;
        notes_with_tags(pool, notes).await?
    };

    Ok(ApiResponse::ok(ApiPageData {
        list,
        page_num,
        page_size,
        total,
    }))
}

pub async fn create_note(pool: &DbPool, create: CreateNote) -> AppResult<i64> {
    let model = NoteModel::builder()
        .set_notebook_id(create.notebook_id)
        .set_title(create.title)
        .set_content(create.content)
        .insert(pool)
        .await?;

    Ok(ApiResponse::ok(model.id))
}

pub async fn remove_note(pool: &DbPool, id: i64) -> AppResult<Empty> {
    NoteEntity::delete_by_id(id).exec(pool).await?;
    Ok(ApiResponse::empty_ok())
}

pub async fn update_note(pool: &DbPool, update: UpdateNote) -> AppResult<Empty> {
    let update_time = OffsetDateTime::now_utc();

    let mut model = NoteModel {
        id: Set(update.note_id),
        updated_at: Set(TimeDateTime::new(update_time.date(), update_time.time())),
        ..Default::default()
    };

    if let Some(v) = update.title {
        model.title = Set(v);
    }

    if let Some(v) = update.notebook_id {
        model.notebook_id = Set(v);
    }

    if let Some(v) = update.content {
        model.content = Set(v);
    }

    model.update(pool).await?;
    Ok(ApiResponse::empty_ok())
}

pub async fn note_detail(pool: &DbPool, id: i64) -> AppResult<Option<NoteInfo>> {
    let data = NoteEntity::find_by_id(id).one(pool).await?;

    if let Some(note) = data {
        let filled_data = notes_with_tags(pool, vec![note]).await?;
        Ok(ApiResponse::ok(filled_data.into_iter().find(|_note| true)))
    } else {
        Ok(ApiResponse::ok(None))
    }
}
