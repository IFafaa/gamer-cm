use axum::http::StatusCode;

use crate::{
    application::interfaces::result_get_communities_interface::IResultGetCommunities,
    domain::community::CommunityRepository,
    shared::{api_error::ApiErrorResponse, api_response::ApiResponse},
};
use std::sync::Arc;

pub struct GetCommunitiesUseCase<R: CommunityRepository> {
    community_repository: Arc<R>,
}

impl<R: CommunityRepository> GetCommunitiesUseCase<R> {
    pub fn new(community_repository: Arc<R>) -> Self {
        Self {
            community_repository,
        }
    }

    pub async fn execute(
        &self,
    ) -> Result<ApiResponse<Vec<IResultGetCommunities>>, (StatusCode, ApiErrorResponse)> {
        let communities = self.community_repository.get_all().await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Internal Server Error".to_string()),
            )
        })?;

        if communities.is_empty() {
            return Err((
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new("No communities found".to_string()),
            ));
        }

        Ok(IResultGetCommunities::new(communities))
    }
}
