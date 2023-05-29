use bitintr::{Popcnt, Tzcnt, Blsr};
use crate::chess::*;

pub fn eval(game : &Game, nmoves:i32 ) -> i32 {
    if nmoves == 0 {
        if game.white_to_play {
            if is_attacked(game.white_to_play, game) {
                return -999999 + (50 * game.nb_coups as i32);
            }
            else {
                return 90999 - (50 * game.nb_coups as i32);
            }
        }
        else if is_attacked(game.white_to_play, game) {
            return 999999 - (50 * game.nb_coups as i32);
        }
        else {
            return -90999 + (50 * game.nb_coups as i32);
        }
    }/*
    let white_score: i32 = (1100 * game.wq.popcnt() + 500*game.wr.popcnt() + 300*game.wb.popcnt() + 300*game.wn.popcnt() + 100*game.wp.popcnt()) as i32;
    let black_score: i32 = (1100 * game.bq.popcnt() + 500*game.br.popcnt() + 300*game.bb.popcnt() + 300*game.bn.popcnt() + 100*game.bp.popcnt()) as i32;
    let mut score = white_score - black_score;
    let wpo = possibility_w(game);
    let bpo = possibility_b(game);
    let mut n = game.wn;
    
    score += (wpo.popcnt() ) as i32;
    score -= (bpo.popcnt() ) as i32;
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
        score += _eval_begin();
    }
    else if game.nb_coups < 30 {
        score += _eval_middle_game(game);
    }
    else {
        score += _eval_late_game(game);
    }
    score*/

    pesto_eval(game)
}


fn _eval_late_game(game : &Game) -> i32 {
    let k = _KING_POS_END[flip(game.wk).tzcnt() as usize] - _KING_POS_END[game.bk.tzcnt() as usize];
    k as i32
}

fn _eval_middle_game(game : &Game) -> i32 {
    let mut n = 0;
    let mut score = 0;
    n = game.wp;
    while n != 0 {
        let k = n.tzcnt();
        score += MG_PAWN_TABLE[k as usize];
        n = n.blsr();
    }
    n = game.bp;
    while n != 0 {
        let k = n.tzcnt();
        score -= MG_PAWN_TABLE[k as usize];
        n = n.blsr();
    }
    /*n = game.wn;
    while n != 0 {
        let k = n.tzcnt();
        score += MG_PAWN_TABLE[k as usize];
        n = n.blsr();
    }
    n = game.bn;
    while n != 0 {
        let k = n.tzcnt();
        score -= MG_PAWN_TABLE[k as usize];
        n = n.blsr();
    }*/
    score += _KING_POS_MIDDLE[flip(game.wk).tzcnt() as usize] - _KING_POS_MIDDLE[game.bk.tzcnt() as usize];
    score 
}

#[inline]
fn pesto_eval(game: &Game) -> i32 {

    let mut game_phase = 0;
    let mut mg_score = 0;
    let mut eg_score = 0;
    /*PAWN  */
    let mut n = game.wp;
    while n != 0 {
        mg_score += MG_VALUE[0] + MG_PAWN_TABLE[flip(n.tzcnt()) as usize];
        eg_score += EG_VALUE[0] + EG_PAWN_TABLE[flip(n.tzcnt()) as usize];
        n = n.blsr();
    }
    n = game.bp;
    while n != 0 {
        mg_score -= MG_VALUE[0] + MG_PAWN_TABLE[n.tzcnt() as usize];
        eg_score -= EG_VALUE[0] + EG_PAWN_TABLE[n.tzcnt() as usize];
        n = n.blsr();
    }

    /*KNIGHT */
    n = game.wn;
    while n != 0 {
        mg_score += MG_VALUE[1] + MG_KNIGHT_TABLE[flip(n.tzcnt()) as usize];
        eg_score += EG_VALUE[1] + EG_KNIGHT_TABLE[flip(n.tzcnt()) as usize];
        game_phase +=1;
        n = n.blsr();
    }
    n = game.bn;
    while n != 0 {
        mg_score -= MG_VALUE[1] + MG_KNIGHT_TABLE[n.tzcnt() as usize];
        eg_score -= EG_VALUE[1] + EG_KNIGHT_TABLE[n.tzcnt() as usize];
        game_phase +=1;
        n = n.blsr();
    }
    /*Bishop */
    n = game.wb;
    while n != 0 {
        mg_score += MG_VALUE[2] + MG_BISHOP_TABLE[flip(n.tzcnt()) as usize];
        eg_score += EG_VALUE[2] + EG_BISHOP_TABLE[flip(n.tzcnt()) as usize];
        game_phase +=1;
        n = n.blsr();
    }
    n = game.bb;
    while n != 0 {
        mg_score -= MG_VALUE[2] + MG_BISHOP_TABLE[n.tzcnt() as usize];
        eg_score -= EG_VALUE[2] + EG_BISHOP_TABLE[n.tzcnt() as usize];
        game_phase +=1;
        n = n.blsr();
    }
    /*Rook */
    n = game.wr;
    while n != 0 {
        mg_score += MG_VALUE[3] + MG_ROOK_TABLE[flip(n.tzcnt()) as usize];
        eg_score += EG_VALUE[3] + EG_ROOK_TABLE[flip(n.tzcnt()) as usize];
        game_phase +=2;
        n = n.blsr();
    }
    n = game.br;
    while n != 0 {
        mg_score -= MG_VALUE[3] + MG_ROOK_TABLE[n.tzcnt() as usize];
        eg_score -= MG_VALUE[3] + EG_ROOK_TABLE[n.tzcnt() as usize];
        game_phase +=2;
        n = n.blsr();
    }
    /*Queen */
    n = game.wq;
    while n != 0 {
        mg_score += MG_VALUE[4] + MG_QUEEN_TABLE[flip(n.tzcnt()) as usize];
        eg_score += EG_VALUE[4] + EG_QUEEN_TABLE[flip(n.tzcnt()) as usize];
        game_phase +=4;
        n = n.blsr();
    }
    n = game.bq;
    while n != 0 {
        mg_score -= MG_VALUE[4] + MG_QUEEN_TABLE[n.tzcnt() as usize];
        eg_score -= EG_VALUE[4] + EG_QUEEN_TABLE[n.tzcnt() as usize];
        game_phase +=4;
        n = n.blsr();
    }
    /*King */
    n = game.wk;
    while n != 0 {
        mg_score += MG_KING_TABLE[flip(n.tzcnt()) as usize];
        eg_score += EG_KING_TABLE[flip(n.tzcnt()) as usize];
        game_phase +=4;
        n = n.blsr();
    }
    n = game.bk;
    while n != 0 {
        mg_score -= MG_KING_TABLE[n.tzcnt() as usize];
        eg_score -= EG_KING_TABLE[n.tzcnt() as usize];
        game_phase +=4;
        n = n.blsr();
    }

    if game_phase > 24 { game_phase = 24 };
    (mg_score * game_phase + eg_score * (24-game_phase))/24
}


pub static MG_PAWN_TABLE : [i32 ; 64] = [
      0,   0,   0,   0,   0,   0,  0,   0,
     98, 134,  61,  95,  68, 126, 34, -11,
     -6,   7,  26,  31,  65,  56, 25, -20,
    -14,  13,   6,  21,  23,  12, 17, -23,
    -27,  -2,  -5,  12,  17,   6, 10, -25,
    -26,  -4,  -4, -10,   3,   3, 33, -12,
    -35,  -1, -20, -23, -15,  24, 38, -22,
      0,   0,   0,   0,   0,   0,  0,   0,
];

pub static EG_PAWN_TABLE : [i32;64] = [
      0,   0,   0,   0,   0,   0,   0,   0,
    178, 173, 158, 134, 147, 132, 165, 187,
     94, 100,  85,  67,  56,  53,  82,  84,
     32,  24,  13,   5,  -2,   4,  17,  17,
     13,   9,  -3,  -7,  -7,  -8,   3,  -1,
      4,   7,  -6,   1,   0,  -5,  -1,  -8,
     13,   8,   8,  10,  13,   0,   2,  -7,
      0,   0,   0,   0,   0,   0,   0,   0,
];
pub static  MG_KNIGHT_TABLE :[i32;64] = [
    -167, -89, -34, -49,  61, -97, -15, -107,
     -73, -41,  72,  36,  23,  62,   7,  -17,
     -47,  60,  37,  65,  84, 129,  73,   44,
      -9,  17,  19,  53,  37,  69,  18,   22,
     -13,   4,  16,  13,  28,  19,  21,   -8,
     -23,  -9,  12,  10,  19,  17,  25,  -16,
     -29, -53, -12,  -3,  -1,  18, -14,  -19,
    -105, -21, -58, -33, -17, -28, -19,  -23,
];

pub static EG_KNIGHT_TABLE : [i32;64] = [
    -58, -38, -13, -28, -31, -27, -63, -99,
    -25,  -8, -25,  -2,  -9, -25, -24, -52,
    -24, -20,  10,   9,  -1,  -9, -19, -41,
    -17,   3,  22,  22,  22,  11,   8, -18,
    -18,  -6,  16,  25,  16,  17,   4, -18,
    -23,  -3,  -1,  15,  10,  -3, -20, -22,
    -42, -20, -10,  -5,  -2, -20, -23, -44,
    -29, -51, -23, -15, -22, -18, -50, -64,
];
pub static MG_BISHOP_TABLE : [i32;64] = [
    -29,   4, -82, -37, -25, -42,   7,  -8,
    -26,  16, -18, -13,  30,  59,  18, -47,
    -16,  37,  43,  40,  35,  50,  37,  -2,
     -4,   5,  19,  50,  37,  37,   7,  -2,
     -6,  13,  13,  26,  34,  12,  10,   4,
      0,  15,  15,  15,  14,  27,  18,  10,
      4,  15,  16,   0,   7,  21,  33,   1,
    -33,  -3, -14, -21, -13, -12, -39, -21,
];

pub static EG_BISHOP_TABLE : [i32;64] = [
    -14, -21, -11,  -8, -7,  -9, -17, -24,
     -8,  -4,   7, -12, -3, -13,  -4, -14,
      2,  -8,   0,  -1, -2,   6,   0,   4,
     -3,   9,  12,   9, 14,  10,   3,   2,
     -6,   3,  13,  19,  7,  10,  -3,  -9,
    -12,  -3,   8,  10, 13,   3,  -7, -15,
    -14, -18,  -7,  -1,  4,  -9, -15, -27,
    -23,  -9, -23,  -5, -9, -16,  -5, -17,
];
pub static MG_ROOK_TABLE : [i32;64] = [
     32,  42,  32,  51, 63,  9,  31,  43,
     27,  32,  58,  62, 80, 67,  26,  44,
     -5,  19,  26,  36, 17, 45,  61,  16,
    -24, -11,   7,  26, 24, 35,  -8, -20,
    -36, -26, -12,  -1,  9, -7,   6, -23,
    -45, -25, -16, -17,  3,  0,  -5, -33,
    -44, -16, -20,  -9, -1, 11,  -6, -71,
    -19, -13,   1,  17, 16,  7, -37, -26,
];

pub static EG_ROOK_TABLE : [i32;64] = [
    13, 10, 18, 15, 12,  12,   8,   5,
    11, 13, 13, 11, -3,   3,   8,   3,
     7,  7,  7,  5,  4,  -3,  -5,  -3,
     4,  3, 13,  1,  2,   1,  -1,   2,
     3,  5,  8,  4, -5,  -6,  -8, -11,
    -4,  0, -5, -1, -7, -12,  -8, -16,
    -6, -6,  0,  2, -9,  -9, -11,  -3,
    -9,  2,  3, -1, -5, -13,   4, -20,
];

pub static MG_QUEEN_TABLE : [i32;64] = [
    -28,   0,  29,  12,  59,  44,  43,  45,
    -24, -39,  -5,   1, -16,  57,  28,  54,
    -13, -17,   7,   8,  29,  56,  47,  57,
    -27, -27, -16, -16,  -1,  17,  -2,   1,
     -9, -26,  -9, -10,  -2,  -4,   3,  -3,
    -14,   2, -11,  -2,  -5,   2,  14,   5,
    -35,  -8,  11,   2,   8,  15,  -3,   1,
     -1, -18,  -9,  10, -15, -25, -31, -50,
];

pub static EG_QUEEN_TABLE : [i32;64] = [
     -9,  22,  22,  27,  27,  19,  10,  20,
    -17,  20,  32,  41,  58,  25,  30,   0,
    -20,   6,   9,  49,  47,  35,  19,   9,
      3,  22,  24,  45,  57,  40,  57,  36,
    -18,  28,  19,  47,  31,  34,  39,  23,
    -16, -27,  15,   6,   9,  17,  10,   5,
    -22, -23, -30, -16, -16, -23, -36, -32,
    -33, -28, -22, -43,  -5, -32, -20, -41,
];
pub static MG_KING_TABLE : [i32;64] = [
    -65,  23,  16, -15, -56, -34,   2,  13,
     29,  -1, -20,  -7,  -8,  -4, -38, -29,
     -9,  24,   2, -16, -20,   6,  22, -22,
    -17, -20, -12, -27, -30, -25, -14, -36,
    -49,  -1, -27, -39, -46, -44, -33, -51,
    -14, -14, -22, -46, -44, -30, -15, -27,
      1,   7,  -8, -64, -43, -16,   9,   8,
    -15,  36,  12, -54,   8, -28,  24,  14,
];

pub static EG_KING_TABLE : [i32;64] = [
    -74, -35, -18, -18, -11,  15,   4, -17,
    -12,  17,  14,  17,  17,  38,  23,  11,
     10,  17,  23,  15,  20,  45,  44,  13,
     -8,  22,  24,  27,  26,  33,  26,   3,
    -18,  -4,  21,  24,  27,  23,   9, -11,
    -19,  -3,  11,  21,  23,  16,   7,  -9,
    -27, -11,   4,  13,  14,   4,  -5, -17,
    -53, -34, -21, -11, -28, -14, -24, -43
];
pub static _KNIGHT_POS_SCORE : [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

pub static _KING_POS_MIDDLE : [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    20, 20,  0,  0,  0,  0, 20, 20,
    20, 30, 10,  0,  0, 10, 30, 20
];
pub static _KING_POS_END : [i32; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50
];


static MG_VALUE : [i32;6] = [ 82, 337, 365, 477, 1025,  0];

static EG_VALUE : [i32;6] = [ 94, 281, 297, 512,  936,  0];

pub fn _flip_vertical( x : u64) -> u64 {
    return  ( (x << 56)                           ) |
            ( (x << 40) & (0x00ff000000000000) ) |
            ( (x << 24) & (0x0000ff0000000000) ) |
            ( (x <<  8) & (0x000000ff00000000) ) |
            ( (x >>  8) & (0x00000000ff000000) ) |
            ( (x >> 24) & (0x0000000000ff0000) ) |
            ( (x >> 40) & (0x000000000000ff00) ) |
            ( (x >> 56) );
}
#[inline(always)]
pub fn flip(x : u64) -> u64 {
    x^56
}