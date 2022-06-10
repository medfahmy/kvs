use crate::log;
use std::process;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::Receiver;
use std::thread::{self, JoinHandle};

pub enum _ThreadPoolError {}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

enum WorkerKind {
    Reder,
    Writer,
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
    kind: WorkerKind, 
}

impl Worker {
    fn new(id: usize, receiver: Arc<RwLock<Receiver<Message>>>, kind: WorkerKind) -> Self {
        let thread = thread::spawn(move || loop {
            let message =  match kind { 
                Writer => receiver.write().unwrap().recv().unwrap(),
                Reader => receiver.read().unwrap().recv().unwrap(),
            };
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
            kind
        }
    }
}

pub struct ThreadPool {
    readers: Vec<Worker>,
    writers: Worker,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {}

    pub fn new(size: usize) -> Self {
        assert!(size > 0, "pool size must be greater than 0!");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), ));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), String>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        match self.sender.send(Message::NewJob(job)) {
            Ok(()) => Ok(()),
            Err(err) => {
                log::error(format!("error sending job to workers: {}", err));
                Err(format!("error sending job to workers: {}", err))
            }
        }
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
