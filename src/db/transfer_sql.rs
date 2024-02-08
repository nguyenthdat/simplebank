use crate::models::Transfer;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CreateTransferParams {
    pub from_account_id: i64,
    pub to_account_id: i64,
    pub amount: i64,
}

pub async fn create_transfer(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    arg: CreateTransferParams,
) -> SQLResult<Transfer> {
    sqlx::query_as!(
        Transfer,
        "INSERT INTO transfers (from_account_id, to_account_id, amount) VALUES ($1, $2, $3) RETURNING *;",
        arg.from_account_id,
        arg.to_account_id,
        arg.amount
    )
    .fetch_one(&mut **transaction)
    .await
}

pub async fn get_transfer(pool: &sqlx::PgPool, id: i64) -> Result<Transfer> {
    let transfer = sqlx::query_as!(
        Transfer,
        "SELECT * FROM transfers WHERE id = $1 LIMIT 1;",
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(transfer)
}

pub async fn list_transfers(pool: &sqlx::PgPool, account_id: i64) -> Result<Vec<Transfer>> {
    let transfers = sqlx::query_as!(
        Transfer,
        "SELECT * FROM transfers WHERE from_account_id = $1 OR to_account_id = $1 ORDER BY id;",
        account_id
    )
    .fetch_all(pool)
    .await?;
    Ok(transfers)
}

mod tests {
    use super::*;
    use crate::{db::create_connection_pool, utils::*};

    #[tokio::test]
    async fn test_create_transfer() {
        dotenv::dotenv().ok();
        let db = create_connection_pool(Some(10))
            .await
            .expect("Failed to create connection pool");

        let from_account = random_account(&db).await.unwrap();
        let to_account = random_account(&db).await.unwrap();

        let money = random_int(10, from_account.balance);
        let mut tx = db.begin().await.unwrap();

        let transfer = create_transfer(
            &mut tx,
            CreateTransferParams {
                from_account_id: from_account.id,
                to_account_id: to_account.id,
                amount: money,
            },
        )
        .await
        .unwrap();

        assert_eq!(transfer.from_account_id, from_account.id);
        assert_eq!(transfer.to_account_id, to_account.id);
        assert_eq!(transfer.amount, money);
    }

    #[tokio::test]
    async fn test_get_transfer() {
        dotenv::dotenv().ok();
        let db = create_connection_pool(Some(10))
            .await
            .expect("Failed to create connection pool");

        let from_account = random_account(&db).await.unwrap();
        let to_account = random_account(&db).await.unwrap();

        let money = random_int(10, from_account.balance);
        let transfer = random_transfer(&db, from_account.id, to_account.id, money)
            .await
            .unwrap();

        let got = get_transfer(&db, transfer.id).await.unwrap();
        assert_eq!(got, transfer);
    }
}
