use axum::{Router, routing::get, serve};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(async || "hi nishant"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
