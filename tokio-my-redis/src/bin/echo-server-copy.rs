use tokio::io;
use tokio::net::TcpListener;
use mini_redis::Connection;
use mini_redis::Frame;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6666").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (mut rd,mut wr) = socket.split();
            if let Err(e) = io::copy(&mut rd, &mut wr).await {
                println!("copy error: {:?}", e);
            }
        });
    }
    Ok(())
}