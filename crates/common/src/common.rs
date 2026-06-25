use crate::error::AppError;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
}

pub type DbPool = DatabaseConnection;
pub type AppResult<T> = Result<ApiResponse<T>, AppError>;
pub type DbError = sea_orm::DbErr;

#[derive(Debug, Serialize)]
pub struct Empty {}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            message: "success".into(),
            data: Some(data),
        }
    }
}

impl ApiResponse<Empty> {
    pub fn err(code: u32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }

    pub fn empty_ok() -> Self {
        Self::ok(Empty {})
    }
}

#[derive(Debug, Serialize)]
pub struct ApiPageData<T> {
    // 集合
    pub list: Vec<T>,
    // 页码
    pub page_num: u64,
    // 容量
    pub page_size: u64,
    // 总计
    pub total: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiPageQuery {
    pub page_num: u64,
    pub page_size: u64,
}

impl Default for ApiPageQuery {
    fn default() -> Self {
        Self {
            page_num: 1,
            page_size: 10,
        }
    }
}
