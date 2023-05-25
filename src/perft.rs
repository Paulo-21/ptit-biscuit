use crate::chess::*;
use crate::table_transposition::*;
pub fn perft(game: Game, depth : i8) -> usize {
    let mut nb_nodes = 0;
    
    let legal_moves = get_legal_moves_fast(&game);
    //let legal_moves = get_legal_move(game.white_to_play,&game);
    
    if depth == 1 {
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

pub fn _perft_hash(game: Game, hash_table : &TranspositionTable , depth : i8) -> usize {
    let mut nb_nodes = 0;
    let legal_moves = get_legal_move(game.white_to_play, &game);
    if depth == 1 {
        return legal_moves.len();
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