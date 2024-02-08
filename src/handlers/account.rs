use crate::prelude::*;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::Value;
use sqlx::PgPool;

pub struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub owner: String,
    pub currency: String,
}

pub async fn create_account(
    DatabaseConnection(mut conn): DatabaseConnection,
    arg: Json<CreateAccountRequest>,
) -> impl IntoResponse {
    todo!()
}

pub async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> std::result::Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
