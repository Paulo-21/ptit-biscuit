use crate::chess::*;
use std::cmp::{ max, min };
use bitintr::Popcnt;

pub fn alpha_beta(game : &mut Game, depth : i8, mut alpha:i32, mut beta : i32, maximizing_player : bool) -> i32 {
    let legal_move = get_legal_move(maximizing_player, game);
    if depth == 0 {
        return eval(game, legal_move.len() as i32);
    };
    let mut value;
    if maximizing_player {
        value = i32::MIN;
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
        value = i32::MAX;
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
            let (a,b) = convert_custum_move(_moveto);
            let mut game1 = *game;

            if game.white_to_play { compute_move_w(a, b, &mut game1); }
            else { compute_move_b(a, b, &mut game1); }
            game1.white_to_play ^= true;
            value = max(value, minimax(&mut game1, depth-1, false, nb_node));
        }
    }
    else {
        value = i32::MAX;
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

fn eval(game : &Game, nmoves:i32 ) -> i32 {
    let white_score: i32 = (1100 * game.wq.popcnt() + 500*game.wr.popcnt() + 300*game.wb.popcnt() + 300*game.wn.popcnt() + 100*game.wp.popcnt()) as i32;
    let black_score: i32 = (1100 * game.bq.popcnt() + 500*game.br.popcnt() + 300*game.bb.popcnt() + 300*game.bn.popcnt() + 100*game.bp.popcnt()) as i32;
    let mut score = white_score - black_score;
    if game.white_to_play {
        score += nmoves/10;
    }
    else {
        score -= nmoves/10;
    }
    if nmoves == 0 {
        if game.white_to_play {
            score = -99999;
        }
        else {
            score = 99999;
        }
    }
    score
}