use std::fmt::Display;

pub fn info(data: impl Display) {
    println!("[INFO] {}", data);
}

pub fn warn(data: impl Display) {
    println!("[WARN] {}", data);
}

pub fn error(data: impl Display) {
    println!("[ERROR] {}", data);
}