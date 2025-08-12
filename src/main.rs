mod ports;

use axum::{
    routing::get,
    Router,
    http::Request
};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/health", get(health));

    println!("{}", ports::get_free_port());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
fn log(route: &str, method: &str, uragent: &str) {
    println!("request::{} on {} by {}", method, route, uragent)
}
async fn health(req: Request<axum::body::Body>) -> &'static str {
    let header = req.headers();
    let method = req.method();
    let uragent = header.get("user-agent").map(|v| v.to_str().unwrap_or(""));

    log("health", &method.to_string(), uragent.expect(""));
    "200 Ok"
}