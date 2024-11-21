use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main] // main関数を非同期関数にする
async fn main() -> Result<()> {
    // ルータの生成
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/health", get(health_check));
    // ポート設定
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    // サーバの起動
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);
    Ok(axum::serve(listener, app).await?)
}

// ハンドラ
async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::test]
async fn health_check_works() {
    let status_code = health_check().await;
    assert_eq!(status_code, StatusCode::OK);
}
