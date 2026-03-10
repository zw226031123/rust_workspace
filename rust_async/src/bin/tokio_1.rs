use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    //::<(String,SocketAddr)>
    let (tx, _) = broadcast::channel(10);
    let token = CancellationToken::new();
    let cancel_token = token.clone();
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                tracing::info!("shutdown task");
                cancel_token.cancel();
            }
            Err(err) => {
                tracing::info!("Error while listening on CTRL-C: {err:#?}");
            }
        }
    });
    loop {
        let tx = tx.clone();
        let token = token.clone();
        let mut rx = tx.subscribe();
        let (mut socket, address) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            tracing::info!("spawn new task");
            let (stream_reader, mut stream_writer) = socket.split();

            let mut message = String::new();
            let mut reader = BufReader::new(stream_reader);
            loop {
                tokio::select! {
                    result = reader.read_line(&mut message) => {
                        tracing::info!("received message from client {}", &message);
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((message.clone(),address)).unwrap();
                        message.clear();

                    },
                    result = rx.recv()=>{
                        let (received_message,sender_address)=result.unwrap();
                        if sender_address!=address {
                            tracing::info!("received message from  channel: {received_message}");
                            stream_writer.write_all(received_message.as_bytes()).await.unwrap();
                        }
                    }
                    _=token.cancelled() => {
                        tracing::info!("cancelled");
                        return;
                    }
                }
            }
        });
    }
}
