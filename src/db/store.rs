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

#[derive(Debug, Clone)]
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

    let result = TransferTxResult {
        transfer,
        from_account: account_sql::get_account(&pool, arg.from_account_id).await?,
        to_account: account_sql::get_account(&pool, arg.to_account_id).await?,
        from_entry,
        to_entry,
    };
    Ok(result)
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

        // run n concurrent transfer transactions
        let n = 10;
        let mut handles = vec![];

        for _ in 0..n {
            let pool = pool.clone();
            let from_account = from_account.clone();
            let to_account = to_account.clone();
            let amount = random_int(10, from_account.balance);

            let handle = tokio::spawn(async move {
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
                result
            });
            handles.push(handle);
        }

        let transfers = futures::future::join_all(handles).await;

        assert_ne!(transfers.len(), 0);
        for transfer in transfers {
            let transfer = transfer.unwrap();
            assert_eq!(transfer.from_account.id, from_account.id);
            assert_eq!(transfer.to_account.id, to_account.id);
            assert_eq!(transfer.from_entry.amount, -transfer.to_entry.amount);
            assert_eq!(transfer.from_entry.amount, -transfer.transfer.amount);
            assert_eq!(transfer.to_entry.amount, transfer.transfer.amount);
            assert_ne!(transfer.transfer.id, 0);

            // TODO: Check the account balances
        }
    }
}
