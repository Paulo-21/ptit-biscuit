use crate::chess::*;
use crate::table_transposition::*;
pub fn perft(mut game: Game, depth : i8) -> usize {
    let mut nb_nodes = 0;
    
    let legal_moves = get_legal_moves_fast(&mut game);
    //let legal_moves = get_legal_move(game.white_to_play,&game);
    
    if depth == 1 || legal_moves.is_empty() {
        return legal_moves.len();
    }
    for moveto in legal_moves {
        let mut game1 = game;
        //let moves = convert_custum_move(moveto);
        let moves = convert_custum_move2(moveto);
        if game.white_to_play {
            compute_move_w_thrust(moves, &mut game1);
        }
        else {
            compute_move_b_thrust(moves, &mut game1);
        }
        game1.white_to_play^=true;
        nb_nodes += perft(game1, depth-1);
    }
    nb_nodes
}
pub fn perft_divide(mut game : Game, depth : i8) {
    let legal_moves = get_legal_moves_fast(&mut game);
    let mut nb_nodes_tot = 0;
    for moveto in legal_moves {
        
        let mut game1 = game;
        let moves = convert_custum_move2(moveto);
        if game.white_to_play {
            compute_move_w_thrust(moves, &mut game1);
        }
        else {
            compute_move_b_thrust(moves, &mut game1);
        }
        game1.white_to_play^=true;
        let nb_nodes = perft(game1, depth-1);
        nb_nodes_tot += nb_nodes;
        println!("{} {}", convert_move_to_str(moves.0, moves.1, moves.2), nb_nodes);
    }
    println!();
    println!("{nb_nodes_tot}");
    //println!("Nodes searched: {nb_nodes_tot}");
}

pub fn _perft_hash(game: Game, hash_table : &TranspositionTable , depth : i8) -> usize {
    let mut nb_nodes = 0;
    let legal_moves = get_legal_move(game.white_to_play, &game);
    if depth == 1 {
        return legal_moves.len();
    }
    if depth == 0 {
        return 1;
    }
    for moveto in legal_moves {
        let mut game1 = game;
        let moves = convert_custum_move(moveto);
        if game.white_to_play {
            compute_move_w(moves, &mut game1);
        }
        else {
            compute_move_b(moves, &mut game1);
        }
        game1.white_to_play^=true;
        nb_nodes += _perft_hash(game1, hash_table, depth-1);
    }
    nb_nodes
}