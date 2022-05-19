use std::net::{TcpListener, TcpStream};
use std::process;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::io::Read;
use crate::{Command, KvStore, Action};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("worker {} is executing job.", id);
                    job();
                }
                Message::Terminate => {
                    println!("worker {} is terminated.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {}

    fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .send(Message::NewJob(job))
            .unwrap_or_else(|err| {
                eprintln!("error sending job to workers: {}", err);
                process::exit(1);
            });
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("sending terminate message to all workers");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("shutting down all workers");

        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap_or_else(|err| {
                    eprintln!(
                        "error shutting down worker {}: {:?}.\n exiting process.",
                        worker.id, err
                    );
                    process::exit(1);
                });
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, kvs: &mut KvStore) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let cmd_str = String::from_utf8_lossy(&buffer);
    let args: Vec<&str> = cmd_str.split(" ").collect();
    let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    
    let cmd = Command::new(args).unwrap_or_else(|err| {
        eprintln!("error parsing arguments: {}", err);
        process::exit(1);
    });

    match kvs.exec_cmd(cmd) {
        Action::Read(value) => {
            if let Some(value) = value {
                println!("{}", value);
            }
        }
        Action::Mutation => {
            println!("successful operation");
        }
    }
}

pub fn create_server(port: usize) {
    let mut kvs = KvStore::new();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap_or_else(|err| {
        eprintln!(
            "error listening to port {}: {}.\n aborting server creation.",
            port, err
        );
        process::exit(1);
    });
    // let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap_or_else(|err| {
            eprintln!(
                "error reading tcp stream {}.\n aborting server creation.",
                err
            );
            process::exit(1);
        });
        // pool.execute(move || handle_connection(stream, &mut kvs));
        handle_connection(stream, &mut kvs);
    }
}
