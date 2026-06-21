use crate::common::{ApiResponse, DbError};
use tauri::ipc::InvokeError;
use tracing::error;

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct BizError {
    pub code: u32,
    pub message: &'static str,
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("database error")]
    Db(#[from] DbError),

    #[error("")]
    BizError(#[from] BizError),

    #[error("{0}")]
    NotFound(String),
}

impl Into<InvokeError> for AppError {
    fn into(self) -> InvokeError {
        // 先记录完整错误
        if !matches!(self, AppError::NotFound(_) | AppError::BizError(_)) {
            error!(error = ?self, "internal server error");
        }

        let (code, message) = match self {
            AppError::NotFound(msg) => (0, msg),
            AppError::BizError(BizError { code, message }) => (code, message.to_string()),
            _ => (9999, "Internal Server Error".to_string()),
        };

        let response = ApiResponse::err(code, message);
        match serde_json::to_value(response) {
            Ok(response) => InvokeError { 0: response },
            Err(err) => {
                error!(
                    "{}",
                    BizError {
                        code,
                        message: "Internal Server Error"
                    }
                );
                InvokeError::from_error(err)
            }
        }
    }
}
