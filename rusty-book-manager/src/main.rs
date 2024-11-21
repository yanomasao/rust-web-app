use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main] // main関数を非同期関数にする
async fn main() {
    // ルータの生成
    let app = Router::new().route("/hello", get(hello_world));
    // ポート設定
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    // サーバの起動
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

// ハンドラ
async fn hello_world() -> &'static str {
    "Hello, world!"
}
