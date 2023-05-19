mod chess;
mod eval;
mod uci;
mod search;
mod perft;
mod table_transposition;
mod zobrist;
mod search_tools;
use uci::*;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    eprintln!("P'TIT BISCUIT");
    uci();
}
