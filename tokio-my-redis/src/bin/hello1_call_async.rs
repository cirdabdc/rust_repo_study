//调用say_hello函数 只是得到一个future,在调用await后才会执行say_hello函数中的代码

async fn say_hello(){
    println!("hello world");
}

#[tokio::main]
async fn main() {
    let op = say_hello();

    println!("hello");

    op.await;
}