use axum::http;
use tower_http::cors::{Any, CorsLayer};

pub fn cors_config() -> Result<CorsLayer, Box<dyn std::error::Error>> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([http::Method::GET, http::Method::POST])
        .allow_headers([http::header::CONTENT_TYPE]);
    Ok(cors)
}
