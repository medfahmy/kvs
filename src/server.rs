use crate::{log, threadpool::ThreadPool, Action, Command, KvStore};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

fn handle_connection(mut stream: TcpStream, kvs: &mut KvStore) -> Result<(), String> {
    let mut buf = [0; 1024];

    let length = match stream.read(&mut buf) {
        Ok(n) => n,
        Err(err) => {
            log::error(&err);
            return Err(format!("error reading tcp stream to buffer: {:?}", err));
        }
    };

    let args: Vec<String> = String::from_utf8_lossy(&buf[0..length])
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
        Action::Read(value) => match value {
            Some(value) => {
                log::info(format!("value: {}", value));
                stream.write(value.as_bytes()).unwrap();
            }
            None => {
                log::warn("key is not stored");
            }
        },
        Action::Mutation => {
            log::info("successful operation");
        }
    };

    stream.flush().unwrap();

    Ok(())
}

pub fn run_server(port: usize) -> Result<(), String> {
    let kvs = KvStore::new();

    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(listener) => listener,
        Err(err) => {
            return Err(format!("error listening to port {}: {:?}", port, err));
        }
    };

    log::info(format!("kvs server listening on port {}", port));

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(err) => {
                return Err(format!("error reading tcp stream {:?}", err));
            }
        };

        // todo!("handle connections using thread pool");

        let mut kvs_ref = kvs.clone();

        match pool.execute(move || handle_connection(stream, &mut kvs_ref).unwrap()) {
            Ok(()) => {
                log::info(format!("arc count: {}", Arc::strong_count(&kvs.hashmap)));
                // should == 2
            }
            Err(err) => {
                log::error(format!("error handling connection by pool: {}", err));
                return Err(format!("error handling connection by pool: {}", err));
            }
        };

        // match handle_connection(stream, &mut kvs) {
        //     Ok(()) => {}
        //     Err(err) => {
        //         log::error(format!("error handling connection by pool: {}", err));
        //         return Err(format!("error handling connection by pool: {}", err));
        //     }
        // }
    }

    Ok(())
}
