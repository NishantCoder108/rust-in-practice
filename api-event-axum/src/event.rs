use crate::AppState;
use axum::{
    Json as JsonResponse, debug_handler,
    extract::{Json, Path, State},
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

pub async fn get_event_by_id(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> JsonResponse<GetEventResponse> {
    println!("Event id: {}", id);

    let event_state = app_state.events.lock().unwrap();

    let event = event_state
        .iter()
        .find(|e| e.id == Some(id.clone()))
        .cloned();

    println!("Event by Id: {:?}", event);

    if let Some(event) = event {
        return JsonResponse(GetEventResponse {
            message: String::from("Event retrieved successfully"),
            event: Some(event),
        });
    } else {
        return JsonResponse(GetEventResponse {
            message: "Event not found".to_string(),
            event: None,
        });
    }
}

#[debug_handler]
pub async fn update_event(
    State(app_state): State<Arc<AppState>>,
    Json(event_req): Json<UpdateEventRequest>,
) -> JsonResponse<EventResponse> {
    println!("Update event id: {}", event_req.id);

    let mut event_state = app_state.events.lock().unwrap();

    if let Some(event) = event_state
        .iter_mut()
        .find(|e| e.id.as_deref() == Some(&event_req.id))
    {
        if let Some(title) = event_req.title {
            event.title = title;
        }
        if let Some(description) = event_req.description {
            event.description = description;
        }
        if let Some(date) = event_req.date {
            event.date = date;
        }
        if let Some(location) = event_req.location {
            event.location = location;
        }
        if let Some(meet_url) = event_req.meet_url {
            event.meet_url = meet_url;
        }
        if let Some(organizer) = event_req.organizer {
            event.organizer = organizer;
        }

        println!("Updated event: {:?}", event);

        return JsonResponse(EventResponse {
            id: event.id.clone().unwrap_or_default(),
            message: String::from("Event updated successfully"),
        });
    } else {
        return JsonResponse(EventResponse {
            id: event_req.id,
            message: String::from("Event not found"),
        });
    }
}

pub async fn delete_event(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> JsonResponse<EventResponse> {
    let mut event_state = app_state.events.lock().unwrap();
    let event = event_state.iter().position(|e| e.id == Some(id.clone()));

    if let Some(pos) = event {
        let deleted_event = event_state.remove(pos);
        return JsonResponse(EventResponse {
            id: deleted_event.id.unwrap(),
            message: String::from("Event deleted successfully"),
        });
    } else {
        return JsonResponse(EventResponse {
            id: id,
            message: String::from("Event not found"),
        });
    }
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateEventRequest {
    id: String,
    title: Option<String>,
    description: Option<String>,
    date: Option<String>,
    location: Option<String>,
    meet_url: Option<String>,
    organizer: Option<String>,
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

#[derive(Serialize)]
pub struct GetEventResponse {
    message: String,
    event: Option<Event>,
}
