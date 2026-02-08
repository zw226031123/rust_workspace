use futures::executor::block_on;

fn main() {
    let future = hello_world();
    block_on(future);
}

async fn hello_world() {
    println!("Hello, world!");
}
