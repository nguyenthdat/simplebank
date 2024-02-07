use crate::model::Transfer;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CreateTransferParams {
    pub from_account_id: i64,
    pub to_account_id: i64,
    pub amount: i64,
}

pub async fn create_transfer(pool: &sqlx::PgPool, arg: CreateTransferParams) -> Result<Transfer> {
    let mut tx = pool.begin().await?;

    let res = sqlx::query_as!(
        Transfer,
        "INSERT INTO transfers (from_account_id, to_account_id, amount) VALUES ($1, $2, $3) RETURNING *;",
        arg.from_account_id,
        arg.to_account_id,
        arg.amount
    )
    .fetch_one(&mut *tx)
    .await;

    match res {
        Ok(transfer) => {
            tx.commit().await?;
            Ok(transfer)
        }
        Err(err) => {
            tx.rollback().await?;
            Err(err.into())
        }
    }
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

pub async fn update_transfer(pool: &sqlx::PgPool, id: i64, amount: i64) -> Result<Transfer> {
    let mut tx = pool.begin().await?;

    let res = sqlx::query_as!(
        Transfer,
        "UPDATE transfers SET amount = $1 WHERE id = $2 RETURNING *;",
        amount,
        id
    )
    .fetch_one(&mut *tx)
    .await;

    match res {
        Ok(transfer) => {
            tx.commit().await?;
            Ok(transfer)
        }
        Err(err) => {
            tx.rollback().await?;
            Err(err.into())
        }
    }
}

pub async fn delete_transfer(pool: &sqlx::PgPool, id: i64) -> Result<Transfer> {
    let mut tx = pool.begin().await?;

    let res = sqlx::query_as!(
        Transfer,
        "DELETE FROM transfers WHERE id = $1 RETURNING *;",
        id
    )
    .fetch_one(&mut *tx)
    .await;

    match res {
        Ok(transfer) => {
            tx.commit().await?;
            Ok(transfer)
        }
        Err(err) => {
            tx.rollback().await?;
            Err(err.into())
        }
    }
}

mod tests {
    use super::*;
    use crate::{db::create_connection_pool, util::*};

    #[tokio::test]
    async fn test_create_transfer() {
        dotenv::dotenv().ok();
        let db = create_connection_pool(Some(10))
            .await
            .expect("Failed to create connection pool");

        let from_account = random_account(&db).await.unwrap();
        let to_account = random_account(&db).await.unwrap();

        let money = random_int(10, from_account.balance);
        let transfer = create_transfer(
            &db,
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

    #[tokio::test]
    async fn test_update_transfer() {
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

        let new_amount = random_int(10, from_account.balance);
        let updated = update_transfer(&db, transfer.id, new_amount).await.unwrap();
        assert_eq!(updated.amount, new_amount);
    }

    #[tokio::test]
    async fn test_delete_transfer() {
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

        let deleted = delete_transfer(&db, transfer.id).await.unwrap();
        assert_eq!(deleted, transfer);
    }
}
