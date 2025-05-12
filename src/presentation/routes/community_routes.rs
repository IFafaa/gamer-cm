use std::sync::Arc;

use crate::{
    application::{
        interfaces::{
            result_get_communities_interface::IResultGetCommunities,
            result_get_community_by_id_interface::IResultGetCommunityById,
        },
        use_cases::{
            create_community_use_case::CreateCommunityUseCase,
            delete_community_use_case::DeleteCommunityUseCase,
            get_communities_use_case::GetCommunitiesUseCase,
            get_community_by_id_use_case::GetCommunityByIdUseCase,
        },
    },
    infra::db::community_repository::PgCommunityRepository,
    presentation::dtos::create_community_dto::CreateCommunityDto,
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
        .route("/{id}", get(get_community_by_id))
        .route("/{id}", delete(delete_community))
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

async fn get_community_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<IResultGetCommunityById>>, (StatusCode, Json<ApiErrorResponse>)> {
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = GetCommunityByIdUseCase::new(Arc::new(community_repository));

    use_case
        .execute(id)
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
