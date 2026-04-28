use axum::{
    Router,
    routing::{delete, get, post},
};

async fn hello() -> String {
    return "hello world".to_string();
}
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hellow, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

/*

1. Routing
2. Handler
3. Extractors - Path, Query, Json, Form
4. Application State

5. Error Handling
6. Middleware with Tower
7. Database Integration - sqlx
8. Serving Static files
*/
