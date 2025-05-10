use axum::{Json, http::StatusCode};
use validator::Validate;

use super::api_error::ApiErrorResponse;

pub fn validate_dto<T: Validate>(dto: &T) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    if let Err(errors) = dto.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiErrorResponse::new(errors.to_string())),
        ));
    }
    Ok(())
}
