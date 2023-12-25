use server::Server;
use http::Request;
use http::Method;

mod http;
mod server; // importing server module from src/server.rs
fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

