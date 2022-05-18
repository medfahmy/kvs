use std::net::TcpStream;
use std::io::Write;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    // stream.write(&[1]).unwrap();

    let buffer = "get john".as_bytes();

    stream.write(&buffer).unwrap();

}