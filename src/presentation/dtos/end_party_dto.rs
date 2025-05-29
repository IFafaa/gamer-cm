use axum::{Json, http::StatusCode};
use serde::Deserialize;
use validator::Validate;

use crate::shared::api_error::ApiErrorResponse;

#[derive(Deserialize, Validate)]
pub struct EndPartyDto {
    pub party_id: i32,

    pub team_winner_id: Option<i32>,
}

impl EndPartyDto {
    pub fn validate(&self) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
        let mut errors: Vec<String> = Vec::new();

        if self.party_id <= 0 {
            errors.push("Party ID must be greater than 0".into());
        }

        if self.team_winner_id.is_some() && self.team_winner_id.unwrap() <= 0 {
            errors.push("Team winner ID must be greater than 0".into());
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
