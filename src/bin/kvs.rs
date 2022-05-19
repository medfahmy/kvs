use std::env;
use std::net::TcpStream;
use std::io::Write;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let args= env::args().collect::<Vec<String>>().join(" ");
    let buf = args.as_bytes();

    stream.write(buf).unwrap();
}