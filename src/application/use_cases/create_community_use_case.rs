use axum::http::StatusCode;

use crate::{
    domain::community::{Community, CommunityRepository},
    presentation::dtos::create_community_dto::CreateCommunityDto,
};
use std::sync::Arc;

pub struct CreateCommunityUseCase<R: CommunityRepository> {
    repository: Arc<R>,
}

impl<R: CommunityRepository> CreateCommunityUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, dto: CreateCommunityDto) -> Result<(), StatusCode> {
        if dto.name.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }

        let already_exists = self
            .repository
            .exists(dto.name.clone())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if already_exists {
            return Err(StatusCode::CONFLICT);
        }

        let community = Community::new(dto.name);
        self.repository
            .insert(&community)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(())
    }
}
