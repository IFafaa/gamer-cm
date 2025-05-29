use axum::http::StatusCode;

use crate::{domain::party::PartyRepository, shared::api_error::ApiErrorResponse};
use std::sync::Arc;

pub struct DeletePartyUseCase<PR: PartyRepository> {
    party_repository: Arc<PR>,
}

impl<PR: PartyRepository> DeletePartyUseCase<PR> {
    pub fn new(party_repository: Arc<PR>) -> Self {
        Self { party_repository }
    }

    pub async fn execute(&self, party_id: i32) -> Result<(), (StatusCode, ApiErrorResponse)> {
        let mut party = self
            .party_repository
            .get_by_id(party_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Failed to fetch party".to_string()),
                )
            })?
            .ok_or((
                StatusCode::BAD_REQUEST,
                ApiErrorResponse::new("Party not found".to_string()),
            ))?;

        if !party.is_enabled() {
            return Err((
                StatusCode::BAD_REQUEST,
                ApiErrorResponse::new("Party is already disabled".to_string()),
            ));
        }

        party.disable();

        self.party_repository.save(&party).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Failed to save party".to_string()),
            )
        })?;

        Ok(())
    }
}
