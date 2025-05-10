use axum::http::StatusCode;

use crate::{
    application::interfaces::result_get_community_by_id_interface::IResultGetCommunityById,
    domain::community::CommunityRepository,
    shared::{api_error::ApiErrorResponse, api_response::ApiResponse},
};
use std::sync::Arc;

pub struct GetCommunityByIdUseCase<R: CommunityRepository> {
    community_repository: Arc<R>,
}

impl<R: CommunityRepository> GetCommunityByIdUseCase<R> {
    pub fn new(community_repository: Arc<R>) -> Self {
        Self {
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        _community_id: i32,
    ) -> Result<ApiResponse<IResultGetCommunityById>, (StatusCode, ApiErrorResponse)> {
        let community = self
            .community_repository
            .get_by_id(_community_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal Server Error".to_string()),
                )
            })?;

        if community.is_none() {
            return Err((
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new("Community not found".to_string()),
            ));
        }

        let community = community.unwrap();

        Ok(IResultGetCommunityById::new(community))
    }
}
