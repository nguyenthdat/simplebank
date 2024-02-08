use crate::handlers::account::{create_account, using_connection_pool_extractor};
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route(
            "/accounts",
            get(using_connection_pool_extractor).post(create_account),
        )
        .with_state(pool)
}
