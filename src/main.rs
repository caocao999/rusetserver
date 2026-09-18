//Cisco　IOS　ACL テスト用ダミーサーバ
// 起動時の引数でポート番号が変わります

use std::{env, println};

use axum::response::Html;
use axum::{Router, routing::get};

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut port = "3000".to_string();

    if args.len() == 2 {
        port = args[1].clone();
    }

    println!("{}", port);

    let server_addr = format!("0.0.0.0:{}", port);

    let app = Router::new().route("/", get(index));
    let listener = tokio::net::TcpListener::bind(server_addr).await?;
    println!("API Sever started!!  on {}", listener.local_addr()?);
    axum::serve(listener, app).await
}
