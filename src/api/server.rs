use axum::Router;
use sqlx::PgPool;
use tokio::net::TcpListener;

pub struct Server {
    listener: TcpListener,
    router: Router,
    db: PgPool,
}

pub struct ServerBuilder {
    listener: Option<TcpListener>,
    router: Option<Router>,
    db: Option<PgPool>,
}

impl Server {
    pub fn builder() -> ServerBuilder {
        ServerBuilder {
            listener: None,
            router: None,
            db: None,
        }
    }

    pub async fn run(self) {
        axum::serve(self.listener, self.router)
            .await
            .expect("Failed to start server");
    }
}

impl ServerBuilder {
    pub fn listener(mut self, listener: TcpListener) -> Self {
        self.listener = Some(listener);
        self
    }

    pub fn router(mut self, router: Router) -> Self {
        self.router = Some(router);
        self
    }

    pub fn db(mut self, db: PgPool) -> Self {
        self.db = Some(db);
        self
    }

    pub async fn build(self) -> Server {
        let listener = match self.listener {
            Some(listener) => listener,
            None => TcpListener::bind("127.0.0.1:3000")
                .await
                .expect("Failed to bind to address"),
        };

        Server {
            listener,
            router: self.router.expect("router is required"),
            db: self.db.expect("db is required"),
        }
    }
}
