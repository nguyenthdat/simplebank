use crate::model::Account;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CreateAccountParams {
    pub owner: String,
    pub balance: i64,
    pub currency: String,
}

static CREATE_ACCOUNT: &'static str =
    "INSERT INTO accounts (owner, balance, currency) VALUES ($1, $2, $3) RETURNING *;";

pub async fn create_account(pool: &sqlx::PgPool, arg: CreateAccountParams) -> Result<Account> {
    let mut tx = pool.begin().await?;

    let account: Account = sqlx::query_as(CREATE_ACCOUNT)
        .bind(arg.owner)
        .bind(arg.balance)
        .bind(arg.currency)
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(account)
}

static GET_ACCOUNT: &'static str = "SELECT * FROM accounts WHERE id = $1 LIMIT 1;";

pub async fn get_account(pool: &sqlx::PgPool, id: i64) -> Result<Account> {
    let account = sqlx::query_as(GET_ACCOUNT).bind(id).fetch_one(pool).await?;
    Ok(account)
}

#[derive(Debug, Clone)]
pub struct ListAccountsParams {
    pub limit: i64,
    pub offset: i64,
}

static LIST_ACCOUNTS: &'static str = "SELECT * FROM accounts ORDER BY id LIMIT $1 OFFSET $2;";

pub async fn list_accounts(pool: &sqlx::PgPool, arg: ListAccountsParams) -> Result<Vec<Account>> {
    let accounts = sqlx::query_as(LIST_ACCOUNTS)
        .bind(arg.limit)
        .bind(arg.offset)
        .fetch_all(pool)
        .await?;
    Ok(accounts)
}

static UPDATE_ACCOUNT: &'static str = "UPDATE accounts SET balance = $2 WHERE id = $1 RETURNING *;";

pub async fn update_account(pool: &sqlx::PgPool, id: i64, balance: i64) -> Result<Account> {
    let mut tx = pool.begin().await?;

    let account: Account = sqlx::query_as(UPDATE_ACCOUNT)
        .bind(id)
        .bind(balance)
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(account)
}

static DELETE_ACCOUNT: &'static str = "DELETE FROM accounts WHERE id = $1;";

pub async fn delete_account(pool: &sqlx::PgPool, id: i64) -> Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query(DELETE_ACCOUNT)
        .bind(id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}

mod tests {
    use super::*;
    use crate::{db::create_connection_pool, util::*};

    async fn create_random_account(pool: &sqlx::PgPool) -> Result<Account> {
        let mut tx = pool.begin().await?;

        let arg = CreateAccountParams {
            owner: random_owner(),
            balance: random_money(),
            currency: random_currency(),
        };

        let account: Account = sqlx::query_as(CREATE_ACCOUNT)
            .bind(arg.owner)
            .bind(arg.balance)
            .bind(arg.currency)
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(account)
    }

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

        let account = create_random_account(&db).await.unwrap();
        let account2 = get_account(&db, account.id).await.unwrap();

        assert_eq!(account, account2);
    }

    #[tokio::test]
    async fn test_update_account() {
        dotenv::dotenv().ok();
        let db = create_connection_pool(Some(10))
            .await
            .expect("Failed to create connection pool");

        let account = create_random_account(&db).await.unwrap();
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

        let account = create_random_account(&db).await.unwrap();
        delete_account(&db, account.id).await.unwrap();

        let account2 = get_account(&db, account.id).await;
        assert!(account2.is_err());
        assert!(account2.unwrap_err().is::<sqlx::Error>());
    }
}
