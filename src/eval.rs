use bitintr::{Popcnt, Tzcnt, Blsr};
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
    let mut n = game.wn;
    
    score += (wpo.popcnt() *2) as i32;
    score -= (bpo.popcnt() *2) as i32;
    while n != 0 { //KNIGHT POS SCORE
        let k = n.tzcnt();
        score += KNIGHT_POS_SCORE[k as usize];
        n = n.blsr();
    }
    n = game.bn;
    while n != 0 { //KNIGHT POS SCORE
        let k = n.tzcnt();
        score -= KNIGHT_POS_SCORE[k as usize];
        n = n.blsr();
    }
    if game.nb_coups < 13 {
        score += ((wpo & SQUARE_CENTER).popcnt() * 3 ) as i32;
        score -= ((bpo & SQUARE_CENTER).popcnt()  * 3 )as i32;
        score += eval_begin();
    }
    else if game.nb_coups < 30 {
        score += eval_middle_game(game);
    }
    else {
        score += eval_late_game(game);
    }
    score
}

fn eval_begin() -> i32 {
    0
}
fn eval_late_game(game : &Game) -> i32 {
    let k = KING_POS_END[game.wk.tzcnt() as usize] - KING_POS_END[game.bk.tzcnt() as usize];
    k as i32
}
fn eval_middle_game(game : &Game) -> i32 {
    let k = KING_POS_MIDDLE[game.wk.tzcnt() as usize] - KING_POS_MIDDLE[game.bk.tzcnt() as usize];
    k as i32
}