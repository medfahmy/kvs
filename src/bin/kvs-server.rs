use kvs::server::create_server;
use kvs::log;

fn main() {
    let port = 7878;
    create_server(port).unwrap_or_else(|err| log::error(err));
}