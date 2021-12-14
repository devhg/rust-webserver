use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let lis = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("establishing connection on port 8000");

    for stream in lis.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
    println!("Hello, world!");
}
