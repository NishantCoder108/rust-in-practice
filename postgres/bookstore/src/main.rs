use std::error::Error;

use sqlx::{Connection, Row, types::Text};

struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

#[tokio::main]
// The `Box<dyn Error>` type allows the function to return any error type that implements the `Error` trait.
// This enables flexible error handling by boxing different kinds of errors into a single return type.
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://dbuser:mysecretpassword@localhost:5432/bookstore";
    // let mut conn = sqlx::postgres::PgConnection::connect(url).await?;
    let pool = sqlx::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?; //add migration to to do

    let book = Book {
        title: "Atomic Habits".to_string(),
        author: "James clear".to_string(),
        isbn: "978-1847941831".to_string(),
    };

    // create(&book, &pool).await?;
    // let res = sqlx::query("SELECT 'Nishant' as name")
    //     .fetch_one(&pool)
    //     .await?;

    // let name: String = res.get("name");

    let rows = sqlx::query("SELECT * FROM book").fetch_all(&pool).await?;

    println!("{:?}", rows);
    // println!("Name = {}", name);

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


---- Migrations ------

7. We install`cargo install  sqlx-cli` for migrations
8. Run command for query : ` sqlx migrate add books_table`
9. Go to sql file and write own migratin script
10. Add migration at main file , with db connection
11. two things, if we have already add in main file or we run sqlx run at external
12. So, after all done and everything fine


----Interact with Table -----

13. We are running sql in docker form so we need to go inside to see what is happening
  1. docker ps
  2. now enter the container
  `docker exec -it awesome_poincare psql -U dbuser  -d bookstore`

  -U -> user name of this connection in docker
  -d -> database

  3. so now, we are inside db
  4. Type command `\dt` to list all tables in the current database along with their schema, name, type, and owner.
    \dt or \d  -> Display tables
    \l  -> list
    \du  -> display users
    \q -> quit

  5. So, if we only changes inside migration file and if we run `cargo run ` or `cargo build` , it will do nothing
    so, we can create a new build script file, for every time we call or run , this script will be executed
    https://docs.rs/sqlx/latest/sqlx/macro.migrate.html

   Run this command  ->  `sqlx migrate build-script`
   It created build.rs file, it will run for every changes


----- SQLx Data writing -----
   1. Create struct and query the table
   2. To see, we go inside sql and run command to see all data
   `SELECT * FROM book;`


*/
