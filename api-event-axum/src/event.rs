use crate::AppState;
use axum::{
    Json as JsonResponse, debug_handler,
    extract::{Json, State},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

pub async fn event_create(
    State(app_state): State<Arc<AppState>>,
    Json(mut event): Json<Event>,
) -> JsonResponse<EventResponse> {
    println!("Events request: {:?}", event);
    println!("Event State: {:?}", app_state.events);
    println!("{:_^24}", "_");

    let mut event_state = app_state.events.lock().unwrap();

    let event_id = Uuid::new_v4().to_string();
    event.id = Some(event_id.clone());
    event_state.push(event.clone());

    JsonResponse(EventResponse {
        id: event_id,
        message: String::from("Event created successfully"),
    })
}

#[debug_handler]
pub async fn get_events(
    State(app_state): State<Arc<AppState>>,
) -> JsonResponse<GetAllEventsResponse> {
    let event_state = app_state.events.lock().unwrap();

    println!("Events: {:?}", event_state);
    JsonResponse(GetAllEventsResponse {
        total_events: event_state.len(),
        message: String::from("Events retrieved successfully"),
        events: event_state.clone(),
    })
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    id: Option<String>,
    title: String,
    description: String,
    date: String,
    location: String,
    meet_url: String,
    organizer: String,
}

#[derive(Serialize)]
pub struct EventResponse {
    id: String,
    message: String,
}

#[derive(Serialize)]
pub struct GetAllEventsResponse {
    total_events: usize,
    message: String,
    events: Vec<Event>,
}
