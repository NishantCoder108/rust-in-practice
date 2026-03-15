use axum::{
    Router,
    routing::{get, post},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod user;
use user::{login_user, register_user};

use crate::user::User;

pub struct AppState {
    users: Mutex<HashMap<String, User>>,
}

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState {
        users: Mutex::new(HashMap::new()),
    });
    let app = Router::new()
        .route("/v1/register", post(register_user))
        .route("/v1/login", post(login_user))
        .with_state(app_state);
    // .route("/v1/events", get(events_get))
    // .route("/v1/event/:id", get(event_get))
    // .route("/v1/event/:id", put(event_put))
    // .route("/v1/event/:id", delete(event_delete));

    //ip address  and listen the port
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    //serve the port and runnning the application
    axum::serve(listener, app).await.unwrap();
}
