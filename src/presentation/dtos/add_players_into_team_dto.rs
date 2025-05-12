use axum::{Json, http::StatusCode};
use serde::Deserialize;

use crate::shared::api_error::ApiErrorResponse;

#[derive(Debug, Deserialize)]
pub struct AddPlayersIntoTeamDto {
    pub team_id: i32,
    pub players_ids: Vec<i32>,
}

impl AddPlayersIntoTeamDto {
    pub fn validate(&self) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
        let mut errors: Vec<String> = Vec::new();

        if self.team_id <= 0 {
            errors.push("Team ID must be greater than 0".into());
        }

        if self.players_ids.is_empty() {
            errors.push("Player IDs cannot be empty".into());
        }

        if self.players_ids.iter().any(|id| *id <= 0) {
            errors.push("All player IDs must be greater than 0".into());
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
