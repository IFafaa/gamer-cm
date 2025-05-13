use axum::{Json, http::StatusCode};
use serde::Deserialize;

use crate::shared::api_error::ApiErrorResponse;

#[derive(Debug, Deserialize)]
pub struct CreatePartyDto {
    pub game_name: String,
    pub teams_ids: Vec<i32>,
    pub community_id: i32,
}

impl CreatePartyDto {
    pub fn validate(&self) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
        let mut errors: Vec<String> = Vec::new();

        if self.community_id <= 0 {
            errors.push("Community ID must be greater than 0".into());
        }

        if self.game_name.trim().is_empty() {
            errors.push("Game name cannot be empty".into());
        }
        if self.teams_ids.len() < 2 {
            errors.push("There must be at least two teams".into());
        }

        if self.teams_ids.iter().any(|id| *id <= 0) {
            errors.push("All team IDs must be greater than 0".into());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err((
                StatusCode::BAD_REQUEST,
                Json(ApiErrorResponse::new(errors.join(", "))),
            ))
        }
    }
}
