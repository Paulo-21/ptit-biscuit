mod chess;
mod eval;
mod uci;
mod search;
mod perft;
mod table_transposition;
mod zobrist;
mod search_tools;
use perft::perft_divide;
use uci::*;
use std::env;
use crate::chess::*;

fn main() {
    if env::args().len() > 1 {
        
        let mut depth = 64i8;
        let mut game  = Game::default();
        for (i, argument) in env::args().into_iter().enumerate() {
            
            match i {
                1 => depth = argument.parse::<i8>().unwrap(),
                2 => {
                    let mut arr = vec!["",""];
                    let mut fen: Vec<&str> = argument.trim().split_ascii_whitespace().collect();
                    arr.append(&mut fen);
                    game = get_bitboard_from_fen(arr);
                },
                3 => {

                },
                _ => {}
            }
        }

        perft_divide(game, depth);
    }
    else {
        //env::set_var("RUST_BACKTRACE", "1");
        eprintln!("P'TIT BISCUIT");
        uci();
    }
}
