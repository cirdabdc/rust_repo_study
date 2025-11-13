use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut file = File::open("./src/bin/hello.txt").await?;
    // // 方式1
    // let mut buffer = [0; 10];
    // let bytes_read = file.read(&mut buffer).await?;
    // println!("Read {} bytes: {:?}", bytes_read, &buffer[..bytes_read]);

    方式2
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    println!("Read {} bytes: {:?}", buffer.len(), &buffer);
    
    Ok(())
}