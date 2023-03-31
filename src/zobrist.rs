use crate::chess::Game;


const SEED: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

impl Game {
    pub fn as_array(&self) -> [u64; 12] {
        [self.wp, self.wn, self.wb, self.wr, self.wq, self.wk, self.bp, self.bn, self.bb, self.br, self.bq, self.bk]
    }
    pub fn castling_as_array(&self) -> [bool; 6] {
        [self.wking_never_move, self.wqueen_rook_never_move, self.wking_rook_never_move,
         self.bking_never_move, self.bqueen_rook_never_move, self.bking_rook_never_move]
    }
}

pub fn init_hash_key(
    piece_square : &mut [[u64;64]; 12], sidetomoveb: &mut u64, castling_right:&mut [u64;4],
    valid_enpassant : &mut [u64; 8],
) {
    let mut rng = XorShiftRng::from_seed(SEED);
    for board in piece_square {
        for square in board {
            *square = rng.next_u64();
        }
    }
    for right in castling_right {
        *right = rng.next_u64();
    }
    for enpassant in valid_enpassant {
        *enpassant = rng.next_u64();
    }
    *sidetomoveb = rng.next_u64();
}
pub fn init_zobrist_key(game : &Game,
    piece_square : &mut [[u64;64]; 12], sidetomoveb: &mut u64, castling_right:&mut [u64;4],
    valid_enpassant : &mut [u64; 8],
) -> u64 {
    let mut zobrist_key = 0u64;
    let mut i = 0;
    for mut piece in game.as_array() {
        while piece != 0 {
            zobrist_key ^= piece_square[i][piece.tzcnt() as usize];
            piece &= piece-1;
        }
        i+=1;
    }
    i = 0;
    for right in game.castling_as_array() {
        if right {
            zobrist_key ^= castling_right[i];
        }
    }
    zobrist_key ^= *sidetomoveb;

    zobrist_key
}