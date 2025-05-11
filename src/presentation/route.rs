use axum::Router;

use crate::shared::state::AppState;

use super::routes::community_routes;
use super::routes::team_routes;

pub fn create_routes(app_state: AppState) -> Router {
    Router::new()
        .nest("/communities", community_routes::community_routes())
        .nest("/teams", team_routes::team_routes())
        .with_state(app_state)
}
