use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    //Create route
    let router01 = Router::new().route("/", get(vehicle_get));

    //ip address  and listen the port
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    //serve the port and runnning the application
    axum::serve(listener, router01).await.unwrap();
}

async fn vehicle_get() {}
