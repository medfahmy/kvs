use std::fmt::Debug;

pub fn info<T: Debug>(data: T) {
    println!("[INFO] {:?}", data);
}

pub fn warn<T: Debug>(data: T) {
    println!("[WARN] {:?}", data);
}

pub fn error<T: Debug>(data: T) {
    println!("[ERROR] {:?}", data);
}