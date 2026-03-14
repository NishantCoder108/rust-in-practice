use axum::{
    Router,
    routing::{get, post},
};

mod user;
use user::register_user;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/v1/register", post(register_user));
    // .route("/v1/register", post(login_user))
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
