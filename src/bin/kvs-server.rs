use std::process;
use kvs::server::run_server;
use kvs::log;

const PORT: usize = 7878;

fn main() {
    run_server(PORT).unwrap_or_else(|err| {
        log::error(err);
        process::exit(1);
    });
}