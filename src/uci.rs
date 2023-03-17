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
    println!("START Compute");
    compute_negamax(game)
    //compute_minimax(game)
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
        if !commande.is_empty() {
            return get_bitboard_from_startpos(commande)
        }
        Game::default()
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
fn compute_negamax(game : &Game) -> (u64 , u64) {
    println!("NEGAMAX");
    let mut nb_node = 0u64;
    let depth = 3;
    let legal_moves = get_legal_move(game.white_to_play, game);
    println!("info : {:?}", legal_moves);
    let mut score = i32::MIN;
    let mut bestmove = 0u64;
    if !legal_moves.is_empty() {
        bestmove = legal_moves.get(0).unwrap().0;
    }
    for moveto in legal_moves {
        let mut game1 = *game;
        let a = moveto.0>>8;
        let b = moveto.0 & 255;
        if game.white_to_play { compute_move_w(a, b, &mut game1); }
        else { compute_move_b(a, b, &mut game1); }
        game1.white_to_play ^= true;

        let move_score = (-1)*negamax(&mut game1, depth, game.white_to_play^true, &mut nb_node);
        eprintln!("{}{} : {}, ", convert_square_to_move(a), convert_square_to_move(b), move_score);
        
        if move_score > score {
            score = move_score;
            bestmove = moveto.0;
        }
    }
    eprintln!();
    let a = bestmove >> 8;
    let b = bestmove & 255;
    println!("NB nodes : {nb_node}");
    (a,b)
}
fn compute_minimax(game : &Game) -> (u64 , u64) {
    println!("MINIMAX");
    let depth = 3;
    let mut nb_node = 0u64;
    let maximizing_player = game.white_to_play;
    let legal_moves = get_legal_move(game.white_to_play, game);
    println!("info : {:?}", legal_moves);
    let mut score = if maximizing_player { i32::MAX } else { i32::MIN };
    let mut bestmove = 0u64;
    if !legal_moves.is_empty() {
        bestmove = legal_moves.get(0).unwrap().0;
    }
    for moveto in legal_moves {
        let mut game1 = *game;
        let a = moveto.0>>8;
        let b = moveto.0 & 255;
        if game.white_to_play { compute_move_w(a, b, &mut game1); }
        else { compute_move_b(a, b, &mut game1); }
        game1.white_to_play ^= true;
        let move_score = minimax(&mut game1, depth, maximizing_player^true, &mut nb_node);
        eprintln!("{}{} : {}, ", convert_square_to_move(a), convert_square_to_move(b), move_score);
        if maximizing_player {
            if move_score > score {
                score = move_score;
                bestmove = moveto.0;
                println!("Change");
            }
        }
        else {
            if move_score < score {
                score = move_score;
                bestmove = moveto.0;
            }
        }
        
    }
    eprintln!();
    let a = bestmove >> 8;
    let b = bestmove & 255;
    println!("NB nodes : {nb_node}");
    (a,b)
}

/*fn get_bitboard_from_fen(_command : &str) -> Game {
    
}*/