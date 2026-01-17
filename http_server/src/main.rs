mod handler;
mod router;
mod server;
use crate::server::Server;

fn main() {
    let server = Server::new("localhost:3000");
    server.run();
}
