use axum::{ http::StatusCode, response::{ IntoResponse, Response }, Json };
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")] Config(#[from] config::ConfigError),

    #[error("IO error: {0}")] Io(#[from] std::io::Error),

    #[error("Invalid regex pattern: {0}")] Regex(#[from] regex::Error),

    #[error("Network request error: {0}")] Request(#[from] reqwest::Error),

    #[error("Backend unavailable")]
    BackendUnavailable,

    #[error("WAF Rule Parse Error: {0}")] WafParse(String),
}

// Implementazione per Axum: converte gli errori in risposte HTTP JSON
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::BackendUnavailable =>
                (StatusCode::BAD_GATEWAY, "Backend services unavailable"),
            AppError::Request(_) => (StatusCode::BAD_GATEWAY, "Upstream error"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(
            json!({
            "error": error_message,
            "details": self.to_string(),
        })
        );

        (status, body).into_response()
    }
}
