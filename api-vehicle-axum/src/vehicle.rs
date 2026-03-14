use axum::{
    debug_handler,
    extract::{Json, Query},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: Option<String>,
}

// #[debug_handler] //It provide readable debug error
pub async fn vehicle_get() -> Json<Vehicle> {
    Json(Vehicle {
        manufacturer: String::from("Bullet"),
        model: String::from("Classic350"),
        year: 2022,
        id: Some(uuid::Uuid::new_v4().to_string()),
    })
}

//For passing inside body, we can use JSON
// #[debug_handler]
// pub async fn vehicle_post(Json(mut v): Json<Vehicle>) -> Json<Vehicle> {
//     println!(
//         "Manufacturer: {}, Model: {}, Year: {}",
//         v.manufacturer, v.model, v.year
//     );
//     v.id = Some(Uuid::new_v4().to_string());

//     Json(v)
// }

//For using query parameters, we can use extract::Query
pub async fn vehicle_post(Query(mut v): Query<Vehicle>) -> Json<Vehicle> {
    println!(
        "Manufacturer: {}, Model: {}, Year: {}",
        v.manufacturer, v.model, v.year
    );
    v.id = Some(Uuid::new_v4().to_string());

    Json(v)
}
