mod core;
use core::server::Server;

fn main(){
    let mut server = Server::new();
    server.run()
}