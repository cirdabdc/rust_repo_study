#[tokio::main]
async fn main(){
    let handle = tokio::spawn(async {
        "return value"
    });

    let res = handle.await.unwrap();
    println!("{:?}", res);
}

