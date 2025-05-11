use std::sync::Arc;

use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{delete, get, post},
};

use crate::{
    application::use_cases::create_team_use_case::CreateTeamUseCase,
    infra::db::{community_repository::PgCommunityRepository, team_repository::PgTeamRepository},
    presentation::dtos::create_team_dto::CreateTeamDto,
    shared::{api_error::ApiErrorResponse, state::AppState, validate_dto::validate_dto},
};

pub fn team_routes() -> Router<AppState> {
    Router::new().route("/", post(create_team))
}

async fn create_team(
    State(state): State<AppState>,
    Json(dto): Json<CreateTeamDto>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    validate_dto(&dto)?;

    let team_repository = PgTeamRepository::new(state.db.clone());
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case =
        CreateTeamUseCase::new(Arc::new(team_repository), Arc::new(community_repository));

    use_case
        .execute(dto)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}
