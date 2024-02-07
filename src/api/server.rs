use axum::Router;
use tokio::net::TcpListener;

pub struct Server {
    listener: TcpListener,
    router: Router,
}
