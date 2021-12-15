mod handler;
mod router;
mod server;

use server::Server;

fn main() {
    let s = Server::new("127.0.0.1:8000");
    s.run();
    println!("Hello, world!");
}
