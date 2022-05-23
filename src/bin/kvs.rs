use std::env;
use std::net::TcpStream;
use std::io::Write;

use kvs::log;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let mut args = env::args();
    args.next();
    let cmd = args.collect::<Vec<String>>().join(" ");
    log::info(format!("sent command: {}", cmd));

    let buf = cmd.as_bytes();
    stream.write(buf).unwrap();
}