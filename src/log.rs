use std::fmt::Display;

pub fn info<T: Display>(data: T) {
    println!("[INFO] {}", data);
}

pub fn warn<T: Display>(data: T) {
    println!("[WARN] {}", data);
}

pub fn error<T: Display>(data: T) {
    println!("[ERROR] {}", data);
}