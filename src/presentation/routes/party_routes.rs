use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, patch, post},
};

use crate::{
    application::{
        interfaces::result_get_party_interface::IResultGetParty,
        use_cases::{
            create_party_use_case::CreatePartyUseCase, end_party_use_case::EndPartyUseCase,
            get_parties_use_case::GetPartiesUseCase,
        },
    },
    domain::team,
    infra::db::{
        community_repository::PgCommunityRepository, party_repository::PgPartyRepository,
        team_repository::PgTeamRepository,
    },
    presentation::dtos::{create_party_dto::CreatePartyDto, end_party_dto::EndPartyDto},
    shared::{
        api_error::ApiErrorResponse, api_response::ApiResponse, state::AppState,
        validate_dto::validate_dto,
    },
};

pub fn party_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_party))
        .route("/", get(get_parties))
        .route("/end", patch(end_party))
}

async fn create_party(
    State(state): State<AppState>,
    Json(dto): Json<CreatePartyDto>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    dto.validate()?;

    let team_repository = PgTeamRepository::new(state.db.clone());
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let party_repository = PgPartyRepository::new(state.db.clone());
    let use_case = CreatePartyUseCase::new(
        Arc::new(team_repository),
        Arc::new(community_repository),
        Arc::new(party_repository),
    );

    use_case
        .execute(dto)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}

async fn get_parties(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<IResultGetParty>>>, (StatusCode, Json<ApiErrorResponse>)> {
    let party_repository = PgPartyRepository::new(state.db.clone());
    let use_case = GetPartiesUseCase::new(Arc::new(party_repository));

    use_case
        .execute()
        .await
        .map(Json)
        .map_err(|(status, error)| (status, Json(error)))
}

async fn end_party(
    State(state): State<AppState>,
    Json(dto): Json<EndPartyDto>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    dto.validate()?;

    let team_repository = PgTeamRepository::new(state.db.clone());
    let party_repository = PgPartyRepository::new(state.db.clone());
    let use_case = EndPartyUseCase::new(Arc::new(party_repository), Arc::new(team_repository));

    use_case
        .execute(dto)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}
