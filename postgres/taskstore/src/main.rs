use std::time::Duration;

use anyhow::{Context, Ok};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    serve,
};
use dotenvy::{dotenv, var};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row, postgres::PgPoolOptions};
use tokio::io::Join;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let url = var("DATABASE_URL").context("Database url must be set.")?;

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&url)
        .await
        .context("Failed to connect database")?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Migration is not working.")?;

    // let que = sqlx::query("SELECT 'Nishant' as name")
    //     .fetch_one(&pool)
    //     .await?;

    // println!("Result query : {:?}", que.get::<String, _>("name"));

    let app = Router::new()
        .route("/", get(async || "hi nishant"))
        .route("/task", post(create_task))
        .route("/task", get(read_task))
        .route("/task", put(update_task))
        .route("/task/{id}", delete(delete_task))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("Failed to listen on port: 3000")?;

    serve(listener, app).await?;

    Ok(())
}

async fn create_task(
    State(pool): State<PgPool>,
    Json(data): Json<TaskRequestBody>,
) -> (StatusCode, Json<TaskResponseBody>) {
    let task = sqlx::query("INSERT INTO task (title) VALUES ($1) RETURNING id")
        .bind(&data.title)
        .fetch_one(&pool)
        .await
        // .context("Failed to create task")
        .unwrap();

    let task_id = task.get::<i32, _>("id");
    println!("Task : {}", task_id);

    (
        StatusCode::CREATED,
        Json(TaskResponseBody {
            message: "Task has been created successfully".to_string(),
            id: task_id,
        }),
    )
}

async fn read_task(State(pool): State<PgPool>) -> Json<TaskReadResponse> {
    let tasks = sqlx::query("SELECT * FROM task")
        .fetch_all(&pool)
        .await
        .context("Failed to read tasks")
        .unwrap();

    println!("Tasks : {:?}", tasks);

    let tasks = tasks
        .into_iter()
        .map(|row| TaskState {
            title: row.get::<String, _>("title"),
            id: row.get::<i32, _>("id"),
        })
        .collect();

    Json(TaskReadResponse {
        message: "Fetched tasks successfully".to_string(),
        tasks: tasks,
    })
}

async fn update_task(
    State(pool): State<PgPool>,
    Json(data): Json<TaskUpdateRequestBody>,
) -> Json<TaskResponseBody> {
    let task = sqlx::query("UPDATE task SET title = $1 WHERE id = $2  RETURNING id")
        .bind(&data.title)
        .bind(&data.id)
        .fetch_one(&pool)
        .await
        .context("Failed to update task")
        .unwrap();

    Json(TaskResponseBody {
        message: "Task updated successfully.".to_string(),
        id: task.get::<i32, _>("id"),
    })
}

async fn delete_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<TaskResponseBody>) {
    let task = sqlx::query("SELECT * FROM task where id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .context("No task found")
        .unwrap();

    if task.is_empty() {
        return (
            StatusCode::NOT_FOUND,
            Json(TaskResponseBody {
                message: "Task doesn't found".to_string(),
                id: id,
            }),
        );
    } else {
        sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(id)
            .execute(&pool)
            .await
            .context("Failed to delete task")
            .unwrap();

        (
            StatusCode::OK,
            Json(TaskResponseBody {
                message: "Deleted task successfully.".to_string(),
                id: id,
            }),
        )
    }
}
// async fn delete_task(State(pool): State<PgPool>, Path(id): Path<u32>) -> StatusCode {
//     let task = sqlx::query("SELECT * FROM task where id = $1")
//     .bind(&id)
//     .await

//     StatusCode::NOT_FOUND
// }

#[derive(Deserialize, Serialize)]
struct TaskRequestBody {
    title: String,
}

#[derive(Deserialize, Serialize)]
struct TaskResponseBody {
    message: String,
    id: i32,
}

#[derive(Deserialize, Serialize)]
struct TaskUpdateRequestBody {
    title: String,
    id: i32,
}

#[derive(Deserialize, Serialize)]
struct TaskState {
    title: String,
    id: i32,
}

#[derive(Deserialize, Serialize)]
struct TaskReadResponse {
    message: String,
    tasks: Vec<TaskState>,
}

/*

1. Setup databse - connect db
2. test or writing something or might i can go inside and check working fine
3. start to create struct
   1. define struct
   2. write routes for create
   3. read
   4. update
   5. delete


   docker run -e POSTGRES_PASSWORD=mypassword  -e POSTGRES_USER=taskuser -e POSTGRES_DB=taskstore  -p 5432:5432 postgres -d
   postgres://username:password/

   docker  exec -it  adoring_shaw  psql -U taskuser -P mypassword  -d taskstore

   docker exec -it tender_kepler psql -U taskuser -d taskstore


   Run : we are containerize from scratch
   Exec: that is already runnig , and we are trying to go inside (execute)

   ---
   for creating sql table or query stuff:
   `sqlx migrate add task_table`

*/
