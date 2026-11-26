//! Tests de non-régression : perft (génération + exécution des coups)
//! et cohérence du hash Zobrist incrémental.
//! Lancer avec : cargo test --release
use crate::chess::*;
use crate::uci::get_bitboard_from_fen;
use crate::zobrist::init_zobrist_key;

fn fen(f: &str) -> Game {
    let mut v = vec!["position", "fen"];
    v.extend(f.split_ascii_whitespace());
    get_bitboard_from_fen(v)
}

/// perft sur le générateur utilisé par la recherche (get_legal_moves_fast_c),
/// en développant les promotions dans les 4 pièces, et en vérifiant à
/// chaque nœud que le hash incrémental == hash recalculé.
fn perft_c(game: &mut Game, depth: u8) -> u64 {
    assert_eq!(game.hash, init_zobrist_key(game), "hash incrémental faux");
    if depth == 0 {
        return 1;
    }
    let (caps, quiets, c_len, q_len) = get_legal_moves_fast_c(game);
    let moves = caps[..c_len]
        .iter()
        .map(|c| c.1)
        .chain(quiets[..q_len].iter().copied());
    let mut n = 0;
    for m in moves {
        let (a, b, p) = convert_custum_move2(m);
        let proms: &[Piece] = match p {
            Piece::QUEEN => &[Piece::QUEEN, Piece::ROOK, Piece::BISHOP, Piece::KNIGHT],
            _ => &[Piece::NONE],
        };
        for &pr in proms {
            let mut g = *game;
            make_move((a, b, pr), &mut g, game.white_to_play);
            g.white_to_play ^= true;
            n += perft_c(&mut g, depth - 1);
        }
    }
    n
}

fn check(f: &str, expected: &[u64]) {
    for (d, &e) in expected.iter().enumerate() {
        let mut g = fen(f);
        assert_eq!(perft_c(&mut g, d as u8 + 1), e, "{f} profondeur {} (recherche)", d + 1);
        let n = crate::perft::perft(fen(f), d as u8 + 1, &mut Vec::new()) as u64;
        assert_eq!(n, e, "{f} profondeur {} (perft rapide)", d + 1);
    }
}

#[test]
fn perft_startpos() {
    check(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        &[20, 400, 8902, 197281, 4865609],
    );
}
#[test]
fn perft_kiwipete() {
    check(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        &[48, 2039, 97862, 4085603],
    );
}
#[test]
fn perft_pos3() {
    check(
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        &[14, 191, 2812, 43238, 674624],
    );
}
#[test]
fn perft_pos4() {
    check(
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        &[6, 264, 9467, 422333],
    );
}
#[test]
fn perft_pos5() {
    check(
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        &[44, 1486, 62379, 2103487],
    );
}
#[test]
fn perft_pos6() {
    check(
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        &[46, 2079, 89890, 3894594],
    );
}
