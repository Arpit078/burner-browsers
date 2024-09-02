use jsonwebtoken::{encode,decode,Validation, Header, EncodingKey,DecodingKey, Algorithm, TokenData};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub async fn jwt_gen(user_email:String) -> String {
    // Define the claims for the token
    let my_claims = Claims {
        sub: user_email,
        exp: (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            + std::time::Duration::from_secs(60 * 10)) // 10 min expiration
            .as_secs() as usize,
    };

    // Define the secret key
    // let secret = env::var("JWT_SECRET").unwrap();

    let secret = "Arpit";

    // Generate the token
    let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret.as_ref())) {
        Ok(t) => t,
        Err(_) => panic!("Token creation failed"), // Proper error handling should be implemented
    };

    token
}

pub async fn jwt_verify(token: &str) -> Result<TokenData<Claims>, String> {
    // Get the secret key from the environment variable

    // let secret = match env::var("JWT_SECRET") {
    //     Ok(val) => val,
    //     Err(_) => return Err("JWT_SECRET environment variable not found".to_string()),
    // };
    let secret = "Arpit";

    // Set up the validation parameters (e.g., Algorithm::HS256)
    let validation = Validation::new(Algorithm::HS256);

    // Decode and verify the token
    match decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation) {
        Ok(token_data) => Ok(token_data),
        Err(err) => Err(format!("Token verification failed: {}", err)),
    }
}


pub async fn verify_jwt_middleware(request : Request,next: Next) -> Result<Response,StatusCode> {
    // Extract the "Authorization" header
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
    println!("got jwt : {:?}",token);
    // Verify the token
    match jwt_verify(token).await {
        Ok(_token_data) => {
            // Token is valid
            Ok(next.run(request).await)
        },
        Err(_) => {
            // Token is invalid or verification failed
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}