use std::error::Error;

use sqlx::{Connection, Row, types::Text};

#[tokio::main]
// The `Box<dyn Error>` type allows the function to return any error type that implements the `Error` trait.
// This enables flexible error handling by boxing different kinds of errors into a single return type.
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://dbuser:mysecretpassword@localhost:5432/bookstore";
    // let mut conn = sqlx::postgres::PgConnection::connect(url).await?;
    let pool = sqlx::PgPool::connect(url).await?;

    let res = sqlx::query("SELECT 'Nishant' as name")
        .fetch_one(&pool)
        .await?;

    let name: String = res.get("name");
    println!("Name = {}", name);

    Ok(())
}

/*
1. Just setup bookstore app
2. connnect with database
3. Make sure function return Result
4. Run this command for running postgres at locally

  $ docker run -e POSTGRES_PASSWORD=mysecretpassword -e POSTGRES_USER=dbuser -e POSTGRES_DB=bookstore  -p 5432:5432 postgres
  database  username  password  Host   Port  dbname
  protocol://user:password@host:port/database

5. Run query to test connection db

6. Run command at terminal to see logs:
  `cargo run -q`

7.
*/
