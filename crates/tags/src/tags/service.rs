use crate::note_tags::service::notes_with_tags;
use common::common::{ApiPageData, ApiResponse, AppResult, DbPool, Empty};
use common::models::note_tags::{Column as NoteTagColumn, Entity as NoteTagEntity};
use common::models::notes::{Column as NoteColumn, Entity as NoteEntity, NoteInfo};
use common::models::tags::{
    ActiveModel as TagModel, Column as TagColumn, CreateTag, Entity as TagEntity, *,
};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};

pub async fn recent_tags(pool: &DbPool) -> AppResult<Vec<TagInfo>> {
    let tags = TagEntity::find()
        .order_by_desc(TagColumn::CreatedAt)
        .limit(10)
        .all(pool)
        .await?;

    Ok(ApiResponse::ok(tags.into_iter().map(Into::into).collect()))
}

pub async fn find_or_create_tag(pool: &DbPool, name: String) -> Result<i64, sea_orm::DbErr> {
    if let Some(tag) = TagEntity::find()
        .filter(TagColumn::Name.eq(name.as_str()))
        .one(pool)
        .await?
    {
        return Ok(tag.id);
    }

    let tag = TagModel::builder().set_name(name).insert(pool).await?;
    Ok(tag.id)
}

pub async fn create_tag(pool: &DbPool, create: CreateTag) -> AppResult<i64> {
    if let Some(tag) = TagEntity::find()
        .filter(TagColumn::Name.eq(create.name.as_str()))
        .one(pool)
        .await?
    {
        return Ok(ApiResponse::ok(tag.id));
    }

    let tag = TagModel::builder()
        .set_name(create.name)
        .insert(pool)
        .await?;
    Ok(ApiResponse::ok(tag.id))
}

pub async fn search_tags(pool: &DbPool, query: SearchTagsQuery) -> AppResult<Vec<TagInfo>> {
    let tags = TagEntity::find()
        .filter(TagColumn::Name.contains(query.keyword))
        .order_by_desc(TagColumn::CreatedAt)
        .limit(10)
        .all(pool)
        .await?;

    Ok(ApiResponse::ok(tags.into_iter().map(Into::into).collect()))
}

pub async fn page_notes_by_tag(
    pool: &DbPool,
    tag_id: i64,
    query: PageNotesByTagQuery,
) -> AppResult<ApiPageData<NoteInfo>> {
    let page_num = query.page_num.max(1);
    let page_size = query.page_size.max(1);
    let note_ids = NoteTagEntity::find()
        .filter(NoteTagColumn::TagId.eq(tag_id))
        .all(pool)
        .await?
        .into_iter()
        .map(|item| item.note_id)
        .collect::<Vec<_>>();

    if note_ids.is_empty() {
        return Ok(ApiResponse::ok(ApiPageData {
            list: Vec::new(),
            page_num,
            page_size,
            total: 0,
        }));
    }

    let paginator = NoteEntity::find()
        .filter(NoteColumn::Id.is_in(note_ids))
        .order_by_desc(NoteColumn::UpdatedAt)
        .paginate(pool, page_size);
    let total = paginator.num_items().await?;
    let data = paginator.fetch_page(page_num - 1).await?;

    Ok(ApiResponse::ok(ApiPageData {
        list: notes_with_tags(pool, data).await?,
        page_num,
        page_size,
        total,
    }))
}

pub(crate) async fn delete_tag(pool: &DbPool, name: String) -> AppResult<Empty> {
    TagModel::builder().set_name(name).delete(pool).await?;
    Ok(ApiResponse::empty_ok())
}
