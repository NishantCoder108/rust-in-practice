use axum::{
    Router,
    routing::{delete, get, post},
};

async fn hello() -> String {
    return "hello world".to_string();
}

async fn index() -> &'static str {
    "Home"
}

async fn about() -> &'static str {
    "About"
}

async fn signup() -> &'static str {
    "Signup"
}

async fn list_users() -> &'static str {
    "list users"
}

async fn create_user() -> &'static str {
    "Create User"
}
#[tokio::main]
async fn main() {
    // let g = || async { "Hellow, World!" };
    let app = Router::new()
        .route("/", get(index))
        .route("/hello", get(hello))
        .route("/about", get(about))
        .route("/signup", post(signup))
        .route("/users", get(list_users).post(create_user));

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
