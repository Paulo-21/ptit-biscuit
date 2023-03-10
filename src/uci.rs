use std::{io, cmp::Ordering};
use crate::chess::*;
use crate::search::*;
pub fn uci () {
    println!("uciok");
    let mut game = Game::default();
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut response = String::new();
        let command = buffer.trim();

        if command == "uci"  {
            input_uci();
        }
        else if command == "isready" {
            input_ready();
        }
        else if command.len() >= 8 && command[..8].cmp("position") == Ordering::Equal {
            
            game = input_position(&command[ 9..]);
        }
        else if command.len() >= 2 && command[..2].cmp("go") == Ordering::Equal {
            //compute();

            let move_to_play = compute(&game, true);
            println!("{:?}", move_to_play);
        }
        else if command == "stop" {
            response.push_str("bestmove e2e4");
        }
    }
}
fn compute(game : &Game, _w_to_play : bool) -> (u64, u64) {
    let depth = 3;
    let maximizing_player = true;
    minimax(game, depth, maximizing_player);
    (2, 3)
}
fn input_uci() {
    println!("id name Ptit Biscuit\n");
    println!("id author Paul Cibier\n");
    println!("uciok");
}
fn input_ready() {
    println!("readyok");
}
fn input_position(mut commande : &str) -> Game {
    let game = if commande.contains("startpos") {
        commande = &commande[9..];
        get_bitboard_from_startpos(commande)
    }
    else {// if commande.contains("fen") {
        commande = &commande[3..];
        get_bitboard_from_startpos(commande)
        //get_bitboard_from_fen(commande)
    };
    game
}
fn get_bitboard_from_startpos(command : &str) -> Game {
    let move_tab = command.split_ascii_whitespace();
    let mut game = get_game_from_basicpos();
    let white_to_play = true;
    for one_move in move_tab {
        let (a,b) = convert_move_to_bitboard(one_move);
        if white_to_play {
            compute_move_w(a, b, &mut game);
        }
        else {
            compute_move_b(a, b, &mut game);
        }
        println!("{one_move}");
    }
    game
}

/*fn get_bitboard_from_fen(_command : &str) -> Game {
    
}*/