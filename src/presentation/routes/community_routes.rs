use std::sync::Arc;

use crate::{
    application::use_cases::{
        add_player_into_community_use_case::AddPlayerIntoCommunityUseCase,
        create_community_use_case::CreateCommunityUseCase,
        get_communities_use_case::GetCommunitiesUseCase,
    },
    domain::community::Community,
    infra::db::{
        community_repository::PgCommunityRepository, player_repository::PgPlayerRepository,
    },
    presentation::dtos::{
        add_player_into_community_dto::AddPlayerIntoCommunityDto,
        create_community_dto::CreateCommunityDto,
    },
    shared::state::AppState,
};
use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    routing::{get, post},
};

pub fn community_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_community))
        .route("/", get(get_communities))
        .route("/add-player", post(add_player_into_community))
}

async fn create_community(
    State(state): State<AppState>,
    Json(dto): Json<CreateCommunityDto>,
) -> Result<(), (StatusCode, String)> {
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = CreateCommunityUseCase::new(Arc::new(community_repository));

    use_case.execute(dto).await
}

async fn get_communities(
    State(state): State<AppState>,
) -> Result<Json<Vec<Community>>, (StatusCode, String)> {
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = GetCommunitiesUseCase::new(Arc::new(community_repository));

    use_case.execute().await.map(Json)
}

async fn add_player_into_community(
    State(state): State<AppState>,
    Json(dto): Json<AddPlayerIntoCommunityDto>,
) -> Result<(), (StatusCode, String)> {
    let player_repository = PgPlayerRepository::new(state.db.clone());
    let use_case = AddPlayerIntoCommunityUseCase::new(Arc::new(player_repository));

    use_case.execute(dto).await
}
