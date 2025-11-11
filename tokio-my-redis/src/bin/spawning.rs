#[tokio::main]
async fn main(){
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop{
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move{
            process(socket).await;
        });
        
        // process(socket).await;
    }
}


async fn process(socket: tokio::net::TcpStream){

    let mut connection = mini_redis::Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap(){
        println!("GOT: {:?}", frame);

        let response = mini_redis::Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}