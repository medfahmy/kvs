use std::env;
use std::collections::HashMap;

mod server;
mod utils;
mod tests;

// kvs set john doe
// kvs get john => doe
// kvs rm john

pub fn run_cli(args: env::Args) {
    let cmd = Command::new(args).unwrap_or_else(|err| panic!("error parsing arguments {}", err));
    // send cmd to server
}

pub fn init_store() {
    let kvs = KvStore::new();
    // run server
}


enum Command {
    Get(String),
    Set(String, String),
    Rm(String)
}

//        [ 0,     1,   2,   3     ]
// args = [target, cmd, key, value?]

impl Command {
    pub fn new(args: env::Args) -> Result<Command, &'static str> {
        let args: Vec<String> = args.collect();
        // TODO: log cmds to run
        match args[1].to_lowercase().as_str() {
            "get" => {
                if args.len() != 3 {
                    return Err("get command: invalid arguments");
                }
                Ok(Command::Get(args[2].clone()))
            }
            "set" => {
                if args.len() != 4 {
                    return Err("set command: invalid arguments");
                }
                Ok(Command::Set(args[2].clone(), args[3].clone()))
            }
            "rm" => {
                if args.len() != 3 {
                    return Err("rm command: invalid arguments");
                }
                Ok(Command::Rm(args[2].clone()))
            }
            _ => {
                return Err("invalid command");
            }
        }
    }
}

enum Action {
    Read(Option<String>),
    Mutation(())
}

pub struct KvStore {
    hashmap: HashMap<String, String>,
}

impl KvStore {
    fn new() -> Self {
        KvStore { hashmap: HashMap::new() }
    }

    fn get(&self, key: String) -> Option<String> {
        self.hashmap.get(&key).cloned()
    }

    fn set(&mut self, key: String, value: String) {
        self.hashmap.insert(key, value);
    }

    fn remove(&mut self, key: String) {
        self.hashmap.remove(&key);
    }

    fn exec_cmd(&mut self, cmd: Command) -> Action {
        match cmd {
            Command::Get(key) => {
                Action::Read(self.get(key))
            }
            Command::Set(key, value) => {
                Action::Mutation(self.set(key, value))
            }
            Command::Rm(key) => {
                Action::Mutation(self.remove(key))
            }
        }
    }
}