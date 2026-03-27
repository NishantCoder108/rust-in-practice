use redis::Commands;

fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let _: () = con.lpush("jobs", "send_email")?;
    println!("Job added");

    Ok(())
}