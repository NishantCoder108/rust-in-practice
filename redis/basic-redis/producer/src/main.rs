// use redis::Commands;
use redis::AsyncCommands;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;
    let mut con = client.get_multiplexed_async_connection().await?;

    let _: () = con.lpush("jobs", "send_email2").await?;
    println!("Job added");

    Ok(())
}