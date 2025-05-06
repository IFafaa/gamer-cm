use axum::http::StatusCode;

use crate::domain::community::{Community, CommunityRepository};
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

    pub async fn execute(&self) -> Result<Vec<Community>, StatusCode> {
        let communities = self
            .community_repository
            .get_all()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if communities.is_empty() {
            Err(StatusCode::NOT_FOUND)
        } else {
            Ok(communities)
        }
    }
}
