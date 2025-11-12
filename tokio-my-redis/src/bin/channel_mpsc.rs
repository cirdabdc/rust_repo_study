use bytes::Bytes;

#[derive(Debug)]
enum Command {
    Set{
        key: String,
        val: Bytes,
    },
    Get{
        key: String,
    }
}


use tokio::sync::mpsc;

#[tokio::main]
async fn main(){
    //create a new channel with a capacity of at most 32
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await.unwrap();
    });

    while let Some(msg) = rx.recv().await {
        println!("Got message {:?}", msg);
    }


}