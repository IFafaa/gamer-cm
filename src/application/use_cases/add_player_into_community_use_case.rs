use axum::http::StatusCode;

use crate::{
    domain::player::{Player, PlayerRepository},
    presentation::dtos::add_player_into_community_dto::AddPlayerIntoCommunityDto,
};
use std::sync::Arc;

pub struct AddPlayerIntoCommunityUseCase<R: PlayerRepository> {
    player_repository: Arc<R>,
}

impl<R: PlayerRepository> AddPlayerIntoCommunityUseCase<R> {
    pub fn new(player_repository: Arc<R>) -> Self {
        Self { player_repository }
    }

    pub async fn execute(
        &self,
        dto: AddPlayerIntoCommunityDto,
    ) -> Result<(), (StatusCode, String)> {
        if dto.nickname.is_empty() || dto.community_id <= 0 {
            return Err((StatusCode::BAD_REQUEST, "Invalid input".to_string()));
        }

        let already_exists = self
            .player_repository
            .exists(dto.nickname.clone(), dto.community_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            })?;
        if already_exists {
            return Err((StatusCode::CONFLICT, "Player already exists".to_string()));
        }

        let player = Player::new(dto.nickname, dto.community_id);
        self.player_repository.insert(&player).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
        })?;
        Ok(())
    }
}
