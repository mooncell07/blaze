mod core;
mod cpe;
use core::server::Server;

fn main() {
    let mut server = Server::new(
        core::server::BRAND.to_string(),
        "Blazing fast Minecraft Classic server in Rust!!!".to_string()
    );

    server.run()
}
