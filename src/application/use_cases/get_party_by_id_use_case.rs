use std::sync::Arc;

use axum::http::StatusCode;

use crate::{
    application::interfaces::result_get_party_interface::IResultGetParty,
    domain::party::PartyRepository,
    shared::{api_error::ApiErrorResponse, api_response::ApiResponse},
};

pub struct GetPartyByIdUseCase<PR: PartyRepository> {
    party_repository: Arc<PR>,
}

impl<PR: PartyRepository> GetPartyByIdUseCase<PR> {
    pub fn new(party_repository: Arc<PR>) -> Self {
        Self { party_repository }
    }

    pub async fn execute(
        &self,
        party_id: i32,
    ) -> Result<ApiResponse<IResultGetParty>, (StatusCode, ApiErrorResponse)> {
        let party = self
            .party_repository
            .get_by_id(party_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Failed to fetch parties".to_string()),
                )
            })?
            .ok_or((
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new("Party not found".to_string()),
            ))?;

        let result = IResultGetParty::new(party);

        Ok(ApiResponse::success(result))
    }
}
