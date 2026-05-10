use std::time::Duration;

use anyhow::{Context, Ok};
use axum::{Router, routing::get, serve};
use dotenvy::{dotenv, var};
use sqlx::{Row, postgres::PgPoolOptions};

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

    // let que = sqlx::query("SELECT 'Nishant' as name")
    //     .fetch_one(&pool)
    //     .await?;

    // println!("Result query : {:?}", que.get::<String, _>("name"));

    let app = Router::new()
        .route("/", get(async || "hi nishant"))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("Failed to listen on port: 3000")?;
    serve(listener, app).await?;

    Ok(())
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


   docker run -e POSTGRES_PASSWORD=mypassword  -e POSTGRES_USER=taskuser -e POSTGRES_DB=taskstore  -p 5432:5432 postgres
   postgres://username:password/

   docker  exec -it  adoring_shaw  psql -U taskuser -P mypassword  -d taskstore

   docker exec -it tender_kepler psql -U taskuser -d taskstore


   Run : we are containerize from scratch
   Exec: that is already runnig , and we are trying to go inside (execute)

*/
