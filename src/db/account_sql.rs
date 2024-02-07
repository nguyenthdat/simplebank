use crate::model::Account;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CreateAccountParams {
    pub owner: String,
    pub balance: i64,
    pub currency: String,
}

pub async fn create_account(pool: &sqlx::PgPool, arg: CreateAccountParams) -> Result<Account> {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;

    let res = sqlx::query_as!(
        Account,
        "INSERT INTO accounts (owner, balance, currency) VALUES ($1, $2, $3) RETURNING *;",
        arg.owner,
        arg.balance,
        arg.currency
    )
    .fetch_one(&mut *tx)
    .await;

    match res {
        Ok(account) => {
            tx.commit().await?;
            Ok(account)
        }
        Err(err) => {
            tx.rollback().await?;
            Err(err.into())
        }
    }
}

pub async fn get_account(pool: &sqlx::PgPool, id: i64) -> Result<Account> {
    let account = sqlx::query_as!(Account, "SELECT * FROM accounts WHERE id = $1 LIMIT 1;", id)
        .fetch_one(pool)
        .await?;
    Ok(account)
}

#[derive(Debug, Clone)]
pub struct ListAccountsParams {
    pub limit: i64,
    pub offset: i64,
}

pub async fn list_accounts(pool: &sqlx::PgPool, arg: ListAccountsParams) -> Result<Vec<Account>> {
    let accounts = sqlx::query_as!(
        Account,
        "SELECT * FROM accounts ORDER BY id LIMIT $1 OFFSET $2;",
        arg.limit,
        arg.offset
    )
    .fetch_all(pool)
    .await?;
    Ok(accounts)
}

pub async fn update_account(pool: &sqlx::PgPool, id: i64, balance: i64) -> Result<Account> {
    let mut tx = pool.begin().await?;

    let res = sqlx::query_as!(
        Account,
        "UPDATE accounts SET balance = $2 WHERE id = $1 RETURNING *;",
        id,
        balance
    )
    .fetch_one(&mut *tx)
    .await;

    match res {
        Ok(account) => {
            tx.commit().await?;
            Ok(account)
        }
        Err(err) => {
            tx.rollback().await?;
            Err(err.into())
        }
    }
}

pub async fn delete_account(pool: &sqlx::PgPool, id: i64) -> Result<()> {
    let mut tx = pool.begin().await?;

    let res = sqlx::query!("DELETE FROM accounts WHERE id = $1;", id)
        .execute(&mut *tx)
        .await;

    match res {
        Ok(_) => {
            tx.commit().await?;
        }
        Err(err) => {
            tx.rollback().await?;
            return Err(err.into());
        }
    }
    Ok(())
}

mod tests {
    use super::*;
    use crate::{db::create_connection_pool, util::*};

    #[tokio::test]
    async fn test_create_account() {
        dotenv::dotenv().ok();
        let db = create_connection_pool(Some(10))
            .await
            .expect("Failed to create connection pool");

        let arg = CreateAccountParams {
            owner: random_owner(),
            balance: random_money(),
            currency: random_currency(),
        };

        let account = create_account(&db, arg.clone()).await.unwrap();
        assert_eq!(account.owner, arg.owner);
        assert_eq!(account.balance, arg.balance);
        assert_eq!(account.currency, arg.currency);

        assert_ne!(account.id, 0);
    }

    #[tokio::test]
    async fn test_get_account() {
        dotenv::dotenv().ok();
        let db = create_connection_pool(Some(10))
            .await
            .expect("Failed to create connection pool");

        let account = random_account(&db).await.unwrap();
        let account2 = get_account(&db, account.id).await.unwrap();

        assert_eq!(account, account2);
    }

    #[tokio::test]
    async fn test_update_account() {
        dotenv::dotenv().ok();
        let db = create_connection_pool(Some(10))
            .await
            .expect("Failed to create connection pool");

        let account = random_account(&db).await.unwrap();
        let new_balance = random_money();
        let account2 = update_account(&db, account.id, new_balance).await.unwrap();

        assert_eq!(account2.balance, new_balance);
        assert_eq!(account2.id, account.id);
        assert_eq!(account2.owner, account.owner);
    }

    #[tokio::test]
    async fn test_delete_account() {
        dotenv::dotenv().ok();
        let db = create_connection_pool(Some(10))
            .await
            .expect("Failed to create connection pool");

        let account = random_account(&db).await.unwrap();
        delete_account(&db, account.id).await.unwrap();

        let account2 = get_account(&db, account.id).await;
        assert!(account2.is_err());
        assert!(account2.unwrap_err().is::<sqlx::Error>());
    }
}
