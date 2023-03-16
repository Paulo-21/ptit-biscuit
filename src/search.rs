use crate::chess::*;
use std::cmp::{ max, min };

pub fn alpha_beta(game : &mut Game, depth : i8, mut alpha:i16, mut beta : i16, maximizing_player : bool) -> i16 {
    let legal_move = get_legal_move(maximizing_player, game);
    if depth == 0 {
        return eval(game, legal_move.len() as i32);
    };
    let mut value;
    if maximizing_player {
        value = i16::MIN;
        for moveto in legal_move {
            let a = moveto.0 >> 8;
            let b = moveto.0 & 255;
            let mut game1 = *game;
            game1.white_to_play ^= true;
            if game1.white_to_play { compute_move_w(a, b, &mut game1); }
            else { compute_move_b(a, b, &mut game1); }
            value = max(value, alpha_beta(&mut game1, depth-1, alpha, beta, maximizing_player^true));
            if value > beta {
                break;
            }
            alpha = max(alpha, value);
        }
    }
    else {
        value = i16::MAX;
        for _moveto in legal_move {
            let mut game1 = *game;
            value = min(value, alpha_beta(&mut game1, depth-1, alpha, beta, maximizing_player^true));
            if value < alpha {
                break;
            }
            beta = min(beta, value)
        }
    }
    value
}

pub fn negamax(game: &mut Game, depth : i8, color : bool, nb_node : &mut u64) -> i16 {
    let legal_moves = get_legal_move(game.white_to_play, game);
    *nb_node+=1;
    if depth == 0 {
        let mut eval = eval(game, legal_moves.len() as i32);
        if color {
            eval *= -1;
        };
        return eval;
    };
    let mut value = i16::MIN;
    for moveto_play in legal_moves {
        let (a,b) = convert_custum_move(moveto_play);
        let mut game1 = *game;

        if game.white_to_play { compute_move_w(a, b, &mut game1); }
        else { compute_move_b(a, b, &mut game1); }

        value = max(value, -negamax(&mut game1, depth-1, color^true, nb_node));
    }
    value
}

pub fn minimax(game: &mut Game, depth : i8, maximizing_player : bool, nb_node : &mut u64) -> i16 {
    let legal_moves = get_legal_move(game.white_to_play, game);
    *nb_node+=1;
    if depth == 0 {
        return eval(game, legal_moves.len() as i32);
    };
    let mut value;

    if maximizing_player {
        value = i16::MIN;
        for _moveto in legal_moves {
            let (a,b) = convert_custum_move(_moveto);
            let mut game1 = *game;

            if game.white_to_play { compute_move_w(a, b, &mut game1); }
            else { compute_move_b(a, b, &mut game1); }
            game1.white_to_play ^= true;
            value = max(value, minimax(&mut game1, depth-1, false, nb_node));
        }
    }
    else {
        value = i16::MAX;
        for _moveto in legal_moves {
            let (a,b) = convert_custum_move(_moveto);
            let mut game1 = *game;
            
            if game.white_to_play { compute_move_w(a, b, &mut game1); }
            else { compute_move_b(a, b, &mut game1); }

            game1.white_to_play ^= true;
            value = min(value, minimax(&mut game1, depth-1, true, nb_node));
        }
    }
    value
}

fn eval(game : &Game, nmoves:i32 ) -> i16 {
    let white_score: i32 = (1100 * game.wq.count_ones() + 500*game.wr.count_ones() + 300*game.wb.count_ones() + 300*game.wn.count_ones() + 100*game.wp.count_ones()) as i32;
    let black_score: i32 = (1100 * game.bq.count_ones() + 500*game.br.count_ones() + 300*game.bb.count_ones() + 300*game.bn.count_ones() + 100*game.bp.count_ones()) as i32;
    let mut score = white_score - black_score;
    score += nmoves;
    /*if game.white_to_play {
        poss
    }*/

    score as i16    
}