use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn typewrite(s: &str) {
    for c in s.chars() {
        print!("{}", c);
        io::stdout().flush().expect("error flushing stdout");
        thread::sleep(Duration::from_millis(100));
    }
}