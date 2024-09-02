use crate::auth::jwt::jwt_gen;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::extract::Request;
use google_oauth::AsyncClient;


pub async fn jwt(request : Request) -> impl IntoResponse {
    println!("in jwt handler");
    let auth_header = match request.headers().get("Authorization") {
        Some(value) => value.to_str().unwrap_or(""),
        None => return Err(StatusCode::UNAUTHORIZED),
    }; 
    println!("got header : {:?}",auth_header);
    // Check if the token is prefixed with "Bearer "
    let token = if auth_header.starts_with("Bearer ") {
        &auth_header[7..] // Remove the "Bearer " prefix to get the token
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let client_id = "your client id";
    let client = AsyncClient::new(client_id);
    println!("got jwt : {:?}",token);

    let payload = client.validate_id_token(token).await.unwrap(); // In production, remember to handle this error.

    println!("Hello, I am {}", &payload.sub);
    if Some(&payload.sub) == None {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let token = jwt_gen(payload.sub).await;
    return Ok((StatusCode::OK, token).into_response());
}
