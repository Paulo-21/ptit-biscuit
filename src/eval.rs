use bitintr::Popcnt;
use crate::chess::*;

pub fn eval(game : &Game, nmoves:i32 ) -> i32 {
    if nmoves == 0 {
        if game.white_to_play {
            return -99999 + (50 * game.nb_coups as i32);
        }
        else {
            return 99999 - (50 * game.nb_coups as i32);
        }
    }
    let white_score: i32 = (1100 * game.wq.popcnt() + 500*game.wr.popcnt() + 300*game.wb.popcnt() + 300*game.wn.popcnt() + 100*game.wp.popcnt()) as i32;
    let black_score: i32 = (1100 * game.bq.popcnt() + 500*game.br.popcnt() + 300*game.bb.popcnt() + 300*game.bn.popcnt() + 100*game.bp.popcnt()) as i32;
    let mut score = white_score - black_score;
    if game.white_to_play {
        score += nmoves;
        score -= get_legal_move(false, game).len() as i32;
    }
    else {
        score += get_legal_move(true, game).len() as i32;
        score -= nmoves;
    }
    score += (possibility_w(game) & SQUARE_CENTER).popcnt() as i32;
    score -= (possibility_b(game) & SQUARE_CENTER).popcnt() as i32;
    
    score
}
/*
fn eval_begin() {

}
fn eval_let_game() {

}
fn eval_middle_game() {

}*/