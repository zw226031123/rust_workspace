use futures::channel::mpsc;
use futures::executor::block_on;
use futures::{SinkExt, Stream, StreamExt, TryStreamExt};
use std::io;
use std::pin::Pin;

fn main() {
    let future = hello_world();
    block_on(future);

    let f = send_recv();
    block_on(f);
}

async fn hello_world() {
    println!("Hello, world!");
}

async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    assert_eq!(rx.next().await, Some(1));
    assert_eq!(rx.next().await, Some(2));
    assert_eq!(rx.next().await, None);
}
#[allow(dead_code)]
async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }
    sum
}
#[allow(dead_code)]
async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
) -> Result<i32, io::Error> {
    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }
    Ok(sum)
}
