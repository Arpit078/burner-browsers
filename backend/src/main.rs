mod handler;
mod auth;
use std::collections::HashMap;
use axum::{
    routing::post,
    Router,
};
use tokio::sync::Mutex;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use handler::startup::{start_container, ServerMap, Server};

#[tokio::main]
async fn main() {
    let server_map = ServerMap(HashMap::from([
        ("server1".to_string(), Server { busy: false, port: 6081 }),
        ("server2".to_string(), Server { busy: false, port: 6082 }),
        ("server3".to_string(), Server { busy: false, port: 6083 })
    ]));

    let server_map = Arc::new(Mutex::new(server_map));

    // Define the CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow requests from any origin
        .allow_methods(Any) // Allow any HTTP methods (GET, POST, etc.)
        .allow_headers(Any); // Allow any headers

    let app = Router::new()
        .route("/", post(start_container))
        .with_state(server_map)
        .layer(cors); // Apply the CORS middleware

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}