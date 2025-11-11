use mini_redis::{client,Result};

#[tokio::main]
async fn main() -> Result<()> {
    // println!("Hello, world!");

    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello1", "world1".into()).await?;
    let res = client.get("hello1").await?;
    println!("{:?}", res);
    Ok(())
}
