use axum::http::StatusCode;

use crate::{
    domain::community::CommunityRepository, // Certifique-se de que este módulo exista
    domain::player::{Player, PlayerRepository},
    presentation::dtos::add_player_into_community_dto::AddPlayerIntoCommunityDto,
    shared::api_error::ApiErrorResponse,
};
use std::sync::Arc;

pub struct AddPlayerIntoCommunityUseCase<PR: PlayerRepository, CR: CommunityRepository> {
    player_repository: Arc<PR>,
    community_repository: Arc<CR>,
}

impl<PR: PlayerRepository, CR: CommunityRepository> AddPlayerIntoCommunityUseCase<PR, CR> {
    pub fn new(player_repository: Arc<PR>, community_repository: Arc<CR>) -> Self {
        Self {
            player_repository,
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        dto: AddPlayerIntoCommunityDto,
    ) -> Result<(), (StatusCode, ApiErrorResponse)> {
        let community = self
            .community_repository
            .get_by_id(dto.community_id)
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

        let already_exists = self
            .player_repository
            .exists(dto.nickname.clone(), dto.community_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal server error".to_string()),
                )
            })?;
        if already_exists {
            return Err((
                StatusCode::CONFLICT,
                ApiErrorResponse::new("Player already exists in the community".to_string()),
            ));
        }

        let player = Player::new(dto.nickname, dto.community_id);
        self.player_repository.insert(&player).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Internal server error".to_string()),
            )
        })?;
        Ok(())
    }
}
