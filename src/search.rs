use crate::chess::*;
use std::cmp::{ max, min };
pub fn alpha_beta(game : &Game, depth : i8, mut alpha:i16, mut beta : i16, maximizing_player : bool, w_play:bool) -> i16 {
    let legal_move = get_legal_move(maximizing_player, game);
    if depth == 0 || legal_move.is_empty() {
        return eval(game);
    };
    let mut value;
    if maximizing_player {
        value = i16::MIN;
        for moveto in legal_move {
            let a = moveto.0 >> 8;
            let b = moveto.0 & !a;
            let mut game1 = game.clone();
            if w_play { compute_move_w(a, b, &mut game1); }
            else { compute_move_b(a, b, &mut game1); }
            
            value = max(value, alpha_beta(&game1, depth-1, alpha, beta, maximizing_player^true, w_play^true));
            if value > beta {
                break;
            }
            alpha = max(alpha, value);
        }
    }
    else {
        value = i16::MAX;
        for _moveto in legal_move {
            value = min(value, alpha_beta(game, depth-1, alpha, beta, maximizing_player^true, w_play^true));
            if value < alpha {
                break;
            }
            beta = min(beta, value)
        }
    }
    return value;
}

pub fn minimax(game: &Game, depth : i8, maximizing_player : bool) -> i16 {
    if depth == 0 {
        return eval(game);
    };
    let mut value;
    if maximizing_player {
        value = i16::MIN;
        let legal_moves = get_legal_move(maximizing_player, game);
        for _moveto in legal_moves {
            value = max(value, minimax(game, depth-1, maximizing_player^true));
        }
    }
    else {
        value = i16::MAX;
        let legal_moves = get_legal_move(maximizing_player, game);
        for _moveto in legal_moves {
            value = min(value, minimax(game, depth-1, maximizing_player^true));
        }
    }
    return value;
}

fn eval(game : &Game ) -> i16 {
    3
}