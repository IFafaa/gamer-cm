use axum::http::StatusCode;

use crate::{
    domain::player::{Player, PlayerRepository},
    shared::api_error::ApiErrorResponse,
};
use std::sync::Arc;

pub struct DeletePlayerOfCommunityUseCase<R: PlayerRepository> {
    player_repository: Arc<R>,
}

impl<R: PlayerRepository> DeletePlayerOfCommunityUseCase<R> {
    pub fn new(player_repository: Arc<R>) -> Self {
        Self { player_repository }
    }

    pub async fn execute(&self, player_id: i32) -> Result<(), (StatusCode, ApiErrorResponse)> {
        let player: Option<Player> =
            self.player_repository
                .get_by_id(player_id)
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ApiErrorResponse::new("Internal server error".to_string()),
                    )
                })?;

        if player.is_none() {
            return Err((
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new("Player not found".to_string()),
            ));
        }

        let mut player = player.unwrap();

        if !player.is_enabled() {
            return Err((
                StatusCode::BAD_REQUEST,
                ApiErrorResponse::new("Player is already disabled".to_string()),
            ));
        }

        player.disable();

        self.player_repository.save(&player).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Internal server error".to_string()),
            )
        })?;
        Ok(())
    }
}
