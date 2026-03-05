use async_std::io::{ReadExt, WriteExt};
use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use async_std::task::spawn;
use futures::StreamExt;
use std::fs;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind(("127.0.0.1", 7878)).await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(None, |tcp_stream| async move {
            let tcp_stream = tcp_stream.unwrap();
            //handle_connection(tcp_stream).await;
            spawn(handle_connection(tcp_stream));
        })
        .await
}

async fn handle_connection(mut tcp_stream: TcpStream) {
    let mut buffer = [0; 1024];
    tcp_stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    tcp_stream.write(response.as_bytes()).await.unwrap();
    tcp_stream.flush().await.unwrap();
}
