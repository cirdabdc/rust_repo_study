use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut file = File::create("./src/bin/foo.txt").await?;
    // let n = file.write(b"hello world").await?;
    // println!("write success, length: {}", n);

    // file.write_all(b"some bytes2").await?;
    // Ok(())


    let mut reader:&[u8] = b"hello2";
    io::copy(&mut reader, &mut file).await?;
    Ok(())
}