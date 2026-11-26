use crate::bmi_features::Tzcnt;
use crate::chess::Game;
use lazy_static::lazy_static;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

const SEED: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
lazy_static! {
    pub static ref PIECE_SQUARE: [[u64; 64]; 12] = {
        let mut piece_square = [[0u64; 64]; 12];
        let mut rng = XorShiftRng::from_seed(SEED);
        for i in 0..12 {
            for k in 0..64 {
                piece_square[i][k] = rng.next_u64();
            }
        }
        piece_square
    };
    pub static ref CASTLING_RIGHT: [u64; 4] = {
        let mut piece_square = [[0u64; 64]; 12];
        let mut castling_right = [0u64; 4];
        let mut rng = XorShiftRng::from_seed(SEED);
        for i in 0..12 {
            for k in 0..64 {
                piece_square[i][k] = rng.next_u64();
            }
        }
        for i in 0..4 {
            castling_right[i] = rng.next_u64();
        }
        castling_right
    };
    pub static ref VALID_ENPASSANT: [u64; 8] = {
        let mut rng = XorShiftRng::from_seed(SEED);
        let mut castling_right = [0u64; 4];
        let mut piece_square = [[0u64; 64]; 12];
        let mut valid_enpassant = [0u64; 8];
        for i in 0..12 {
            for k in 0..64 {
                piece_square[i][k] = rng.next_u64();
            }
        }
        for i in 0..4 {
            castling_right[i] = rng.next_u64();
        }
        for i in 0..8 {
            valid_enpassant[i] = rng.next_u64();
        }
        valid_enpassant
    };
    pub static ref SIDETOMOVE: u64 = {
        let mut rng = XorShiftRng::from_seed(SEED);
        let mut castling_right = [0u64; 4];
        let mut piece_square = [[0u64; 64]; 12];
        let mut valid_enpassant = [0u64; 8];

        for i in 0..12 {
            for k in 0..64 {
                piece_square[i][k] = rng.next_u64();
            }
        }
        for i in 0..4 {
            castling_right[i] = rng.next_u64();
        }
        for i in 0..8 {
            valid_enpassant[i] = rng.next_u64();
        }
        rng.next_u64()
    };
}

impl Game {
    pub fn as_array(&self) -> [u64; 12] {
        [
            self.wp, self.wn, self.wb, self.wr, self.wq, self.wk, self.bp, self.bn, self.bb,
            self.br, self.bq, self.bk,
        ]
    }
    pub fn castling_as_array(&self) -> [bool; 4] {
        [
            self.wqueen_castle,
            self.wking_castle,
            self.bqueen_castle,
            self.bking_castle,
        ]
    }
}
/// Partie du hash qui ne dépend pas des pièces : droits de roque, case
/// en passant et trait (SIDETOMOVE présent ssi c'est aux noirs de jouer).
#[inline(always)]
fn state_key(game: &Game) -> u64 {
    let mut key = 0u64;
    for (i, right) in game.castling_as_array().into_iter().enumerate() {
        if right {
            key ^= CASTLING_RIGHT[i];
        }
    }
    if game.en_passant != 0 {
        key ^= VALID_ENPASSANT[(game.en_passant.tzcnt() & 7) as usize];
    }
    if !game.white_to_play {
        key ^= *SIDETOMOVE;
    }
    key
}

pub fn init_zobrist_key(game: &Game) -> u64 {
    let mut zobrist_key = state_key(game);
    for (i, mut piece) in game.as_array().into_iter().enumerate() {
        while piece != 0 {
            zobrist_key ^= PIECE_SQUARE[i][piece.tzcnt() as usize];
            piece &= piece - 1;
        }
    }
    zobrist_key
}

/// Différence de hash entre deux positions séparées par un coup.
/// Le trait n'est pas encore inversé dans `after` (c'est l'appelant qui le
/// fait), on l'inverse donc ici.
#[inline(always)]
pub fn zobrist_delta(before: &Game, after: &Game) -> u64 {
    let mut delta = *SIDETOMOVE;
    let (b, a) = (before.as_array(), after.as_array());
    for i in 0..12 {
        let mut diff = b[i] ^ a[i];
        while diff != 0 {
            delta ^= PIECE_SQUARE[i][diff.tzcnt() as usize];
            diff &= diff - 1;
        }
    }
    let mut a_state = *after;
    a_state.white_to_play = before.white_to_play;
    delta ^ state_key(before) ^ state_key(&a_state)
}
