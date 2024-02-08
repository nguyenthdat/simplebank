use crate::{
    db::account_sql::{create_account, CreateAccountParams},
    models::Account,
    prelude::*,
};
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

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub owner: String,
    pub currency: String,
}

pub async fn create_account_handler(
    State(pool): State<PgPool>,
    arg: Json<CreateAccountRequest>,
) -> ServerResult<Json<Account>> {
    if arg.owner.is_empty() {
        return Err(ServerError::CreateAccountFail);
    }
    if arg.currency.is_empty() {}

    let params = CreateAccountParams {
        owner: arg.owner.clone(),
        balance: 0,
        currency: arg.currency.clone(),
    };

    let account = create_account(&pool, params).await.unwrap();

    Ok(Json(account))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
