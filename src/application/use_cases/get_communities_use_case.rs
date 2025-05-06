use axum::http::StatusCode;

use crate::domain::community::{Community, CommunityRepository};
use std::sync::Arc;

pub struct GetCommunitiesUseCase<R: CommunityRepository> {
    repository: Arc<R>,
}

impl<R: CommunityRepository> GetCommunitiesUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<Community>, StatusCode> {
        let communities = self
            .repository
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
