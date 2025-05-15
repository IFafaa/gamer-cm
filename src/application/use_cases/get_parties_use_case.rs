use std::sync::Arc;

use axum::http::StatusCode;

use crate::{
    application::interfaces::result_get_party_interface::IResultGetParty,
    domain::party::{IGetPartiesByParams, PartyRepository},
    shared::{api_error::ApiErrorResponse, api_response::ApiResponse},
};

pub struct GetPartiesUseCase<PR: PartyRepository> {
    party_repository: Arc<PR>,
}

impl<PR: PartyRepository> GetPartiesUseCase<PR> {
    pub fn new(party_repository: Arc<PR>) -> Self {
        Self { party_repository }
    }

    pub async fn execute(
        &self,
    ) -> Result<ApiResponse<Vec<IResultGetParty>>, (StatusCode, ApiErrorResponse)> {
        let params = IGetPartiesByParams {
            community_id: None,
            game_name: None,
            created_at: None,
            updated_at: None,
            teams_ids: None,
            team_winner_ids: None,
        };

        let parties = self
            .party_repository
            .get_by_params(params)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Failed to fetch parties".to_string()),
                )
            })?;

        let result = parties
            .into_iter()
            .map(|party| IResultGetParty::new(party))
            .collect();

        Ok(ApiResponse::success(result))
    }
}
