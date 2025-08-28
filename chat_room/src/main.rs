use anyhow::Result;
use tokio::net::TcpListener;
// use std::net::TcpListener;
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};
use chat_room::{config::AppConfig, get_router};
use axum::Router;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, chat_room!");

    // 固定架子代码
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    // 加载配置
    let config: AppConfig = AppConfig::load()?;

    let port: u16 = config.server.port;
    let addr: String = format!("0.0.0.0:{}", port);

    // let app: Router = get_router(config)?;

    let listener: TcpListener = TcpListener::bind(&addr).await?;
    info!("服务启动: {}", addr);

    // axum::serve(listener, app::into_make_service()).await?;
    // axum::serve(listener, app::into_make_service()).await?;

    Ok(())
}
