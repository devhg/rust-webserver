use std::str;
use std::{
    io::{Read, Write},
    net::TcpStream,
};
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();
    println!("recv from server: {:?}", str::from_utf8(&buffer).unwrap());
}
