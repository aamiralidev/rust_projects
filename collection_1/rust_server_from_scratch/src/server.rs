use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
// use super::http::Request;
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::convert::TryInto;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e:ParseError) -> Response{
        println!("failed to parse request {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server{
    addr:String,
}

impl Server{
    pub fn new(addr:String) -> Server{
        Server{addr}
    }
    pub fn run(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("listening on {}", self.addr);
        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Recieved a request {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(req) => {
                                    // dbg!(req);
                                    // Response::new(StatusCode::Ok,
                                    //               Some("<h1>It works</h1>".to_string()))
                                    handler.handle_request(&req)
                                }
                                Err(e) => {
                                    handler.handle_bad_request(e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream){
                                println!("Failed to parse request {}", e);
                            }
                            // let res: Result<Request, _> = buffer.try_into();
                        }
                        Err(e) => {println!("Failed to read connection {}", e)}
                    }
                } Err(e) => {
                    println!("some error occured. {}", e);
                }
            }
        }

    }
}