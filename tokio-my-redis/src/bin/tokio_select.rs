use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();


    tokio::spawn(async move {
        tx1.send("one").unwrap();
    });

    tokio::spawn(async move {
        tx2.send("two").unwrap();
    });

    tokio::select! {
        res1 = rx1 => {
            println!("rx1 {:?}", res1);
        }
        res2 = rx2 => {
            println!("rx2 {:?}", res2);
        }
    }
}
