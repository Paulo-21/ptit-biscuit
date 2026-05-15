pub fn get_legal_moves_fast(game: &mut Game, legal_moves: &mut [u64; 70]) -> usize {
    //Vec<u64> {
    //let mut legal_moves = Vec::with_capacity(30);
    //let mut legal_moves = [0u64; 70];
    let mut i = 0;
    let white = game.white();
    let black = game.black();
    let occupied = white | black;
    let empty = !occupied;
    if game.white_to_play {
        //WHITE
        let checkmask = get_checked_mask_w(game);
        let (pin_hv, pin_d12) = get_pinned_mask_w(game);
        game.en_passant &= !RANK_MASK[2];
        //PAWN
        let unpinned_wp = game.wp & !pin_hv & !pin_d12;
        let mut p_at =
            (unpinned_wp & !FILE_MASKS[0]) & ((black | game.en_passant) >> 7) & (checkmask >> 7)
                | (game.wp & pin_d12 & (black & pin_d12) >> 7 & (checkmask >> 7));
        let mut p_at2 =
            (unpinned_wp & !FILE_MASKS[7]) & ((black | game.en_passant) >> 9) & (checkmask >> 9)
                | (game.wp & pin_d12 & (black & pin_d12) >> 9 & (checkmask >> 9));
        let mut p_at3 =
            (unpinned_wp) & ((empty >> 8) & (empty >> 16)) & RANK_MASK[1] & (checkmask >> 16)
                | (game.wp
                    & pin_hv
                    & (empty & pin_hv) >> 8
                    & RANK_MASK[1]
                    & (empty & pin_hv) >> 16
                    & (checkmask >> 16));
        let mut p_at4 = (unpinned_wp) & (empty >> 8) & (checkmask >> 8)
            | (game.wp & pin_hv & (empty & pin_hv) >> 8 & (checkmask >> 8));

        while p_at != 0 {
            let pi_square = p_at.tzcnt();
            let piece = 1 << pi_square;
            //legal_moves.push((pi_square <<9) | (((pi_square+7)<<1)) | (((piece)&RANK_MASK[6]) != 0) as u64);
            legal_moves[i] =
                (pi_square << 9) | ((pi_square + 7) << 1) | (((piece) & RANK_MASK[6]) != 0) as u64;
            i += 1;
            p_at = p_at.blsr();
        }
        while p_at2 != 0 {
            let pi_square = p_at2.tzcnt();
            let piece = 1 << pi_square;
            //legal_moves.push((pi_square <<9) | ((pi_square+9)<<1)  | (((piece)&RANK_MASK[6]) != 0) as u64);
            legal_moves[i] =
                (pi_square << 9) | ((pi_square + 9) << 1) | (((piece) & RANK_MASK[6]) != 0) as u64;
            i += 1;
            p_at2 = p_at2.blsr();
        }
        while p_at3 != 0 {
            let pi_square = p_at3.tzcnt();
            let piece = 1 << pi_square;
            //legal_moves.push((pi_square <<9) | ((pi_square+16)<<1)  | (((piece)&RANK_MASK[6]) !=0) as u64);
            legal_moves[i] =
                (pi_square << 9) | ((pi_square + 16) << 1) | (((piece) & RANK_MASK[6]) != 0) as u64;
            i += 1;
            p_at3 = p_at3.blsr();
        }
        while p_at4 != 0 {
            let pi_square = p_at4.tzcnt();
            let piece = 1 << pi_square;
            //legal_moves.push((pi_square <<9) | ((pi_square+8)<<1)  | (((piece)&RANK_MASK[6])!=0) as u64);
            legal_moves[i] =
                (pi_square << 9) | ((pi_square + 8) << 1) | (((piece) & RANK_MASK[6]) != 0) as u64;
            i += 1;
            p_at4 = p_at4.blsr();
        }

        //KNIGHT
        let mut copy = game.wn & !(pin_hv | pin_d12);
        while copy != 0 {
            let copy_tzcnt = copy.tzcnt();
            let mut att = KNIGHT_MOVE[copy_tzcnt as usize] & !white & checkmask;
            while att != 0 {
                //legal_moves.push((copy.tzcnt() <<9) + (att.tzcnt()<<1) );
                legal_moves[i] = (copy_tzcnt << 9) + (att.tzcnt() << 1);
                i += 1;
                att = att.blsr();
            }
            copy = copy.blsr();
        }
        //BISHOP
        let mut p = (game.wb | game.wq) & !(pin_hv | pin_d12);
        //p |= game.wq & !(pin_hv | pin_d12);
        let mut p1 = (game.wb | game.wq) & pin_d12;
        //p1  |= game.wq & pin_d12;
        while p != 0 {
            let p_tzcnt = p.tzcnt();
            let mut att = diag_antid_moves(p_tzcnt, occupied) & !white & checkmask;
            while att != 0 {
                //legal_moves.push((p.tzcnt() <<9) + (att.tzcnt()<<1) );
                legal_moves[i] = (p_tzcnt << 9) + (att.tzcnt() << 1);
                i += 1;
                att = att.blsr();
            }
            p = p.blsr();
        }
        while p1 != 0 {
            let p1_tzcnt = p1.tzcnt();
            let mut att = diag_antid_moves(p1_tzcnt, occupied) & !white & checkmask & pin_d12;
            while att != 0 {
                //legal_moves.push((p1.tzcnt() <<9) + (att.tzcnt()<<1) );
                legal_moves[i] = (p1_tzcnt << 9) + (att.tzcnt() << 1);
                i += 1;
                att = att.blsr();
            }
            p1 = p1.blsr();
        }
        //ROOK
        let mut p = (game.wr | game.wq) & !(pin_hv | pin_d12);
        let mut p1 = (game.wr | game.wq) & pin_hv;
        //p |= game.wq & !(pin_hv | pin_d12);
        //p1 |= game.wq & pin_hv;
        //let mut p2 = game.wq & pin_d12;
        while p != 0 {
            let p_tzcnt = p.tzcnt();
            let mut att = hv_moves(p_tzcnt, occupied) & !white & checkmask;
            while att != 0 {
                //legal_moves.push((p.tzcnt() <<9) + (att.tzcnt()<<1) );
                legal_moves[i] = (p_tzcnt << 9) + (att.tzcnt() << 1);
                i += 1;
                att = att.blsr();
            }
            p = p.blsr();
        }
        while p1 != 0 {
            let p1_tzcnt = p1.tzcnt();
            let mut att = hv_moves(p1_tzcnt, occupied) & !white & checkmask & pin_hv;
            while att != 0 {
                //legal_moves.push((p1.tzcnt() <<9) + (att.tzcnt()<<1) );
                legal_moves[i] = (p1_tzcnt << 9) + (att.tzcnt() << 1);
                i += 1;
                att = att.blsr();
            }
            p1 = p1.blsr();
        }
        //KING
        let attack_b = attack_b(game);
        let wk_tzcnt = game.wk.tzcnt();
        let mut p = KING_MOVE[wk_tzcnt as usize] & !attack_b & !white; // & !pin_hv & !pin_d12;
        while p != 0 {
            //legal_moves.push((game.wk.tzcnt() <<9) + (p.tzcnt()<<1) );
            legal_moves[i] = (wk_tzcnt << 9) + (p.tzcnt() << 1);
            i += 1;
            p = p.blsr();
        }
        if game.wqueen_castle
            && occupied & (2u64.pow(3) + 2u64.pow(2) + 2u64.pow(1)) == 0
            && attack_b & (2u64.pow(3) + 2u64.pow(2) + 2u64.pow(4)) == 0
        {
            //legal_moves.push((4<<9)+(2<<1));
            legal_moves[i] = (4 << 9) + (2 << 1);
            i += 1;
        }
        if game.wking_castle
            && occupied & (2u64.pow(6) + 2u64.pow(5)) == 0
            && attack_b & (2u64.pow(6) + 2u64.pow(5) + 2u64.pow(4)) == 0
        {
            //legal_moves.push((4<<9)+(6<<1));
            legal_moves[i] = (4 << 9) + (6 << 1);
            i += 1;
        }
    } else {
        //BLACK
        let checkmask = get_checked_mask_b(game);
        let (pin_hv, pin_d12) = get_pinned_mask_b(game);
        game.en_passant &= !RANK_MASK[5];
        //_draw_bitboard(black);
        //PAWN
        let unpinned_bp = game.bp & !pin_hv & !pin_d12;
        let mut p_at =
            (unpinned_bp & !FILE_MASKS[7]) & ((white | game.en_passant) << 7) & (checkmask << 7)
                | (game.bp & pin_d12 & (white & pin_d12) << 7 & (checkmask << 7));
        let mut p_at2 =
            (unpinned_bp & !FILE_MASKS[0]) & ((white | game.en_passant) << 9) & (checkmask << 9)
                | (game.bp & pin_d12 & (white & pin_d12) << 9 & (checkmask << 9));
        let mut p_at3 =
            (unpinned_bp) & ((empty << 16) & (empty << 8)) & RANK_MASK[6] & (checkmask << 16)
                | (game.bp
                    & pin_hv
                    & (empty & pin_hv) << 8
                    & RANK_MASK[6]
                    & (empty & pin_hv) << 16
                    & (checkmask << 16));
        let mut p_at4 = (unpinned_bp) & (empty << 8) & (checkmask << 8)
            | (game.bp & pin_hv & (empty & pin_hv) << 8 & (checkmask << 8));

        while p_at != 0 {
            let pi_square = p_at.tzcnt();
            //legal_moves.push((pi_square <<9) + ((pi_square-7)<<1) | (((1<<pi_square)&RANK_MASK[1]) != 0) as u64);
            legal_moves[i] = (pi_square << 9) + ((pi_square - 7) << 1)
                | (((1 << pi_square) & RANK_MASK[1]) != 0) as u64;
            i += 1;
            p_at = p_at.blsr();
        }
        while p_at2 != 0 {
            let pi_square = p_at2.tzcnt();
            //legal_moves.push((pi_square <<9) + ((pi_square-9)<<1) | (((1<<pi_square)&RANK_MASK[1]) != 0) as u64);
            legal_moves[i] = (pi_square << 9) + ((pi_square - 9) << 1)
                | (((1 << pi_square) & RANK_MASK[1]) != 0) as u64;
            i += 1;
            p_at2 = p_at2.blsr();
        }
        while p_at3 != 0 {
            let pi_square = p_at3.tzcnt();
            //legal_moves.push((pi_square <<9) + ((pi_square-16)<<1) | (((1<<pi_square)&RANK_MASK[1]) != 0) as u64);
            legal_moves[i] = (pi_square << 9) + ((pi_square - 16) << 1)
                | (((1 << pi_square) & RANK_MASK[1]) != 0) as u64;
            i += 1;
            p_at3 = p_at3.blsr();
        }
        while p_at4 != 0 {
            let pi_square = p_at4.tzcnt();
            //legal_moves.push((pi_square <<9) + ((pi_square-8)<<1) | (((1<<pi_square)&RANK_MASK[1]) != 0) as u64);
            legal_moves[i] = (pi_square << 9) + ((pi_square - 8) << 1)
                | (((1 << pi_square) & RANK_MASK[1]) != 0) as u64;
            i += 1;
            p_at4 = p_at4.blsr();
        }

        //KNIGHT
        let mut copy = game.bn & !(pin_hv | pin_d12);
        while copy != 0 {
            let copy_tzcnt = copy.tzcnt();
            let mut att = KNIGHT_MOVE[copy_tzcnt as usize] & !black & checkmask;
            while att != 0 {
                //legal_moves.push((copy.tzcnt() <<9) + (att.tzcnt()<<1) );
                legal_moves[i] = (copy_tzcnt << 9) + (att.tzcnt() << 1);
                i += 1;
                att = att.blsr();
            }
            copy = copy.blsr();
        }

        //BISHOP
        let mut p = (game.bb | game.bq) & !(pin_hv | pin_d12);
        let mut p1 = (game.bb | game.bq) & pin_d12;
        while p != 0 {
            let p_tzcnt = p.tzcnt();
            let mut att = diag_antid_moves(p_tzcnt, occupied) & !black & checkmask;
            while att != 0 {
                //legal_moves.push((p.tzcnt() <<9) + (att.tzcnt()<<1) );
                legal_moves[i] = (p_tzcnt << 9) + (att.tzcnt() << 1);
                i += 1;
                att = att.blsr();
            }
            p = p.blsr();
        }
        while p1 != 0 {
            let p1_tzcnt = p1.tzcnt();
            let mut att = diag_antid_moves(p1_tzcnt, occupied) & !black & checkmask & pin_d12;
            while att != 0 {
                //legal_moves.push((p1.tzcnt() <<9) + (att.tzcnt()<<1) );
                legal_moves[i] = (p1_tzcnt << 9) + (att.tzcnt() << 1);
                i += 1;
                att = att.blsr();
            }
            p1 = p1.blsr();
        }
        //ROOK
        let mut p = (game.br | game.bq) & !(pin_hv | pin_d12);
        let mut p1 = (game.br | game.bq) & pin_hv;

        while p != 0 {
            let p_tzcnt = p.tzcnt();
            let mut att = hv_moves(p_tzcnt, occupied) & !black & checkmask;
            while att != 0 {
                //legal_moves.push((p.tzcnt() <<9) + (att.tzcnt()<<1) );
                legal_moves[i] = (p_tzcnt << 9) + (att.tzcnt() << 1);
                i += 1;
                att = att.blsr();
            }
            p = p.blsr();
        }
        while p1 != 0 {
            let p1_tzcnt = p1.tzcnt();
            let mut att = hv_moves(p1_tzcnt, occupied) & !black & checkmask & pin_hv;
            while att != 0 {
                //legal_moves.push((p1.tzcnt() <<9) + (att.tzcnt()<<1) );
                legal_moves[i] = (p1_tzcnt << 9) + (att.tzcnt() << 1);
                i += 1;
                att = att.blsr();
            }
            p1 = p1.blsr();
        }
        //KING
        let attack_w = attack_w(game);
        let bk_tzcnt = game.bk.tzcnt();
        let mut p = KING_MOVE[bk_tzcnt as usize] & !attack_w & !black;
        while p != 0 {
            //legal_moves.push((game.bk.tzcnt() <<9) + (p.tzcnt()<<1) );
            legal_moves[i] = (bk_tzcnt << 9) + (p.tzcnt() << 1);
            i += 1;
            p = p.blsr();
        }
        if game.bqueen_castle
            && occupied & (2u64.pow(58) + 2u64.pow(59) + 2u64.pow(57)) == 0
            && attack_w & (2u64.pow(58) + 2u64.pow(59) + 2u64.pow(60)) == 0
        {
            //legal_moves.push((60<<9) + (58<<1));
            legal_moves[i] = (60 << 9) + (58 << 1);
            i += 1;
        }
        if game.bking_castle
            && occupied & (2u64.pow(61) + 2u64.pow(62)) == 0
            && attack_w & (2u64.pow(61) + 2u64.pow(62) + 2u64.pow(60)) == 0
        {
            //legal_moves.push((60<<9) + (62<<1));
            legal_moves[i] = (60 << 9) + (62 << 1);
            i += 1;
        }
    }
    i
}
