use sea_orm::prelude::*;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "note_tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub note_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: i64,
    pub created_at: TimeDateTime,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddNoteTag {
    // 标签名称
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagsUsedTimesQuery {
    // id列表
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize, FromQueryResult, Serialize)]
pub struct TagsUsedTimesResponse {
    // tag id
    #[serde(rename = "id")]
    pub tag_id: i64,

    // 使用次数
    pub use_count: i64,
}
