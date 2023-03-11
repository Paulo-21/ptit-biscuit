use std::{io, cmp::{Ordering, max, min}};
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
            let (a, b) = compute(&game);
            let bestmovea = convert_square_to_move(a);
            let bestmoveb = convert_square_to_move(b);
            println!("bestmove {}{}", bestmovea, bestmoveb);
        }
        else if command == "stop" {
            response.push_str("bestmove e2e4");
        }
    }
}
fn compute(game : &Game) -> (u64, u64) {
    let depth = 3;
    let maximizing_player = true;
    //draw_board(game);
    let legal_moves = get_legal_move(game.white_to_play, game);
    println!("info : {:?}", legal_moves);
    let mut score = i16::MIN;
    let mut bestmove = 0u64;
    for moveto in legal_moves {
        let mut game1 = game.clone();
        game1.white_to_play ^= true;
        let a = moveto.0>>8;
        let b = moveto.0 & 255;
        if game.white_to_play { compute_move_w(a, b, &mut game1); }
        else { compute_move_b(a, b, &mut game1); }
        let move_score = minimax(&mut game1, depth, maximizing_player);
        eprint!("{}{} : {}", convert_square_to_move(a), convert_square_to_move(b), move_score);
        if move_score > score {
            score = move_score;
            bestmove = moveto.0;
        }
    }
    eprintln!();
    let a = bestmove >> 8;
    let b = bestmove & 255;
    (a, b)
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
        commande = &commande[15..];
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
    for one_move in move_tab {
        let (a,b) = convert_move_to_bitboard(one_move);
        if game.white_to_play {
            compute_move_w(a, b, &mut game);
        }
        else {
            compute_move_b(a, b, &mut game);
        }
        game.white_to_play ^= true;
        //println!("{one_move}");
    }
    game
}

/*fn get_bitboard_from_fen(_command : &str) -> Game {
    
}*/