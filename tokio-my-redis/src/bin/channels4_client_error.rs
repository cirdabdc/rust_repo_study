use mini_redis::{client,Result};

#[tokio::main]
async fn main(){
    // println!("Hello, world!");

    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    let t1 = tokio::spawn(async {
        client.set("hello1", "world1".into()).await;
    });

    let t2 = tokio::spawn(async {
        let res = client.get("hello1").await;
        println!("{:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
}

// huoyuyan@Mac-mini my-redis % cargo run --bin channels4_client_error
//    Compiling my-redis v0.1.0 (/Users/huoyuyan/source/rust_repo_study/tokio-my-redis)
// warning: unused import: `Result`
//  --> src/bin/channels4_client.rs:1:25
//   |
// 1 | use mini_redis::{client,Result};
//   |                         ^^^^^^
//   |
//   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

// error[E0373]: async block may outlive the current function, but it borrows `client`, which is owned by the current function
//   --> src/bin/channels4_client.rs:9:27
//    |
//  9 |     let t1 = tokio::spawn(async {
//    |                           ^^^^^ may outlive borrowed value `client`
// 10 |         client.set("hello1", "world1".into()).await;
//    |         ------ `client` is borrowed here
//    |
//    = note: async blocks are not executed immediately and must either take a reference or ownership of outside variables they use
// help: to force the async block to take ownership of `client` (and any other referenced variables), use the `move` keyword
//    |
//  9 |     let t1 = tokio::spawn(async move {
//    |                                 ++++

// error[E0499]: cannot borrow `client` as mutable more than once at a time
//   --> src/bin/channels4_client.rs:13:27
//    |
//  9 |       let t1 = tokio::spawn(async {
//    |                -            ----- first mutable borrow occurs here
//    |  ______________|
//    | |
// 10 | |         client.set("hello1", "world1".into()).await;
//    | |         ------ first borrow occurs due to use of `client` in coroutine
// 11 | |     });
//    | |______- argument requires that `client` is borrowed for `'static`
// 12 |
// 13 |       let t2 = tokio::spawn(async {
//    |                             ^^^^^ second mutable borrow occurs here
// 14 |           let res = client.get("hello1").await;
//    |                     ------ second borrow occurs due to use of `client` in coroutine

// Some errors have detailed explanations: E0373, E0499.
// For more information about an error, try `rustc --explain E0373`.
// warning: `my-redis` (bin "channels4_client") generated 1 warning
// error: could not compile `my-redis` (bin "channels4_client") due to 2 previous errors; 1 warning emitted
// huoyuyan@Mac-mini my-redis % 