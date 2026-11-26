use crate::chess::*;

/// perft sur le générateur utilisé par la recherche (get_legal_moves_fast_c).
/// Les promotions sont développées en dame/tour/fou/cavalier pour pouvoir
/// comparer aux valeurs de référence.
pub fn perft(game: Game, depth: u8, _legal_moves_tab: &mut Vec<[u64; 70]>) -> usize {
    // un buffer par profondeur, alloué une seule fois
    let mut bufs = vec![[0u64; MAX_MOVES_FAST]; depth as usize + 1];
    perft_rec(game, depth, &mut bufs)
}

/// perft rapide sur get_legal_moves_fast (liste unique, sans tri des prises).
/// Le générateur de la recherche (get_legal_moves_fast_c) est vérifié par
/// `cargo test --release`.
pub fn perft_rec(mut game: Game, depth: u8, bufs: &mut [[u64; MAX_MOVES_FAST]]) -> usize {
    if depth == 0 {
        return 1;
    }
    let (moves, rest) = bufs.split_last_mut().unwrap();
    let len = get_legal_moves_fast(&mut game, moves);
    if depth == 1 {
        // comptage en bloc : chaque promotion compte pour 4 coups
        let pawns_7th = if game.white_to_play { game.wp & RANK_MASK[6] } else { game.bp & RANK_MASK[1] };
        if pawns_7th == 0 {
            return len;
        }
        let promos = moves[..len].iter().filter(|&&m| m & 1 == 1).count();
        return len + 3 * promos;
    }
    let mut nb_nodes = 0;
    for &m in &moves[..len] {
        let (a, b, p) = convert_custum_move2(m);
        let proms: &[Piece] = match p {
            Piece::QUEEN => &[Piece::QUEEN, Piece::ROOK, Piece::BISHOP, Piece::KNIGHT],
            _ => &[Piece::NONE],
        };
        for &pr in proms {
            let mut game1 = game;
            make_move_nohash((a, b, pr), &mut game1, game.white_to_play);
            game1.white_to_play ^= true;
            nb_nodes += perft_rec(game1, depth - 1, rest);
        }
    }
    nb_nodes
}
/*pub fn perft_divide(mut game: Game, depth: i8) {
    let (legal_moves, _len) = get_legal_moves_fast(&mut game);
    let mut nb_nodes_tot = 0;
    for moveto in legal_moves {
        if moveto == 0 {
            break;
        }
        let mut game1 = game;
        let moves = convert_custum_move2(moveto);
        if game.white_to_play {
            compute_move_w_thrust(moves, &mut game1);
        } else {
            compute_move_b_thrust(moves, &mut game1);
        }
        game1.white_to_play ^= true;
        let nb_nodes = perft(game1, depth - 1);
        nb_nodes_tot += nb_nodes;
        println!(
            "{} {}",
            convert_move_to_str(moves.0, moves.1, moves.2),
            nb_nodes
        );
    }
    println!();
    println!("{nb_nodes_tot}");
    //println!("Nodes searched: {nb_nodes_tot}");
}*/
/*
pub fn _perft_hash(mut game: Game, depth : i8, tt : &mut TranspositionTable) -> usize {
    let mut nb_nodes = 0;

    let (legal_moves, i) = get_legal_moves_fast(&mut game);

    if depth == 1 || i == 0 {
        return i;
    }
    if depth == 0 {
        return 1;
    }
    /*let tt_entry = tt.get(game.hash);
    if tt_entry.hash == game.hash {
        if tt_entry.depth >= depth {
            return tt_entry.eval;
        }
    }*/
    for moveto in legal_moves {
        let mut game1 = game;
        //let moves = convert_custum_move(moveto);
        let moves = convert_custum_move2(moveto);
        if game.white_to_play {
            compute_move_w_hash(moves, &mut game1);
        }
        else {
            compute_move_b_hash(moves, &mut game1);
        }
        game1.white_to_play^=true;
        nb_nodes += _perft_hash(game1, depth-1, tt);
    }
    nb_nodes
}
*/
