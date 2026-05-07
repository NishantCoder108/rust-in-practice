use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    response::Html,
    routing::{delete, get, post},
};

// async fn hello() -> String {
//     return "hello world".to_string();
// }

// async fn index() -> &'static str {
//     "Home"
// }

// async fn about() -> &'static str {
//     "About"
// }

// async fn signup() -> &'static str {
//     "Signup"
// }

async fn list_users() -> &'static str {
    "list users"
}

async fn create_user() -> &'static str {
    "Create User"
}

// async fn get_user() -> &'static str {
//     "Get User by id"
// }

// async fn serve_file() -> &'static str {
//     "Serve file by path"
// }

async fn not_found() -> &'static str {
    "Not found"
}

async fn update_user(Path(id): Path<u64>) -> String {
    format!("Updated Users: {id}")
}

async fn delete_user() -> &'static str {
    "Delete user"
}

async fn health_route() -> &'static str {
    "Health Route"
}

async fn plain_text() -> &'static str {
    "Plain Text"
}

async fn no_content() -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn page() -> Html<&'static str> {
    Html(
        "<div> <h1> Hello Nishant</h1> <p> What's going on? Are you Rust backend Developer </p></div>",
    )
}

// async fn json_data() -> Json<serde_json::Value> {}

async fn get_user(Path(id): Path<u64>) -> String {
    format!("User #{id}")
}

// Multiple path params
async fn get_comment(Path((post_id, comment_id)): Path<(u64, u64)>) -> String {
    print!("pointer is comming here...");
    format!("Post {post_id}, Comment {comment_id}")
}

async fn get_health_post_route(Path())
#[tokio::main]
async fn main() {
    // let app = Router::new()
    //     .route("/", get(plain_text))
    //     .route("/no-content", get(no_content))
    //     .route("/page", get(page))
    //     .route("/{id}", get(get_user));
    let user_routes = Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user));

    let health_routes = Router::new().route("/health", get(health_route));
    let merge_routes = Router::new().merge(user_routes).merge(health_routes);

    let blog_routes = Router::new().route("/{post_id}/comment/{comment_id}", get(get_comment));

    let app = Router::new()
        .nest("/v1/api/users", merge_routes)
        .nest("/v1/api/posts", blog_routes)
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
