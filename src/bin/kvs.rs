use kvs::log;
use std::io::Write;

fn main() {
    let mut stream = std::net::TcpStream::connect("127.0.0.1:7878").unwrap();

    let cmd = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    log::info(format!("sent command: {}", cmd));

    let buf = cmd.as_bytes();
    stream.write_all(buf).unwrap();
}