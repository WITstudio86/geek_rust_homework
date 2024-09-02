use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use axum::{
    extract::Request, handler::HandlerWithoutStateExt, response::Html, routing::get, Extension,
    Router,
};
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

use crate::CmdExector;

#[derive(Debug, Parser)]
pub struct FileServeOpts {
    #[clap(long, default_value = ".")]
    pub path: String,
    #[clap(long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExector for FileServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        // 创建服务 path 并创建为共享状态
        let path = PathBuf::from(self.path);
        let appstate = Arc::new(path.clone());
        // 打开文件服务并在找不到文件的时候调用 get_index
        let service = ServeDir::new(path).not_found_service(get_index.into_service());
        // 开启服务
        let app = Router::new()
            .route("/", get(hello))
            .nest_service("/files", service)
            .layer(Extension(appstate));
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr).await?;
        info!("Listening on http://{}", listener.local_addr()?);
        axum::serve(listener, app).await?;

        Ok(())
    }
}

async fn hello() -> Html<String> {
    "<h1>访问 /files 使用文件服务<h1>".to_string().into()
}

/// 返回目录下的文件列表
async fn get_index(
    Extension(app_state): Extension<Arc<PathBuf>>,
    request: Request,
) -> Html<String> {
    // let app_state = app_state.join(Path::new(request.uri().path()));
    let dir = request.uri().path();
    let dir = &dir[1..dir.len() - 1];
    let app_state = Path::new(app_state.as_path()).join(Path::new(dir));
    info!("reading on {:?}", app_state);

    if !app_state.exists() {
        "<h1>文件/路径不存在</h1>".to_string().into()
    } else {
        let mut content = String::from("<h1>index</h1>");
        content.push_str("<p>你访问的是一个目录而不是一个文件 , 当前目录下的项目有:</p>");
        content.push_str("<ul>");
        for file in app_state.read_dir().unwrap() {
            let file = file.unwrap();
            let file_name = file.file_name();
            let file_name = file_name.to_str().unwrap();
            content.push_str(&format!(
                "<li><a href=\"{name}\">{name}</a></li>",
                name = file_name
            ));
        }
        content.push_str("</ul>");
        content.into()
    }
}
