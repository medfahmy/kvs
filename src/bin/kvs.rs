use std::env;
use std::net::TcpStream;
use std::io::Write;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let args = env::args();
    let args_str: Vec<String> = args.collect();
    let str = args_str[1..].join(" ");

    let buffer = str.as_bytes();

    stream.write(&buffer).unwrap();
}