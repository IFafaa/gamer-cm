use serde::Serialize;

#[derive(Serialize)]
pub struct ApiErrorResponse {
    message: String,
    timestamp: String,
}

impl ApiErrorResponse {
    pub fn new(message: String) -> Self {
        Self {
            message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
