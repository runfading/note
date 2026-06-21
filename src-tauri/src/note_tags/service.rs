use super::models::{
    AddNoteTag, Column as NoteTagColumn, Entity as NoteTagEntity, TagsUsedTimesQuery,
    TagsUsedTimesResponse,
};
use crate::common::{ApiResponse, AppResult, DbError, DbPool, Empty};
use crate::note_tags::models::ActiveModel as NoteTagModel;
use crate::notes::models::{Model as Note, NoteInfo};
use crate::tags::models::{Column as TagColumn, Entity as TagEntity, TagInfo};
use crate::tags::service::find_or_create_tag;
use sea_orm::prelude::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ExprTrait, QueryFilter, QuerySelect, Set,
};
use std::collections::HashMap;

pub async fn add_note_tag(pool: &DbPool, note_id: i64, tag: AddNoteTag) -> AppResult<Empty> {
    let tag_id = find_or_create_tag(pool, tag.name).await?;
    let exists = NoteTagEntity::find()
        .filter(NoteTagColumn::NoteId.eq(note_id))
        .filter(NoteTagColumn::TagId.eq(tag_id))
        .one(pool)
        .await?
        .is_some();

    if !exists {
        NoteTagModel {
            note_id: Set(note_id),
            tag_id: Set(tag_id),
            ..Default::default()
        }
        .insert(pool)
        .await?;
    }

    Ok(ApiResponse::empty_ok())
}

pub async fn notes_with_tags(pool: &DbPool, notes: Vec<Note>) -> Result<Vec<NoteInfo>, DbError> {
    let note_ids = notes.iter().map(|note| note.id).collect::<Vec<_>>();
    if note_ids.is_empty() {
        return Ok(Vec::new());
    }

    // 查所有笔记关联信息
    let relations = NoteTagEntity::find()
        .filter(NoteTagColumn::NoteId.is_in(note_ids))
        .all(pool)
        .await?;
    if relations.is_empty() {
        return Ok(notes.into_iter().map(Into::into).collect());
    }

    // 查所有的tag信息
    let tag_ids = relations
        .iter()
        .map(|relation| relation.tag_id)
        .collect::<Vec<_>>();
    let tags = TagEntity::find()
        .filter(TagColumn::Id.is_in(tag_ids))
        .all(pool)
        .await?;

    // tag_id = tag_info
    let tag_map: HashMap<i64, TagInfo> = tags.into_iter().map(|tag| (tag.id, tag.into())).collect();
    let mut tags_by_note = HashMap::<i64, Vec<TagInfo>>::new();
    for relation in relations {
        if let Some(tag) = tag_map.get(&relation.tag_id) {
            tags_by_note
                .entry(relation.note_id)
                .or_default()
                .push(tag.clone());
        }
    }

    // 数据组装
    Ok(notes
        .into_iter()
        .map(|note| {
            let mut info: NoteInfo = note.into();
            info.tags = tags_by_note.remove(&info.id).unwrap_or_default();
            info
        })
        .collect())
}

pub async fn delete_note_tag(pool: &DbPool, note_id: i64, tag: String) -> AppResult<Empty> {
    let tag_id = find_or_create_tag(pool, tag).await?;
    let model = NoteTagModel::builder()
        .set_note_id(note_id)
        .set_tag_id(tag_id);

    model.delete(pool).await?;

    Ok(ApiResponse::empty_ok())
}

pub(crate) async fn query_tag_used_times(
    pool: &DbPool,
    query: TagsUsedTimesQuery,
) -> AppResult<Vec<TagsUsedTimesResponse>> {
    Ok(ApiResponse::ok(
        NoteTagEntity::find()
            .select_only()
            .column(NoteTagColumn::TagId)
            .column_as(Expr::col(NoteTagColumn::TagId).count(), "use_count")
            .filter(NoteTagColumn::TagId.is_in(query.ids))
            .group_by(NoteTagColumn::TagId)
            .into_model::<TagsUsedTimesResponse>()
            .all(pool)
            .await?,
    ))
}
