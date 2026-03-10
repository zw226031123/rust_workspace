use tokio::runtime;
use tokio::task::JoinSet;

// fn main() {
//     let rt = runtime::Builder::new_current_thread()
//         .enable_all()
//         .build()
//         .unwrap();
//     rt.block_on(hi());
// }
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // hi().await;
    let mut set=JoinSet::new();
    for i in 0..10 {
        set.spawn(add(i,2));
    }
    while let Some(x) = set.join_next().await {
        println!("{:?}", x);
    }





}
async fn add(a: i32, b:i32) -> i32 {
    a+b
}
async fn hi() {
    println!("hello tokio");
}
