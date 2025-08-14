use crate::ports;
use ports::get_free_port;
use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub async fn setup() {
    let naomii_port = get_free_port();
    let addr = format!("0.0.0.0:{}", naomii_port);

    println!("{}", naomii_port);

    let app = Router::new().route("/", get(|| async { "hi" }));
    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
