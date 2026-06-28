use crate::models::tags::TagInfo;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "notes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub notebook_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: TimeDateTime,
    pub updated_at: TimeDateTime,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for NoteInfo {
    fn from(value: Model) -> Self {
        NoteInfo {
            id: value.id,
            notebook_id: value.notebook_id,
            title: value.title,
            content: Some(value.content),
            tags: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NoteInfo {
    // 标识
    pub id: i64,
    // 笔记本标识
    pub notebook_id: i64,
    // 标题
    pub title: String,
    // 内容
    pub content: Option<String>,
    // 标签列表
    pub tags: Vec<TagInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNote {
    // 笔记本标识
    pub notebook_id: i64,
    // 标题
    pub title: String,
    // 内容
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageNotesQuery {
    #[serde(default = "default_page_num")]
    pub page_num: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
    // 笔记本标识，不传则查询全部
    pub notebook_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchNotesQuery {
    #[serde(default = "default_page_num")]
    pub page_num: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
    // 搜索关键字
    pub keyword: String,
}

fn default_page_num() -> u64 {
    1
}

fn default_page_size() -> u64 {
    10
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateNote {
    /// 笔记id
    pub note_id: i64,
    /// 笔记本标识
    pub notebook_id: Option<i64>,
    /// 标题
    pub title: Option<String>,
    /// 内容
    pub content: Option<String>,
}
