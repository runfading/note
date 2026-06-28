use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "notebooks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub created_at: TimeDateTime,
    pub updated_at: TimeDateTime,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for NotebookInfo {
    fn from(value: Model) -> Self {
        NotebookInfo {
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NotebookInfo {
    // 标识
    pub id: i64,
    // 笔记本名称
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNotebook {
    // 笔记本名称
    pub name: String,
}
