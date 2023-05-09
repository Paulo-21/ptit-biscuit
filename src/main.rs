mod chess;
mod eval;
mod uci;
mod search;
use uci::*;
//use crate::{chess::*, table_transposition::TranspositionTable};
use std::env;
mod perft;
//use perft::*;
mod table_transposition;

//use std::time::Instant;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    eprintln!("P'TIT BISCUIT");
    uci();
    /*loop {
        //print!(">");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut split = buffer.split_ascii_whitespace();
        let _n = split.next().unwrap();
        match buffer.trim() {
            "uci" => {
                uci();
            },
            "perft" => {
                let game = Game::default();
                let mut i = 1;
                loop {
                    let now = Instant::now();
                    println!("Perft <{i}> : {} {} milliseconde", perft(game, i), now.elapsed().as_millis());
                    i+=1;
                }
            }
            "perfth" => {
                let game = Game::default();
                let mut i = 1;
                loop {
                    let now = Instant::now();
                    let hash_table = TranspositionTable::with_capacity(21000);
                    println!("Hash table : {}", hash_table.table.len());
                    println!("Perft <{i}> : {} {} milliseconde", perft_hash(game, &hash_table, i), now.elapsed().as_millis());
                    i+=1;
                }
            }
            _ => {
                println!("info {buffer}");
            }
        }

    }*/
}
