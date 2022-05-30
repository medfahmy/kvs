use kvs::log;
use kvs::server::run_server;
use std::process;

const PORT: usize = 7878;

fn main() {
    run_server(PORT).unwrap_or_else(|err| {
        log::error(format!("failed to run server: {}", err));
        process::exit(1);
    });
}
