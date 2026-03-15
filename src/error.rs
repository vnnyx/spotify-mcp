use rmcp::model::{ErrorCode, ErrorData};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Spotify API error: {0}")]
    SpotifyApi(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Token error: {0}")]
    Token(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

impl From<AppError> for ErrorData {
    fn from(err: AppError) -> Self {
        ErrorData {
            code: match &err {
                AppError::Auth(_) | AppError::Token(_) => ErrorCode(-32001),
                AppError::SpotifyApi(_) => ErrorCode(-32002),
                AppError::Http(_) => ErrorCode(-32003),
                _ => ErrorCode(-32000),
            },
            message: err.to_string().into(),
            data: None,
        }
    }
}
