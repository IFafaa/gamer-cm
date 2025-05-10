use std::sync::Arc;

use crate::{
    application::{
        interfaces::get_communities_interface::IResultGetCommunities,
        use_cases::{
            add_player_into_community_use_case::AddPlayerIntoCommunityUseCase,
            create_community_use_case::CreateCommunityUseCase,
            delete_community_use_case::DeleteCommunityUseCase,
            delete_player_of_community_use_case::DeletePlayerOfCommunityUseCase,
            get_communities_use_case::GetCommunitiesUseCase,
        },
    },
    infra::db::{
        community_repository::PgCommunityRepository, player_repository::PgPlayerRepository,
    },
    presentation::dtos::{
        add_player_into_community_dto::AddPlayerIntoCommunityDto,
        create_community_dto::CreateCommunityDto,
    },
    shared::{
        api_error::ApiErrorResponse, api_response::ApiResponse, state::AppState,
        validate_dto::validate_dto,
    },
};
use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{delete, get, post},
};

pub fn community_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_community))
        .route("/", get(get_communities))
        .route("/player", post(add_player_into_community))
        .route("/{id}", delete(delete_community))
        .route("/player/{id}", delete(delete_player_of_community))
}

async fn create_community(
    State(state): State<AppState>,
    Json(dto): Json<CreateCommunityDto>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    validate_dto(&dto)?;

    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = CreateCommunityUseCase::new(Arc::new(community_repository));

    use_case
        .execute(dto)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}

async fn get_communities(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<IResultGetCommunities>>>, (StatusCode, Json<ApiErrorResponse>)> {
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = GetCommunitiesUseCase::new(Arc::new(community_repository));

    use_case
        .execute()
        .await
        .map(|response| Json(response))
        .map_err(|(status, error)| (status, Json(error)))
}

async fn delete_community(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = DeleteCommunityUseCase::new(Arc::new(community_repository));

    use_case
        .execute(id)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}

async fn add_player_into_community(
    State(state): State<AppState>,
    Json(dto): Json<AddPlayerIntoCommunityDto>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    validate_dto(&dto)?;

    let player_repository = PgPlayerRepository::new(state.db.clone());
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = AddPlayerIntoCommunityUseCase::new(
        Arc::new(player_repository),
        Arc::new(community_repository),
    );

    use_case
        .execute(dto)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}

async fn delete_player_of_community(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    let player_repository = PgPlayerRepository::new(state.db.clone());
    let use_case = DeletePlayerOfCommunityUseCase::new(Arc::new(player_repository));

    use_case
        .execute(id)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}
