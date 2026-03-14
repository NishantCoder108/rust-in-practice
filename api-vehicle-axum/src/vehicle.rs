use axum::{Json, debug_handler};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: String,
}

// #[debug_handler] //It provide readable debug error
pub async fn vehicle_get() -> Json<Vehicle> {
    Json(Vehicle {
        manufacturer: String::from("Bullet"),
        model: String::from("Classic350"),
        year: 2022,
        id: uuid::Uuid::new_v4().to_string(),
    })
}

pub async fn vehicle_post() {}
