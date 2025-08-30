use crate::{fs::create_new_file_copy, ports::get_free_port};

use std::{
    error::Error,
    sync::{Arc, atomic::{AtomicU16, Ordering}},
};
use axum::{extract::State, response::Redirect, routing::get, Router};
use tokio::net::TcpListener;

struct AppState {
    server_uri: String,
    local_server_path: String,
    users: AtomicU16,
}

pub async fn setup(nport: u32, server_uri: String, local_server_path: String) -> Result<(), Box<dyn Error>> {
    let port = if nport == 0 { get_free_port() } else { nport };
    let addr = format!("0.0.0.0:{port}");
    println!("Naomii running on port {port}");

    let state = Arc::new(AppState {
        server_uri,
        local_server_path,
        users: AtomicU16::new(0),
    });

    let app = Router::new()
        .route("/", get(redirect))
        .with_state(state.clone()); // Arc is Clone

    let listener = TcpListener::bind(&addr).await
        .map_err(|e| format!("Failed to bind to {addr}: {e}"))?;

    axum::serve(listener, app).await
        .map_err(|e: std::io::Error| format!("Server error: {e}"))?;

    Ok(())
}

async fn redirect(State(state): State<Arc<AppState>>) -> Redirect {
    let mut server_uri = state.server_uri.clone(); // take an owned String
    let new_val = state.users.fetch_add(1, Ordering::SeqCst) + 1; // old + 1
    
    if new_val > 5 {
        server_uri = create_new_file_copy(&state.local_server_path)
            .expect("Failed to create new copy");
    }

    println!("Users on server: {new_val}");
    Redirect::temporary(&server_uri) // borrow here is fine
}
