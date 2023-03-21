use crate::chess::*;
use crate::eval::*;
use std::cmp::{ max, min };

pub fn alpha_beta(game : &mut Game, depth : i8, mut alpha:i32, mut beta : i32, nb_node : &mut u64) -> i32 {
    *nb_node+=1;
    let legal_move = get_legal_move(game.white_to_play, game);
    if depth == 0 || legal_move.len() == 0 {
        return eval(game, legal_move.len() as i32);
    };
    let mut value;
    game.nb_coups+=1;
    if !game.white_to_play {
        value = i32::MAX;
        for moveto in legal_move {
            let (a, b, prom) = convert_custum_move(moveto);
            let mut game1 = *game;
            
            game1.white_to_play ^= true;
            compute_move_b((a, b, prom), &mut game1);
            value = min(value, alpha_beta(&mut game1, depth-1, alpha, beta, nb_node));
            if alpha >= value {
                return value;
            }
            beta = min(beta, value);
        }
    }
    else {
        value = i32::MIN;
        for moveto in legal_move {
            let (a,b, prom) = convert_custum_move(moveto);
            let mut game1 = *game;
            compute_move_w((a, b, prom), &mut game1);
            game1.white_to_play ^= true;
            
            value = max(value, alpha_beta(&mut game1, depth-1, alpha, beta, nb_node));
            if value >= beta {
                return value;
            }
            alpha = max(alpha, value)
        }
    }
    value
}
/*
pub fn alpha_beta_neg(game: &Game, depth : i8, mut alpha : i32, mut beta : i32) -> i32 {
    let legal_moves = get_legal_move(game.white_to_play, game);
    if depth == 0 || legal_moves.len() == 0 {
        let mut eval = eval(game, legal_moves.len() as i32);
        if !game.white_to_play {
            eval *= -1;
        };
        return eval;
    };
    let mut value = i32::MIN;
    for moveto_play in legal_moves {
        let (a,b, prom) = convert_custum_move(moveto_play);

        let mut game1 = *game;

        if game.white_to_play { compute_move_w((a, b, Piece::NONE), &mut game1); }
        else { compute_move_b((a, b, Piece::NONE), &mut game1); }
        game1.white_to_play^=true;
        
        value = max(value, alpha_beta_neg(&mut game1, depth-1, -beta, -alpha)*(-1i32));
        if value >= beta {
            return value;
        }
        alpha = max(alpha, value)
    }
    value
}

pub fn negamax(game: &mut Game, depth : i8, color : bool, nb_node : &mut u64) -> i32 {
    let legal_moves = get_legal_move(game.white_to_play, game);
    *nb_node+=1;
    if depth == 0 || legal_moves.len() == 0 {
        let mut eval = eval(game, legal_moves.len() as i32);
        if !color {
            eval *= -1;
        };
        return eval;
    };
    let mut value = i32::MIN;
    for moveto_play in legal_moves {
        let (a,b) = convert_custum_move(moveto_play);
        let mut game1 = *game;

        if game.white_to_play { compute_move_w(a, b, &mut game1); }
        else { compute_move_b(a, b, &mut game1); }
        game1.white_to_play^=true;
        value = max(value, negamax(&mut game1, depth-1, color^true, nb_node)*(-1i32));
    }
    value
}
*/
pub fn minimax(game: &mut Game, depth : i8, maximizing_player : bool, nb_node : &mut u64) -> i32 {
    let legal_moves = get_legal_move(game.white_to_play, game);
    *nb_node+=1;
    if depth == 0 || legal_moves.len() == 0 {
        return eval(game, legal_moves.len() as i32);
    };
    let mut value;

    if maximizing_player {
        value = i32::MIN;
        for _moveto in legal_moves {
            let (a,b, prom) = convert_custum_move(_moveto);
            let mut game1 = *game;
            compute_move_w((a, b, prom), &mut game1);
            
            game1.white_to_play ^= true;
            value = max(value, minimax(&mut game1, depth-1, false, nb_node));
        }
    }
    else {
        value = i32::MAX;
        for _moveto in legal_moves {
            let (a,b, prom) = convert_custum_move(_moveto);
            let mut game1 = *game;
            compute_move_b((a, b,prom), &mut game1);

            game1.white_to_play ^= true;
            value = min(value, minimax(&mut game1, depth-1, true, nb_node));
        }
    }
    value
}

