use crate::routes::users::router as users_router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

pub fn api_router() -> Router {
    Router::new()
        .route("/", get(probe))
        .merge(users_router())
        .fallback(not_found)
}

async fn probe() -> &'static str {
    "OK"
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}
