use kvs::{server::create_server, KvStore};

fn main() {
    create_server(7878);
}