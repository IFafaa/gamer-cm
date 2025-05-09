use axum::http::StatusCode;

use crate::domain::player::{Player, PlayerRepository};
use std::sync::Arc;

pub struct DeletePlayerOfCommunityUseCase<R: PlayerRepository> {
    player_repository: Arc<R>,
}

impl<R: PlayerRepository> DeletePlayerOfCommunityUseCase<R> {
    pub fn new(player_repository: Arc<R>) -> Self {
        Self { player_repository }
    }

    pub async fn execute(&self, player_id: i32) -> Result<(), (StatusCode, String)> {
        let player: Option<Player> =
            self.player_repository
                .get_by_id(player_id)
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )
                })?;

        if player.is_none() {
            return Err((StatusCode::NOT_FOUND, "Player not found".to_string()));
        }

        let mut player = player.unwrap();
        player.disable();

        self.player_repository.save(&player).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
        })?;
        Ok(())
    }
}
