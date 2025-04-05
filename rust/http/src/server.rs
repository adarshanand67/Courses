use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    pub ip_address: String,
    pub port: u32,
}

impl Server {
    pub fn new(ip_address: String, port: u32) -> Self {
        Server { ip_address, port }
    }

    pub fn run(&mut self) {
        println!("Server is running on {}:{}", self.ip_address, self.port);
        let address = format!("{}:{}", self.ip_address, self.port);

        let listener = TcpListener::bind(address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("Connection received from: {}", addr);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    println!("Request parsed successfully: {:?}", request);
                                }
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }
        }
    }
}
