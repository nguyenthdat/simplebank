use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, PartialEq, Clone)]
pub struct Account {
    pub id: i64,
    pub owner: String,
    pub balance: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}
