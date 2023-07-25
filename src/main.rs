use std::thread;
use std::time::Duration;
use crate::cat::cat::Cat;

mod cat;
mod message;
mod utils;

#[async_std::main]
async fn main() {
    let cat = Cat::new();


    let trans = cat.new_transaction("".to_string(), "".to_string());
    trans.complete().await;

    thread::sleep(Duration::from_secs(10))
}
