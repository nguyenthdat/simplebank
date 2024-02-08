use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, PartialEq, Clone, Serialize)]
pub struct Account {
    pub id: i64,
    pub owner: String,
    pub balance: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, PartialEq, Clone)]
pub struct Entry {
    pub id: i64,
    pub account_id: i64,
    pub amount: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, PartialEq, Clone)]
pub struct Transfer {
    pub id: i64,
    pub from_account_id: i64,
    pub to_account_id: i64,
    pub amount: i64,
    pub created_at: DateTime<Utc>,
}
