#[tokio::main]
async fn main(){

    let v = vec![1, 2, 3];

    let handle = tokio::spawn(async move {
        
        println!("{:?}", v);
    });
    handle.await.unwrap();
}

