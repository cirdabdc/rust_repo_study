// #[tokio::main]
// async fn main(){

//     tokio::spawn(async{
//         {
//             let rc = std::rc::Rc::new("h1");
//             println!("{:?}", rc);
//         }
//         tokio::task::yield_now().await;
//     });
// }

#[tokio::main]
async fn main(){

    tokio::spawn(async{
        
        let rc = std::rc::Rc::new("h1");

        tokio::task::yield_now().await;

        println!("{:?}", rc);
    });
}