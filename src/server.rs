use crate::{Action, Command, KvStore};
use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handle_connection<'a>(mut stream: TcpStream, kvs: &mut KvStore) -> Result<(), String> {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();

    let args: Vec<String> = String::from_utf8_lossy(&buf)
        .split(" ")
        .map(|s| s.to_string())
        .collect();

    println!("command args: {:?}", args);

    let cmd = match Command::new(args) {
        Ok(cmd) => cmd,
        Err(err) => {
            return Err(format!("error parsing arguments: {}", err));
        }
    };
    // .unwrap_or_else(|err| {
    //     eprintln!("error parsing arguments: {}", err);
        
    // });

    match kvs.exec_cmd(cmd) {
        Action::Read(value) => {
            match value {
                Some(value) => {
                    println!("value: {}", value);
                }
                None => {
                    println!("key is not stored");
                }
            }
        }
        Action::Mutation => {
            println!("successful operation");
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

    //     eprintln!(
    //         "error listening to port {}: {}.\n aborting server creation.",
    //         port, err
    //     );
    // });
    // let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(err) => {
                return Err(format!("error reading tcp stream {:?}.\n aborting server creation", err));
            }
        };
        //     eprintln!(
        //         "error reading tcp stream {}.\n aborting server creation.",
        //         err
        //     );
        // });
        // pool.execute(move || handle_connection(stream, &mut kvs));
        match handle_connection(stream, &mut kvs) {
            Ok(()) => {},
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(())
}
