pub mod log;
pub mod server;
mod tests;
pub mod utils;

use std::collections::HashMap;

// kvs set john doe
// kvs get john => doe
// kvs rm john

pub enum Command {
    Get(String),
    Set(String, String),
    Rm(String),
}

//        [ 0,     1,   2,   3     ]
// args = [target, cmd, key, value?]

impl Command {
    pub fn new(args: Vec<String>) -> Result<Command, String> {
        // TODO: log cmds to run in a file
        match args[0].to_lowercase().as_str() {
            "get" => {
                if args.len() != 2 {
                    return Err("get command: invalid arguments".to_string());
                }
                Ok(Command::Get(args[1].clone()))
            }
            "set" => {
                if args.len() != 3 {
                    return Err("set command: invalid arguments".to_string());
                }
                Ok(Command::Set(args[1].clone(), args[2].clone()))
            }
            "rm" => {
                if args.len() != 2 {
                    return Err("rm command: invalid arguments".to_string());
                }
                Ok(Command::Rm(args[1].clone()))
            }
            cmd => {
                return Err(format!("invalid command: {}", cmd));
            }
        }
    }
}

pub enum Action {
    Read(Option<String>),
    Mutation,
}

pub struct KvStore {
    hashmap: HashMap<String, String>,
}

impl KvStore {
    fn new() -> Self {
        KvStore {
            hashmap: HashMap::new(),
        }
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
            Command::Get(key) => Action::Read(self.get(key)),
            Command::Set(key, value) => {
                self.set(key, value);
                Action::Mutation
            }
            Command::Rm(key) => {
                self.remove(key);
                Action::Mutation
            }
        }
    }
}
