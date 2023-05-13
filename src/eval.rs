use bitintr::Popcnt;
use crate::chess::*;

pub fn eval(game : &Game, nmoves:i32 ) -> i32 {
    if nmoves == 0 {
        if game.white_to_play {
            if is_attacked(game.white_to_play, game) {
                return -99999 + (50 * game.nb_coups as i32);
            }
            else {
                return 90999 - (50 * game.nb_coups as i32);
            }
        }
        else if is_attacked(game.white_to_play, game) {
            return 99999 - (50 * game.nb_coups as i32);
        }
        else {
            return -90999 + (50 * game.nb_coups as i32);
        }
    }
    let white_score: i32 = (1100 * game.wq.popcnt() + 500*game.wr.popcnt() + 300*game.wb.popcnt() + 300*game.wn.popcnt() + 100*game.wp.popcnt()) as i32;
    let black_score: i32 = (1100 * game.bq.popcnt() + 500*game.br.popcnt() + 300*game.bb.popcnt() + 300*game.bn.popcnt() + 100*game.bp.popcnt()) as i32;
    let mut score = white_score - black_score;
    let wpo = possibility_w(game);
    let bpo = possibility_b(game);
    score += wpo.popcnt() as i32;
    score -= bpo.popcnt() as i32;
    score += ((wpo & SQUARE_CENTER).popcnt() /* 10 */) as i32;
    score -= ((bpo & SQUARE_CENTER).popcnt() /* 10 */)as i32;
    
    score
}
/*
fn eval_begin() {

}
fn eval_let_game() {

}
fn eval_middle_game() {

}*/