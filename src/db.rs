use crate::prelude::*;
use futures::future::BoxFuture;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub mod account_sql;
pub mod entry_sql;
pub mod store;
pub mod transfer_sql;

async fn create_connection_pool(max_conn: Option<u32>) -> Result<PgPool> {
    let max_conn = max_conn.unwrap_or(5);
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(&database_url)
        .await?;
    Ok(pool)
}

async fn _tx_exec<F, T>(pool: &PgPool, f: F) -> Result<T>
where
    F: for<'c> FnOnce(&'c mut sqlx::Transaction<'_, sqlx::Postgres>) -> BoxFuture<'c, Result<T>>,
{
    let mut tx = pool.begin().await?;
    let res = f(&mut tx).await;
    match res {
        Ok(result) => Ok(result),
        Err(err) => {
            tx.rollback().await?;
            Err(err)
        }
    }
}

mod tests {
    use super::*;
    use crate::{
        db::{
            create_connection_pool,
            entry_sql::{create_entry, CreateEntryParams},
        },
        util::*,
    };

    #[tokio::test]
    async fn test_tx_exec() {
        dotenv::dotenv().ok();
        let pool = create_connection_pool(Some(10)).await.unwrap();
        let account = random_account(&pool).await.unwrap();
        let amount = random_money();

        let result = _tx_exec(&pool, |tx| {
            Box::pin(async move {
                let entry = create_entry(
                    tx,
                    CreateEntryParams {
                        account_id: account.id,
                        amount,
                    },
                )
                .await?;
                Ok(entry)
            })
        })
        .await
        .unwrap();

        assert_eq!(result.account_id, account.id);
        assert_eq!(result.amount, amount);
    }
}
