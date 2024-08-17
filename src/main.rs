mod core;
use core::server::Server;

fn main() {
    let mut server = Server::new("Blaze".to_string(), "Blazing fast classic server in rust!!!".to_string());
    server.run()
}
