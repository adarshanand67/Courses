#![allow(dead_code)]
#![allow(unused_variables)]

mod http;
mod server;

use http::Method;
use http::Request;
use server::Server;

fn main() {
    let ip_address: String = "127.0.0.1".to_string();
    let port: u32 = 8080;
    let mut server: Server = Server::new(ip_address, port);

    println!(
        "Server is starting... on {}:{}",
        server.ip_address, server.port
    );

    server.run();
    println!("Exiting server...");
}
