use crate::{
    db::{
        account_sql,
        entry_sql::{create_entry, CreateEntryParams},
        exec_transaction,
        transfer_sql::{create_transfer, CreateTransferParams},
    },
    model::{Account, Entry, Transfer},
    prelude::*,
};
use sqlx::PgPool;

pub struct TransferTxParams {
    pub from_account_id: i64,
    pub to_account_id: i64,
    pub amount: i64,
}

pub struct TransferTxResult {
    pub transfer: Transfer,
    pub from_account: Account,
    pub to_account: Account,
    pub from_entry: Entry,
    pub to_entry: Entry,
}

pub async fn transfer_tx(pool: &PgPool, arg: TransferTxParams) -> Result<TransferTxResult> {
    let transfer = exec_transaction(&pool, |tx| {
        Box::pin(async move {
            let transfer = create_transfer(
                tx,
                CreateTransferParams {
                    from_account_id: arg.from_account_id,
                    to_account_id: arg.to_account_id,
                    amount: arg.amount,
                },
            )
            .await?;
            Ok(transfer)
        })
    })
    .await?;

    let from_entry = exec_transaction(&pool, |tx| {
        Box::pin(async move {
            let entry = create_entry(
                tx,
                CreateEntryParams {
                    account_id: arg.from_account_id,
                    amount: -arg.amount,
                },
            )
            .await?;
            Ok(entry)
        })
    })
    .await?;

    let to_entry = exec_transaction(&pool, |tx| {
        Box::pin(async move {
            let entry = create_entry(
                tx,
                CreateEntryParams {
                    account_id: arg.from_account_id,
                    amount: arg.amount,
                },
            )
            .await?;
            Ok(entry)
        })
    })
    .await?;

    // TODO: Update the account balances
    todo!()
}

mod tests {
    use super::*;
    use crate::{db::create_connection_pool, util::*};

    #[tokio::test]
    async fn test_transfer_tx() {
        dotenv::dotenv().ok();
        let pool = create_connection_pool(Some(10)).await.unwrap();
        let from_account = random_account(&pool).await.unwrap();
        let to_account = random_account(&pool).await.unwrap();
        let amount = random_money();

        let result = transfer_tx(
            &pool,
            TransferTxParams {
                from_account_id: from_account.id,
                to_account_id: to_account.id,
                amount,
            },
        )
        .await
        .unwrap();
    }
}
