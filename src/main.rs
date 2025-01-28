use std::io;
mod chat;
use chat::init_chat_loop as init_chat_loop;

fn main() {
    println!("Enter your name:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim().to_string();

    init_chat_loop(name);
}
