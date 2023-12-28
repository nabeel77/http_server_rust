use std::net::TcpListener;
use std::io::Read;
use std::convert::TryFrom;
use crate::http::{Request, request};
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        // Self is same like returning Server
        Self {
            address
        }
    }

    pub fn run(self) {
        println!("Server is listening on {} ", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();
        // infinite loop syntax
        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {:?}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse request: {}", e)   
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
        }
    }
}
