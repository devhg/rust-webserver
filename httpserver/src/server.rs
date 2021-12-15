use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let lis = TcpListener::bind(self.socket_addr).unwrap();
        println!("Listening on {}", self.socket_addr);

        for conn in lis.incoming() {
            let mut conn = conn.unwrap();
            println!("New connection established: {}", conn.peer_addr().unwrap());
            let mut read_buf = [0; 512];
            conn.read(&mut read_buf).unwrap();
            let req: HttpRequest = String::from_utf8(read_buf.to_vec()).unwrap().into();

            println!("{:?} {:?} {:?}", req.method, req.version, req.resource);

            Router::route(req, &mut conn);
        }
    }
}
