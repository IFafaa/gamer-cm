use std::sync::Arc;

use crate::{
    application::use_cases::create_community_use_case::CreateCommunityUseCase,
    infra::db::community_repository::PgCommunityRepository,
    presentation::dtos::create_community_dto::CreateCommunityDto, shared::state::AppState,
};
use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    routing::post,
};

pub fn community_routes() -> Router<AppState> {
    Router::new().route("/", post(create_community))
}

async fn create_community(
    State(state): State<AppState>,
    Json(dto): Json<CreateCommunityDto>,
) -> Result<(), StatusCode> {
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = CreateCommunityUseCase::new(Arc::new(community_repository));

    use_case.execute(dto).await
}
