use std::process;
use kvs::server::create_server;
use kvs::log;

const PORT: usize = 7878;

fn main() {
    create_server(PORT).unwrap_or_else(|err| {
        log::error(err);
        process::exit(1);
    });
}