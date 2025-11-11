//#[tokio::main] 相当于包装了一个runtime,在runtime中执行异步代码

async fn say_hello(){
    println!("hello world");
}

// #[tokio::main]
fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async{
        let op = say_hello();
        op.await;
    });
    // let op = say_hello();

    // println!("hello");

    // op.await;
}