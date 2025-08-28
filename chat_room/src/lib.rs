pub mod config;
pub mod error;
pub mod openapi;
pub mod handlers;

use std::sync::Arc;

use anyhow::Result;
use axum::{Router, routing::get};
use config::AppConfig;


#[derive(Debug, Clone)]
pub(crate) struct AppState {
  inner: Arc<AppStateInner>,
}

#[allow(unused)]
#[derive(Debug)]
struct AppStateInner {
  pub(crate) config: AppConfig,
  // pub(crate) dk: DecodingKey,
  // pub(crate) ek: EncodingKey,
  // pub(crate) pool: PgPool,
}

// 
pub fn get_router(state: AppState) -> Result<Router> {
  // let router = Router::new().route("/", get(|| async { "hello, chat room" }));
  // Ok(router)

  todo!();
}