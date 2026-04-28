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

async fn get_user() -> &'static str {
    "Get User by id"
}

async fn serve_file() -> &'static str {
    "Serve file by path"
}

async fn not_found() -> &'static str {
    "Not found"
}

async fn update_user() -> &'static str {
    "Update user"
}

async fn delete_user() -> &'static str {
    "Delete user"
}

async fn health_route() -> &'static str {
    "Health Route"
}
#[tokio::main]
async fn main() {
    let user_routes = Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user));

    let health_routes = Router::new().route("/health", get(health_route));

    let merge_routes = Router::new().merge(user_routes).merge(health_routes);

    let app = Router::new()
        .nest("/v1/api/users", merge_routes)
        .route("/{*path}", get(not_found));

    // let g = || async { "Hellow, World!" };
    // let app = Router::new()
    //     .route("/", get(index))
    //     .route("/hello", get(hello))
    //     .route("/about", get(about))
    //     .route("/signup", post(signup))
    //     .route("/users", get(list_users).post(create_user))
    //     .route("/users/{id}", get(get_user))
    //     .route("/files/{*path}", get(serve_file))
    //     .route("/{*notfound}", get(not_found));

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




/user/1
*/
