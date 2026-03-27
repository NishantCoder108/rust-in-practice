use redis::Commands;
// use redis::AsyncCommands;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // let mut con = client.get_multiplexed_async_connection().await?;
    // let mut con = client.get_connection().await?;

    loop {
        // let res: (String, String) = redis::cmd("BRPOP").arg("jobs").arg(0).query(&mut con)?;

        // println!("Hello from Worker");
        // println!("Processing job: {:?}", res.1);

        let res: (String, String) = con.brpop("jobs", 0.0)?;
        // let res:(String, String) = con.brpop("jobs", 0.0).await?;
        println!("Hello from Worker");
        println!("Processing job: {:?}", res);

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
