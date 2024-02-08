use crate::handlers::account::create_account_handler;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/accounts", post(create_account_handler))
        .with_state(pool)
}
