use mini_redis::{client,Result};
use tokio::sync::mpsc;
use bytes::Bytes;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: Bytes,
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {        
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(msg) = rx.recv().await {
            println!("msg: {:?}", msg);
            match msg {
                Command::Get { key } => {
                    let res = client.get(&key).await.unwrap();
                    println!("get:{:?}", res);
                }
                Command::Set { key, val } => {
                    let res = client.set(&key, val).await.unwrap();
                }
            }
        }
    });


    let t1 = tokio::spawn(async move {
        tx.send(Command::Set {
            key: "hello".to_string(),
            val: "world".into(),
        }).await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        tx2.send(Command::Get {
            key: "hello".to_string(),
        }).await.unwrap();
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}