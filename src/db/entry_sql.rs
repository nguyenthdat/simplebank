use crate::model::Entry;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CreateEntryParams {
    pub account_id: i64,
    pub amount: i64,
}

pub async fn create_entry(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    arg: CreateEntryParams,
) -> SQLResult<Entry> {
    sqlx::query_as!(
        Entry,
        "INSERT INTO entries (account_id, amount) VALUES ($1, $2) RETURNING *;",
        arg.account_id,
        arg.amount
    )
    .fetch_one(&mut **transaction)
    .await
}

pub async fn get_entry(pool: &sqlx::PgPool, id: i64) -> Result<Entry> {
    let entry = sqlx::query_as!(Entry, "SELECT * FROM entries WHERE id = $1 LIMIT 1;", id)
        .fetch_one(pool)
        .await?;
    Ok(entry)
}

pub async fn list_entries(pool: &sqlx::PgPool, account_id: i64) -> Result<Vec<Entry>> {
    let entries = sqlx::query_as!(
        Entry,
        "SELECT * FROM entries WHERE account_id = $1 ORDER BY id;",
        account_id
    )
    .fetch_all(pool)
    .await?;
    Ok(entries)
}

mod tests {
    use super::*;
    use crate::model::Entry;
    use crate::{db::create_connection_pool, util::*};

    #[tokio::test]
    async fn test_create_entry() {
        dotenv::dotenv().ok();
        let pool = create_connection_pool(Some(10)).await.unwrap();
        let account = random_account(&pool).await.unwrap();
        let amount = random_money();

        let mut tx = pool.begin().await.unwrap();
        let entry = create_entry(
            &mut tx,
            CreateEntryParams {
                account_id: account.id,
                amount,
            },
        )
        .await
        .unwrap();

        assert_eq!(entry.account_id, account.id);
        assert_eq!(entry.amount, amount);
    }

    #[tokio::test]
    async fn test_get_entry() {
        dotenv::dotenv().ok();
        let pool = create_connection_pool(Some(10)).await.unwrap();
        let account = random_account(&pool).await.unwrap();

        let entry = random_entry(&pool, account.id).await.unwrap();

        let got = get_entry(&pool, entry.id).await.unwrap();
        assert_eq!(got, entry);
    }
}
