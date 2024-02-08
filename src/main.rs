#![allow(unused)]

mod api;
mod db;
mod error;
mod handlers;
mod models;
mod prelude;
mod utils;

use api::router;

use crate::api::server::Server;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db = db::create_connection_pool(Some(10))
        .await
        .expect("Failed to create connection pool");

    let router = api::router::routes(db);

    Server::builder().router(router).build().await.run().await;
}
