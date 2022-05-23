use crate::{Action, Command, KvStore, log};
use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handle_connection<'a>(mut stream: TcpStream, kvs: &mut KvStore) -> Result<(), String> {
    let mut buf = [0; 1024];

    let n = match stream.read(&mut buf) {
        Ok(n) => n,
        Err(err) => {
            log::error(&err);
            return Err(format!("error reading tcp stream to buffer: {:?}", err));
        }
    };

    let args: Vec<String> = String::from_utf8_lossy(&buf[0..n])
        .trim()
        .split(" ")
        .map(|s| s.to_string())
        .collect();

    log::info(format!("received command: {:?}", args));

    let cmd = match Command::new(args) {
        Ok(cmd) => cmd,
        Err(err) => {
            return Err(format!("error parsing arguments: {}", err));
        }
    };

    match kvs.exec_cmd(cmd) {
        Action::Read(value) => {
            match value {
                Some(value) => {
                    log::info(format!("value: {}", value));
                }
                None => {
                    log::warn("key is not stored");
                }
            }
        }
        Action::Mutation => {
            log::info("successful operation");
        }
    };

    Ok(())
}

pub fn create_server(port: usize) -> Result<(), String> {
    let mut kvs = KvStore::new();
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(listener) => listener,
        Err(err) => {
            return Err(format!("error listening to port {}: {:?}.\n aborting server creation", port, err));
        }
    };

    log::info(format!("kvs server listening on port {}", port));

    // let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(err) => {
                return Err(format!("error reading tcp stream {:?}.\n aborting server creation", err));
            }
        };

        match handle_connection(stream, &mut kvs) {
            Ok(()) => {},
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(())
}
