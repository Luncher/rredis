extern crate rredis;

use std::net::{TcpListener};
use rredis::resp;

struct Server {
  addr: String,
  listener: TcpListener,
}

impl Server {
    fn new(addr: &str) -> Server {
      Server {
        addr: String::from(addr),
        listener: TcpListener::bind(addr).unwrap()
      }
    }

    fn run(&self) {
      println!("rredis listen on: {}", self.addr);
      for stream in self.listener.incoming() {
        match stream {
          Ok(stream) => {
            let mut resp = resp::parse(stream);
          },
          Err(e) => {
            println!("listner error: {:?}", e);
          }
        }
      }
    }
 }

 fn main() {
   let server = Server::new("127.0.0.1:6379");
   server.run();
 }