use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router {
    Router::new().route("/api/users/login", get(login_user))
}

async fn login_user() -> &'static str {
    return "Hello, world!";
}
