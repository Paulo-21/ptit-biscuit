use std::io;
mod chess;
mod uci;
mod search;
use uci::*;

fn main() {
    println!("P'TIT BISCUIT");
    loop {
        //print!(">");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        match buffer.trim() {
            "uci" => {
                uci();
            },
            _ => {
                println!("info {buffer}");
            }
        }

    }
}
