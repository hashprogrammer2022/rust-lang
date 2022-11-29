use std::{io::Read, io::Write, net::TcpListener};

use crate::http::{ParseError, Request, Response, StatusCode};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, error: &ParseError) -> Response {
        println!("Failed to parse a request : {}", error);
        Response::new(StatusCode::NotFound, None)
    }
}

pub struct Server {
    addr: String,
    ip: String,
    port: i32,
}

impl Server {
    pub fn new(addrr: &str) -> Self {
        let (ip, port, clean_addr) =
            Self::parse_user_provided_addr(addrr).expect("Wrong address check format");
        Self {
            addr: clean_addr,
            ip,
            port,
        }
    }
    pub fn run(self, mut handler: impl Handler) {
        //the bind will listen to connection
        let listener = TcpListener::bind(&self.addr).unwrap();
        //when we are able to listen
        //display some welcome message
        println!(
            "The server is running on addr {} port {}\n{} Ok!",
            &self.ip, self.port, &self.addr
        );

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];

                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));

                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    //handle later
                                    handler.handle_request(&request)
                                }
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send a response : {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection : {}", e);
                    continue;
                }
            };
        }
    }

    fn parse_user_provided_addr(addr: &str) -> Option<(String, i32, String)> {
        let ip_port: Vec<&str> = addr.trim().split(":").collect();
        match ip_port.len() {
            2 => Some((
                ip_port[0].trim().to_string(),
                ip_port[1].trim().parse::<i32>().unwrap(),
                (ip_port[0].trim().to_string() + ":" + &ip_port[1].trim()),
            )),
            _ => None,
        }
    }
}
