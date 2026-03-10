use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _) = broadcast::channel::<String>(10);
    loop {
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let (mut socket, _address) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let (stream_reader, mut stream_writer) = socket.split();

            let mut message = String::new();
            let mut reader = BufReader::new(stream_reader);
            loop {
                tokio::select! {
                    result = reader.read_line(&mut message) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send(message.clone()).unwrap();
                        message.clear();

                    },
                    result = rx.recv()=>{
                        let received_message=result.unwrap();
                        stream_writer.write_all(received_message.as_bytes()).await.unwrap();
                    }
                }
            }
        });
    }
}
