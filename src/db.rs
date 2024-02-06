use crate::prelude::*;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub mod account_sql;

async fn create_connection_pool(max_conn: Option<u32>) -> Result<PgPool> {
    let max_conn = max_conn.unwrap_or(5);
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(&database_url)
        .await?;
    Ok(pool)
}
