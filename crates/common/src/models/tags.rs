use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub created_at: TimeDateTime,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for TagInfo {
    fn from(value: Model) -> Self {
        TagInfo {
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct TagInfo {
    // 标识
    pub id: i64,
    // 标签名称
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTag {
    // 标签名称
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchTagsQuery {
    // 搜索关键字
    pub keyword: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageNotesByTagQuery {
    #[serde(default = "default_page_num")]
    pub page_num: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page_num() -> u64 {
    1
}

fn default_page_size() -> u64 {
    10
}
