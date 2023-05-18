mod chess;
mod eval;
mod uci;
mod search;
mod perft;
mod table_transposition;
mod zobrist;
use bitintr::Tzcnt;
use uci::*;
use std::env;
use crate::eval::*;

use crate::chess::_draw_bitboard;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    eprintln!("P'TIT BISCUIT");
    uci();
}
