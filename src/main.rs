mod ports;

use axum::{
    routing::get,
    Router,
    http::Request
};
use ports::get_free_port;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(health));
    
    println!("running on {} [unorganized mains]", get_free_port());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("mains [unorganized] running on 0.0.0.0:3000 ");
    axum::serve(listener, app).await.unwrap();
}
fn log(route: &str, method: &str) {
    println!("request::{} on {} for mains [unorganized]", method, route)
}
async fn health(req: Request<axum::body::Body>) -> &'static str {
    let method = req.method();

    log("health", &method.to_string());
    "200 Ok"
}