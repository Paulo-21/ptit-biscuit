use std::io;
mod chess;
mod eval;
mod uci;
mod search;
use uci::*;
use crate::chess::*;
use std::env;
mod perft;
use perft::*;
use std::time::Instant;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    println!("P'TIT BISCUIT");
    loop {
        //print!(">");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut split = buffer.split_ascii_whitespace();
        let n = split.next().unwrap();
        match buffer.trim() {
            "uci" => {
                uci();
            },
            "perft" => {
                let mut game = Game::default();
                //game.white_to_play ^= true;
                let mut i = 1;
                loop {
                    let now = Instant::now();
                    println!("Perft <{i}> : {} {} milliseconde", perft(game, i), now.elapsed().as_millis());
                    i+=1;
                }
            }
            _ => {
                println!("info {buffer}");
            }
        }

    }
}
