use crate::fs::create_new_file_copy;
use crate::ports;
use ports::get_free_port;
use axum::{
    extract::State,
    response::Redirect,
    routing::get,
    Router,
};
use std::{error::Error, sync::Arc};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    server_uri: String,
    local_server_path: String
}


pub async fn setup(nport: u32, server_uri: String, local_server_path: String) -> Result<(), Box<dyn Error>> {    
    let naomii_port: u32 = if nport == 0 {
        get_free_port()
    } else {
        nport
    };
    let addr = format!("0.0.0.0:{}", naomii_port);
    println!("Naomii running on port {}", naomii_port);

    let state = Arc::new(AppState { server_uri, local_server_path });

    let app = Router::new()
        .route("/", get(redirect_with_balance))
        .with_state(state);

    let listener = TcpListener::bind(&addr).await
        .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;

    axum::serve(listener, app).await
        .map_err(|e: std::io::Error| format!("Server error: {}", e))?;

    Ok(())
}

async fn redirect_with_balance(State(state): State<Arc<AppState>>) -> Redirect {
    let _ = create_new_file_copy(&state.local_server_path);
    println!("{}", &state.local_server_path);
    Redirect::temporary(&state.server_uri)
}
