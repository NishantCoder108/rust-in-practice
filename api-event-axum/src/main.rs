use axum::{
    Router,
    routing::{delete, get, post},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod event;
mod user;
use crate::event::Event;
use crate::user::User;
use event::{delete_event, event_create, get_event_by_id, get_events};
use user::{login_user, register_user};

pub struct AppState {
    users: Mutex<HashMap<String, User>>,
    events: Mutex<Vec<Event>>,
}

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState {
        users: Mutex::new(HashMap::new()),
        events: Mutex::new(Vec::new()),
    });
    let app = Router::new()
        .route("/v1/register", post(register_user))
        .route("/v1/login", post(login_user))
        .route("/v1/event", post(event_create))
        .route("/v1/events", get(get_events))
        .route("/v1/event/{id}", get(get_event_by_id))
        .route("/v1/event/{id}", delete(delete_event))
        .with_state(app_state);
    // .route("/v1/event/:id", put(event_put))
    // .route("/v1/event/:id", delete(event_delete));

    //ip address  and listen the port
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    //serve the port and runnning the application
    axum::serve(listener, app).await.unwrap();
}
