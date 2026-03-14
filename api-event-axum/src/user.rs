use axum::{Json as JsonResponse, debug_handler, extract::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[debug_handler]
pub async fn register_user(Json(mut u): Json<User>) -> JsonResponse<UserRegisterResponse> {
    println!(
        "Name: {}, Role: {:?}, Username: {}, Password: {}",
        u.name, u.role, u.username, u.password
    );
    u.id = Some(Uuid::new_v4().to_string());

    JsonResponse(UserRegisterResponse {
        id: u.id.unwrap(),
        message: String::from("Registration successful"),
    })
}

// pub fn login_user(Json(u): Json<User>) -> Json<UserLoginResponse> {
//     Json(UserLoginResponse {
//         id: u.id.unwrap(),
//         token: String::from("test"),
//         message: String::from("login successful"),
//     })
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Option<String>,
    name: String,
    role: Role,
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserRegisterResponse {
    id: String,
    message: String,
}

pub struct UserLoginResponse {
    id: String,
    token: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum Role {
    ADMIN,
    USER,
}
