use axum::{routing::get, Router};
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

use crate::CmdExector;

#[derive(Debug, Parser)]
pub struct FileServeOpts {
    #[clap(long, default_value = "./")]
    pub path: String,
    #[clap(long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExector for FileServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let path = std::path::Path::new(&self.path);
        let service = ServeDir::new(path);

        let app = Router::new()
            .route("/", get(index_handle))
            .nest_service("/tower", service);
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr).await?;
        info!("Listening on http://{}", listener.local_addr()?);
        axum::serve(listener, app).await?;
        Ok(())
    }
}

async fn index_handle() -> &'static str {
    "Hello World"
}
