use axum::http::StatusCode;
use std::sync::Arc;

use crate::{
    domain::{player::PlayerRepository, team::TeamRepository},
    presentation::dtos::add_players_into_team_dto::AddPlayersIntoTeamDto,
    shared::api_error::ApiErrorResponse,
};

pub struct AddPlayersIntoTeamUseCase<PR: PlayerRepository, TR: TeamRepository> {
    player_repository: Arc<PR>,
    team_repository: Arc<TR>,
}

impl<PR: PlayerRepository, TR: TeamRepository> AddPlayersIntoTeamUseCase<PR, TR> {
    pub fn new(player_repository: Arc<PR>, team_repository: Arc<TR>) -> Self {
        Self {
            player_repository,
            team_repository,
        }
    }

    pub async fn execute(
        &self,
        dto: AddPlayersIntoTeamDto,
    ) -> Result<(), (StatusCode, ApiErrorResponse)> {
        let mut team = self
            .team_repository
            .get_by_id(dto.team_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal server error".to_string()),
                )
            })?
            .ok_or((
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new("Team not found".to_string()),
            ))?;

        let players_ids = dto.players_ids.clone();
        let players = self
            .player_repository
            .get_by_ids(players_ids)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal server error".to_string()),
                )
            })?;

        if players.is_empty() {
            return Err((
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new("No players found".to_string()),
            ));
        }

        for player in &players {
            team.add_player(player.clone());
        }

        self.team_repository.save(&team).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Internal server error".to_string()),
            )
        })?;

        let players_not_found: Vec<i32> = dto
            .players_ids
            .iter()
            .filter(|id| !players.iter().any(|p| p.id == **id))
            .cloned()
            .collect();

        if !players_not_found.is_empty() {
            return Err((
                StatusCode::MULTI_STATUS,
                ApiErrorResponse::new(format!(
                    "Some players were not found: {}",
                    players_not_found
                        .iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )),
            ));
        }

        Ok(())
    }
}
