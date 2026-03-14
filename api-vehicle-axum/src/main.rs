use axum::{
    Router,
    routing::{get, post},
};

mod vehicle;

#[tokio::main]
async fn main() {
    //Create route
    let router01 = Router::new()
        .route("/", get(vehicle::vehicle_get))
        .route("/", post(vehicle::vehicle_post));

    //ip address  and listen the port
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    //serve the port and runnning the application
    axum::serve(listener, router01).await.unwrap();
}
