use crate::routes::users::router as users_router;
use axum::Router;

pub fn api_router() -> Router {
    Router::new().merge(users_router())
}
