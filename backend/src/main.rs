mod handler;
mod auth;
use std::collections::HashMap;
use auth::jwt::verify_jwt_middleware;
use axum::middleware;
use axum::{
    routing::post,
    Router,
};
use tokio::sync::Mutex;
use std::sync::Arc;
use handler::startup::{start_container, ServerMap,Server};
use handler::jwt::jwt;


#[tokio::main]
async fn main() {
    // let x = jwt_gen("user_email".to_string()).await;
    // println!("x is : {:?}",x);
    let server_map = ServerMap(HashMap::from([
        ("server1".to_string(), Server { busy: false, port: 6081 }),
        ("server2".to_string(), Server { busy: false, port: 6082 }),
        ("server3".to_string(), Server { busy: false, port: 6083 })

    ]));

    let server_map = Arc::new(Mutex::new(server_map));

    let app = Router::new()
        .route("/", post(start_container))
        .route_layer(middleware::from_fn(verify_jwt_middleware))
        .route("/auth/jwt", post(jwt))
        .with_state(server_map);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
