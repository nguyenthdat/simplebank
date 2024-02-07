use crate::{
    model::{Entry, Transfer},
    prelude::*,
};
use rand::Rng;

use crate::{db::account_sql::CreateAccountParams, model::Account};

pub fn random_int(min: i64, max: i64) -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn random_string(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let s: String = (0..len)
        .map(|_| {
            let c: char = rng.gen_range(b'a'..b'z') as char;
            c
        })
        .collect();
    s
}

pub fn random_owner() -> String {
    random_string(6)
}

pub fn random_money() -> i64 {
    random_int(0, 1000)
}

pub fn random_currency() -> String {
    let currencies = vec!["USD", "EUR", "JPY", "CNY", "KRW"];
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..currencies.len());
    currencies[idx].to_string()
}

pub async fn random_account(pool: &sqlx::PgPool) -> Result<Account> {
    let mut tx = pool.begin().await?;

    let arg = CreateAccountParams {
        owner: random_owner(),
        balance: random_money(),
        currency: random_currency(),
    };

    let account: Account = sqlx::query_as(
        "INSERT INTO accounts (owner, balance, currency) VALUES ($1, $2, $3) RETURNING *;",
    )
    .bind(arg.owner)
    .bind(arg.balance)
    .bind(arg.currency)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(account)
}

pub async fn random_entry(pool: &sqlx::PgPool, account_id: i64) -> Result<Entry> {
    let mut tx = pool.begin().await?;

    let amount = random_money();

    let entry: Entry =
        sqlx::query_as("INSERT INTO entries (account_id, amount) VALUES ($1, $2) RETURNING *;")
            .bind(account_id)
            .bind(amount)
            .fetch_one(&mut *tx)
            .await?;

    tx.commit().await?;
    Ok(entry)
}

pub async fn random_transfer(
    pool: &sqlx::PgPool,
    from_account_id: i64,
    to_account_id: i64,
    amount: i64,
) -> Result<Transfer> {
    let mut tx = pool.begin().await?;

    let transfer: Transfer = sqlx::query_as(
        "INSERT INTO transfers (from_account_id, to_account_id, amount) VALUES ($1, $2, $3) RETURNING *;",
    )
    .bind(from_account_id)
    .bind(to_account_id)
    .bind(amount)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(transfer)
}
