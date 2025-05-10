use axum::http::StatusCode;

use crate::{
    domain::community::{Community, CommunityRepository},
    shared::api_error::ApiErrorResponse,
};
use std::sync::Arc;

pub struct DeleteCommunityUseCase<R: CommunityRepository> {
    community_repository: Arc<R>,
}

impl<R: CommunityRepository> DeleteCommunityUseCase<R> {
    pub fn new(community_repository: Arc<R>) -> Self {
        Self {
            community_repository,
        }
    }

    pub async fn execute(&self, community_id: i32) -> Result<(), (StatusCode, ApiErrorResponse)> {
        let community: Option<Community> = self
            .community_repository
            .get_by_id(community_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal server error".to_string()),
                )
            })?;

        if community.is_none() {
            return Err((
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new("Community not found".to_string()),
            ));
        }

        let mut community = community.unwrap();
        community.disable();

        self.community_repository
            .save(&community)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal server error".to_string()),
                )
            })?;
        Ok(())
    }
}
