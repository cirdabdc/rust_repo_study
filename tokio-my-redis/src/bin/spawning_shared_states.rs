use mini_redis::cmd::Set;
use tokio::process::Command;

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

    let mut db = std::collections::HashMap::new();

    let mut connection = mini_redis::Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap(){

        let response = match mini_redis::Command::from_frame(frame).unwrap(){
            mini_redis::Command::Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                mini_redis::Frame::Simple("OK".to_string())
            }
            mini_redis::Command::Get(cmd) => {
                if let Some(value) = db.get(cmd.key()){
                    mini_redis::Frame::Bulk(value.clone().into())
                } else {
                    mini_redis::Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd)
        };

        connection.write_frame(&response).await.unwrap();
    }
}