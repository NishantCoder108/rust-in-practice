use axum::{Json, Router, extract::State, routing::{get, post}};
use std::{error, net::SocketAddr};
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;



pub struct AppState {
    pub db: Mutex<HashMap<String, String>>,

}
async fn health_check() -> Json<String> {
    Json("OK".to_string())
}

pub async fn get_goals(State(app_state): State<Arc<AppState>>) -> Json<Vec<String>> {
    let db = app_state.db.lock().unwrap();
    let goals: Vec<String> = db.values().cloned().collect();
    Json(goals)
}

pub async fn create_goal(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateGoalRequest>,
) -> Json<CreateGoalResponse> {
    // validate_create_goal_request(&payload)?;
    // let payload = normalize_create_goal_request(payload);
    let goal_id = uuid::Uuid::new_v4();
    let goal_id_str = goal_id.to_string();
    //store at in-memory database
    // {
        let mut db = app_state.db.lock().unwrap();
        db.insert(goal_id_str.clone(), payload.goal);
    // }



    //send notification to message queue

    println!("Goal created: {}", goal_id);

    let goal = CreateGoalResponse {
        goal_id,
        status: "created".to_string(),
        message: "Goal created successfully".to_string(),
    };

    Json(goal)
}


#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app_state = Arc::new(AppState {
        db: Mutex::new(HashMap::new()),
    });
    let app = Router::new().route("/health", get(health_check))
    .route("/goal", post(create_goal))
    .route("/goals", get(get_goals))
    .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Vision system server listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateGoalRequest {
    pub goal: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateGoalResponse {
    pub goal_id: uuid::Uuid,
    pub status: String,
    pub message: String,
}
