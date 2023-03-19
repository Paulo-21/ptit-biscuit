use std::io;
mod chess;
mod eval;
mod uci;
mod search;
use uci::*;
use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
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
