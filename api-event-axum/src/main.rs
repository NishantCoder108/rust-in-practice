use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod auth;
mod event;
mod user;
use crate::event::Event;
use crate::user::User;
use auth::auth_middleware;
use event::{delete_event, event_create, get_event_by_id, get_events, update_event};
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
    let public_routes = Router::new()
        .route("/v1/register", post(register_user))
        .route("/v1/login", post(login_user));

    let protected_routes = Router::new()
        .route("/v1/event", post(event_create))
        .route("/v1/events", get(get_events))
        .route("/v1/event/{id}", get(get_event_by_id))
        .route("/v1/event", put(update_event))
        .route("/v1/event/{id}", delete(delete_event))
        .layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(app_state);

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
