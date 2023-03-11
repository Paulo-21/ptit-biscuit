use crate::chess::*;
use std::cmp::{ max, min };

pub fn alpha_beta(game : &mut Game, depth : i8, mut alpha:i16, mut beta : i16, maximizing_player : bool) -> i16 {
    let legal_move = get_legal_move(maximizing_player, game);
    if depth == 0 || legal_move.is_empty() {
        return eval(game);
    };
    let mut value;
    if maximizing_player {
        value = i16::MIN;
        for moveto in legal_move {
            let a = moveto.0 >> 8;
            let b = moveto.0 & 255;
            let mut game1 = game.clone();
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
            value = min(value, alpha_beta(game, depth-1, alpha, beta, maximizing_player^true));
            if value < alpha {
                break;
            }
            beta = min(beta, value)
        }
    }
    return value;
}

pub fn minimax(game: &mut Game, depth : i8, maximizing_player : bool) -> i16 {
    if depth == 0 {
        //eprintln!("EVAL {}", eval(game));
        return eval(game);
    };
    let mut value;
    game.white_to_play ^= true;
    if maximizing_player {
        value = i16::MIN;
        let legal_moves = get_legal_move(game.white_to_play, game);
        for _moveto in legal_moves {
            let (a,b) = convert_custum_move(_moveto);
            let mut game1 = game.clone();

            if game.white_to_play { compute_move_w(a, b, &mut game1); }
            else { compute_move_b(a, b, &mut game1); }

            value = max(value, minimax(&mut game1, depth-1, maximizing_player^true));
        }
    }
    else {
        value = i16::MAX;
        let legal_moves = get_legal_move(game.white_to_play , game);
        for _moveto in legal_moves {
            let (a,b) = convert_custum_move(_moveto);
            let mut game1 = game.clone();
            
            if game.white_to_play { compute_move_w(a, b, &mut game1); }
            else { compute_move_b(a, b, &mut game1); }

            value = min(value, minimax(&mut game1, depth-1, maximizing_player^true));
        }
    }
    return value;
}

fn eval(game : &Game ) -> i16 {
    let white_score: i32 = (11 * game.wq.count_ones() + 5*game.wr.count_ones() + 3*game.wb.count_ones() + 3*game.wn.count_ones() + game.wp.count_ones()) as i32;
    let black_score: i32 = (11 * game.bq.count_ones() + 5*game.br.count_ones() + 3*game.bb.count_ones() + 3*game.bn.count_ones() + game.bp.count_ones()) as i32;
    let score = if game.white_to_play {
        white_score - black_score
    }
    else {
        black_score - white_score
    };
    score as i16
}