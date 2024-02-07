use crate::{
    db::{
        account_sql::{get_account_for_update, update_account_tx},
        entry_sql::{create_entry, CreateEntryParams},
        transfer_sql::{create_transfer, CreateTransferParams},
    },
    model::{Account, Entry, Transfer},
    prelude::*,
};
use futures::future::BoxFuture;
use sqlx::PgPool;

macro_rules! execute_transaction {
    ($tx:expr, $action:expr) => {
        match $action.await {
            Ok(result) => result,
            Err(err) => {
                $tx.rollback().await?;
                return Err(err.into());
            }
        }
    };
}

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
    let mut tx = pool.begin().await?;

    let transfer = execute_transaction!(
        tx,
        create_transfer(
            &mut tx,
            CreateTransferParams {
                from_account_id: arg.from_account_id,
                to_account_id: arg.to_account_id,
                amount: arg.amount,
            },
        )
    );

    let from_entry = execute_transaction!(
        tx,
        create_entry(
            &mut tx,
            CreateEntryParams {
                account_id: arg.from_account_id,
                amount: -arg.amount,
            },
        )
    );

    let to_entry = execute_transaction!(
        tx,
        create_entry(
            &mut tx,
            CreateEntryParams {
                account_id: arg.from_account_id,
                amount: arg.amount,
            },
        )
    );

    let from_account =
        execute_transaction!(tx, get_account_for_update(&mut tx, arg.from_account_id));

    let to_account = execute_transaction!(tx, get_account_for_update(&mut tx, arg.to_account_id));

    let from_account = execute_transaction!(
        tx,
        update_account_tx(&mut tx, from_account.id, from_account.balance - arg.amount,)
    );

    let to_account = execute_transaction!(
        tx,
        update_account_tx(&mut tx, to_account.id, to_account.balance + arg.amount)
    );

    tx.commit().await?;

    let result = TransferTxResult {
        transfer,
        from_account,
        to_account,
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

        println!(
            ">> -- from_account balance before tx: {}",
            from_account.balance
        );
        println!(">> -- to_account balance before tx: {}", to_account.balance);

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

            // assert_eq!(
            //     transfer.from_account.balance + transfer.transfer.amount,
            //     transfer.to_account.balance
            // );
            // assert_eq!(
            //     transfer.to_account.balance - transfer.transfer.amount,
            //     transfer.to_account.balance
            // );
            // assert_eq!(
            //     transfer.from_account.balance - transfer.transfer.amount,
            //     transfer.from_account.balance
            // );
            // assert!(transfer.from_account.balance >= 0);
            // assert!(transfer.to_account.balance >= 0);
            // assert!(transfer.from_account.balance <= from_account.balance);
            // assert!(transfer.to_account.balance >= to_account.balance);
        }

        println!(
            ">> -- from_account balance after tx: {}",
            get_account(&pool, from_account.id).await.unwrap().balance
        );

        println!(
            ">> -- to_account balance after tx: {}",
            get_account(&pool, to_account.id).await.unwrap().balance
        );
    }
}
