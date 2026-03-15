use crate::AppState;
use axum::{
    Json as JsonResponse, debug_handler,
    extract::{Json, State},
};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

pub async fn register_user(
    State(app_state): State<Arc<AppState>>,
    Json(mut user): Json<User>,
) -> JsonResponse<UserRegisterResponse> {
    println!("Users input: {:?}", user);

    let mut users = app_state.users.lock().unwrap();
    println!("Users stored: {:?}", users);

    if users.contains_key(&user.id.to_string()) {
        return JsonResponse(UserRegisterResponse {
            username: user.username,
            message: String::from("User already exists"),
        });
    }
    user.username = String::from(user.username + &Uuid::new_v4().to_string());
    users.insert(user.id.to_string(), user.clone());

    println!("User inserted: {:?}", users.get(&user.id.to_string()));

    JsonResponse(UserRegisterResponse {
        username: user.username,
        message: String::from("Registration successful"),
    })
}

#[debug_handler]
pub async fn login_user(
    State(app_state): State<Arc<AppState>>,
    Json(user): Json<UserLoginRequest>,
) -> JsonResponse<UserLoginResponse> {
    println!("Users input: {:?}", user);

    let users = app_state.users.lock().unwrap();
    println!("Users stored: {:?}", users);

    if !users.contains_key(&user.id.to_string()) {
        return JsonResponse(UserLoginResponse {
            username: None,
            token: None,
            message: String::from("User not found"),
        });
    }

    let user_data = users.get(&user.id.to_string()).unwrap().clone();
    if user.password != user_data.password {
        return JsonResponse(UserLoginResponse {
            username: None,
            token: None,
            message: String::from("Incorrect password"),
        });
    };

    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600;

    let claims = Claims {
        sub: user_data.username.to_string(),
        exp: exp as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    println!("JWT: {:?}", token);
    JsonResponse(UserLoginResponse {
        username: Some(user_data.username),
        token: Some(token),
        message: String::from("login successful"),
    })
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: String,
    name: String,
    role: Role,
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserRegisterResponse {
    username: String,
    message: String,
}

#[derive(Debug, Deserialize)]
pub struct UserLoginRequest {
    id: String,
    password: String,
}
#[derive(Debug, Serialize)]
pub struct UserLoginResponse {
    username: Option<String>,
    token: Option<String>,
    message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Role {
    ADMIN,
    USER,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}
