use crate::chess::*;
use crate::eval::*;
use crate::search_tools::SearchTools;
//use crate::table_transposition::Transposition;
use crate::table_transposition::NodeType;
use crate::table_transposition::TranspositionTable;
use std::cmp::{max, min};
use std::sync::atomic::Ordering;

pub fn _alpha_beta(
    game: &mut Game,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    nb_node: &mut u64,
) -> i32 {
    *nb_node += 1;
    let legal_move = get_legal_move(game.white_to_play, game);
    if depth == 0 || legal_move.is_empty() {
        return eval(game, legal_move.len() as i32);
    };
    let mut value;
    if !game.white_to_play {
        value = i32::MAX;
        for moveto in legal_move {
            let (a, b, prom) = convert_custum_move(moveto);
            let mut game1 = *game;

            game1.white_to_play ^= true;
            compute_move_b((a, b, prom), &mut game1);
            value = min(
                value,
                _alpha_beta(&mut game1, depth - 1, alpha, beta, nb_node),
            );
            if alpha >= value {
                return value;
            }
            beta = min(beta, value);
        }
    } else {
        value = i32::MIN;
        for moveto in legal_move {
            let (a, b, prom) = convert_custum_move(moveto);
            let mut game1 = *game;
            compute_move_w((a, b, prom), &mut game1);
            game1.white_to_play ^= true;

            value = max(
                value,
                _alpha_beta(&mut game1, depth - 1, alpha, beta, nb_node),
            );
            if value >= beta {
                return value;
            }
            alpha = max(alpha, value)
        }
    }
    value
}

pub fn _minimax(game: &mut Game, depth: u8, maximizing_player: bool, nb_node: &mut u64) -> i32 {
    let legal_moves = get_legal_move(game.white_to_play, game);
    *nb_node += 1;
    if depth == 0 || legal_moves.is_empty() {
        return eval(game, legal_moves.len() as i32);
    };
    let mut value;

    if maximizing_player {
        value = i32::MIN;
        for _moveto in legal_moves {
            let (a, b, prom) = convert_custum_move(_moveto);
            let mut game1 = *game;
            compute_move_w((a, b, prom), &mut game1);

            game1.white_to_play ^= true;
            value = max(value, _minimax(&mut game1, depth - 1, false, nb_node));
        }
    } else {
        value = i32::MAX;
        for _moveto in legal_moves {
            let (a, b, prom) = convert_custum_move(_moveto);
            let mut game1 = *game;
            compute_move_b((a, b, prom), &mut game1);

            game1.white_to_play ^= true;
            value = min(value, _minimax(&mut game1, depth - 1, true, nb_node));
        }
    }
    value
}

pub fn _pvs(game: &mut Game, depth: u8, mut alpha: i32, beta: i32, nb_node: &mut u64) -> i32 {
    let legal_moves = get_legal_move(game.white_to_play, game);
    *nb_node += 1;
    if depth == 0 || legal_moves.is_empty() {
        let mut eval = eval(game, legal_moves.len() as i32);
        if !game.white_to_play {
            eval *= -1;
        };
        return eval;
    };
    let mut first = true;
    let mut score = 0;
    for moveto in legal_moves {
        let (a, b, prom) = convert_custum_move(moveto);
        let mut game1 = *game;
        game1.white_to_play ^= true;
        if game.white_to_play {
            compute_move_w_hash((a, b, prom), &mut game1);
        } else {
            compute_move_b_hash((a, b, prom), &mut game1);
        }
        if first {
            first = false;
            score = -_pvs(&mut game1, depth - 1, -beta, -alpha, nb_node);
        } else {
            score = -_pvs(&mut game1, depth - 1, -alpha - 1, -alpha, nb_node);
            if alpha < score && score < beta {
                score = -_pvs(&mut game1, depth - 1, -beta, -score, nb_node);
            }
        }
        alpha = max(alpha, score);
        if alpha > beta {
            break;
        }
    }
    alpha
}

pub fn _pvs_tt(
    game: &mut Game,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    nb_node: &mut u64,
    tt: &mut TranspositionTable,
) -> i32 {
    let alpha_orgi = alpha;
    let mut hash_move = 0;
    let tt_entry = tt.get(game.hash);
    if tt_entry.hash == game.hash && tt_entry.depth >= depth {
        match tt_entry.node_type {
            NodeType::PV => return tt_entry.eval,
            NodeType::ALL => alpha = max(alpha, tt_entry.eval),
            NodeType::CUT => {
                beta = min(beta, tt_entry.eval);
                hash_move = tt_entry.bestmove;
            }
        }
        if alpha >= beta {
            return tt_entry.eval;
        }
    }
    let legal_moves = get_legal_move(game.white_to_play, game);
    *nb_node += 1;
    if depth == 0 || legal_moves.is_empty() {
        let mut eval = eval(game, legal_moves.len() as i32);
        if !game.white_to_play {
            eval *= -1;
        };
        return eval;
    };
    /*if hash_move != 0 {
        legal_moves.push_front((hash_move, Piece::NONE));
    }*/
    let mut first = true;
    let mut score = 0;
    let mut bestmove = 0;
    for moveto in legal_moves {
        let (a, b, prom) = convert_custum_move(moveto);
        let mut game1 = *game;
        game1.white_to_play ^= true;
        if game.white_to_play {
            compute_move_w_hash((a, b, prom), &mut game1);
        } else {
            compute_move_b_hash((a, b, prom), &mut game1);
        }
        if first {
            first = false;
            score = -_pvs_tt(&mut game1, depth - 1, -beta, -alpha, nb_node, tt);
            bestmove = moveto.0;
        } else {
            score = -_pvs_tt(&mut game1, depth - 1, -alpha - 1, -alpha, nb_node, tt);

            if alpha < score && score < beta {
                bestmove = moveto.0;
                score = -_pvs_tt(&mut game1, depth - 1, -beta, -score, nb_node, tt);
            }
        }
        if score > alpha {
            bestmove = moveto.0;
            alpha = score;
        }
        //alpha = max(alpha, score);
        if alpha > beta {
            break;
        }
    }
    let node_t;
    if score <= alpha_orgi {
        node_t = NodeType::CUT;
    } else if score >= beta {
        node_t = NodeType::ALL;
    } else {
        node_t = NodeType::PV;
    }
    tt.set(game.hash, depth, score, bestmove, node_t);

    alpha
}
/*pub fn _pvs_tt_best(game : &mut Game, depth : u8, mut alpha:i32, mut beta : i32, nb_node : &mut u64, tt : &mut TranspositionTable, first_move:u64) -> i32 {
    let alpha_orgi = alpha;
    let mut hash_move = 0;
    let tt_entry = tt.get(game.hash);
    if tt_entry.hash == game.hash && tt_entry.depth >= depth {
        match tt_entry.node_type {
            NodeType::PV =>  return tt_entry.eval,
            NodeType::ALL => alpha = max(alpha, tt_entry.eval),
            NodeType::CUT => { beta = min(beta, tt_entry.eval); hash_move = tt_entry.bestmove;}
        }
        if alpha >= beta {
            return tt_entry.eval;
        }
    }
    let (mut captures, legal_moves, score_move) = get_legal_moves_fast_c(game);
    *nb_node+=1;
    if depth == 0 || legal_moves.is_empty() {
        let mut eval = eval(game, legal_moves.len() as i32);
        if !game.white_to_play {
            eval *= -1;
        };
        return eval;
    };
    /*if hash_move != 0 {
        legal_moves.push_front((hash_move, Piece::NONE));
    }
    if first_move != 0 {
        legal_moves.push_front((first_move, Piece::NONE));
    }*/
    sort_move(&mut captures, score_move);
    let mut first = true;
    let mut score = 0;
    let mut bestmove = 0;
    for moveto in legal_moves {
        let (a, b, prom) = convert_custum_move2(moveto);
        let mut game1 = *game;
        game1.white_to_play ^= true;
        if game.white_to_play {
            compute_move_w_hash((a, b, prom), &mut game1);
        } else {
            compute_move_b_hash((a, b, prom), &mut game1);
        }
        if first {
            first = false;
            score = -_pvs_tt_best(&mut game1, depth- 1, -beta, -alpha, nb_node, tt, 0);
            bestmove = moveto;
        }
        else {
            score = -_pvs_tt_best(&mut game1, depth- 1, -alpha-1, -alpha, nb_node, tt, 0);

            if alpha < score && score < beta {
                bestmove = moveto;
                score = -_pvs(&mut game1, depth- 1, -beta, -score, nb_node);
            }
        }
        if score > alpha {
            bestmove = moveto;
            alpha = score;
        }
        //alpha = max(alpha, score);
        if alpha > beta {
            break;
        }
    }
    let node_t;
    if score <= alpha_orgi {
        node_t = NodeType::CUT;
    }
    else if score >= beta {
        node_t = NodeType::ALL;
    }
    else {
        node_t = NodeType::PV;
    }
    tt.set(game.hash, depth, score, bestmove , node_t);

    alpha
}
pub fn _pvs_tt_best_root(game : &mut Game, depth : u8, mut alpha:i32, mut beta : i32, nb_node : &mut u64, tt : &mut TranspositionTable, first_move:u64) -> i32 {
    let alpha_orgi = alpha;
    let mut hash_move = 0;
    let tt_entry = tt.get(game.hash);
    if tt_entry.hash == game.hash && tt_entry.depth >= depth {
        match tt_entry.node_type {
            NodeType::PV =>  return tt_entry.eval,
            NodeType::ALL => alpha = max(alpha, tt_entry.eval),
            NodeType::CUT => { beta = min(beta, tt_entry.eval); hash_move = tt_entry.bestmove;}
        }
        if alpha >= beta {
            return tt_entry.eval;
        }
    }
    let (mut captures, legal_moves, score_move) = get_legal_moves_fast_c(game);
    *nb_node+=1;
    if depth == 0 || legal_moves.is_empty() {
        let mut eval = eval(game, legal_moves.len() as i32);
        if !game.white_to_play {
            eval *= -1;
        };
        return eval;
    };
    /*if hash_move != 0 {
        legal_moves.push_front((hash_move, Piece::NONE));
    }
    if first_move != 0 {
        legal_moves.push_front((first_move, Piece::NONE));
    }*/
    sort_move(&mut captures, score_move);
    let mut first = true;
    let mut score = 0;
    let mut bestmove = 0;
    for moveto in legal_moves {
        let (a, b, prom) = convert_custum_move2(moveto);
        let mut game1 = *game;
        game1.white_to_play ^= true;
        if game.white_to_play {
            compute_move_w_hash((a, b, prom), &mut game1);
        } else {
            compute_move_b_hash((a, b, prom), &mut game1);
        }
        if first {
            first = false;
            score = -_pvs_tt_best(&mut game1, depth- 1, -beta, -alpha, nb_node, tt, 0);
            bestmove = moveto;
        }
        else {
            score = -_pvs_tt_best(&mut game1, depth- 1, -alpha-1, -alpha, nb_node, tt, 0);

            if alpha < score && score < beta {
                bestmove = moveto;
                score = -_pvs(&mut game1, depth- 1, -beta, -score, nb_node);
            }
        }
        if score > alpha {
            bestmove = moveto;
            alpha = score;
        }
        //alpha = max(alpha, score);
        if alpha > beta {
            break;
        }
    }
    let node_t;
    if score <= alpha_orgi {
        node_t = NodeType::CUT;
    }
    else if score >= beta {
        node_t = NodeType::ALL;
    }
    else {
        node_t = NodeType::PV;
    }
    tt.set(game.hash, depth, score, bestmove , node_t);

    alpha
}*/
pub fn _alpha_beta_neg(
    game: &Game,
    depth: u8,
    mut alpha: i32,
    beta: i32,
    nb_node: &mut u64,
) -> i32 {
    let legal_moves = get_legal_move(game.white_to_play, game);
    *nb_node += 1;
    if depth == 0 || legal_moves.is_empty() {
        let mut eval = eval(game, legal_moves.len() as i32);
        if !game.white_to_play {
            eval *= -1;
        };
        return eval;
    };
    let mut value = i32::MIN >> 1;
    for moveto_play in legal_moves {
        let (a, b, prom) = convert_custum_move(moveto_play);

        let mut game1 = *game;

        if game.white_to_play {
            compute_move_w((a, b, prom), &mut game1);
        } else {
            compute_move_b((a, b, prom), &mut game1);
        }
        game1.white_to_play ^= true;

        value = max(
            value,
            -_alpha_beta_neg(&mut game1, depth - 1, -beta, -alpha, nb_node),
        );
        /*if value >= beta {
            return value;
        }*/
        alpha = max(alpha, value);
        if alpha >= beta {
            break;
        }
    }
    value
}

pub fn _alpha_beta_neg_tt(
    game: &Game,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    tt: &mut TranspositionTable,
    nb_node: &mut u64,
) -> i32 {
    *nb_node += 1;
    let alpha_orgi = alpha;
    /* TT look up */

    let tt_entry = tt.get(game.hash);
    if tt_entry.hash == game.hash && tt_entry.depth >= depth {
        match tt_entry.node_type {
            NodeType::PV => return tt_entry.eval,
            NodeType::ALL => alpha = max(alpha, tt_entry.eval),
            NodeType::CUT => beta = min(beta, tt_entry.eval),
        }
        if alpha >= beta {
            return tt_entry.eval;
        }
    }
    let legal_moves = get_legal_move(game.white_to_play, game);
    if depth == 0 || legal_moves.is_empty() {
        let mut eval = eval(game, legal_moves.len() as i32);
        if !game.white_to_play {
            eval *= -1;
        };
        return eval;
    };
    let mut value = i32::MIN >> 1;
    for moveto_play in legal_moves {
        let (a, b, prom) = convert_custum_move(moveto_play);

        let mut game1 = *game;

        if game.white_to_play {
            compute_move_w_hash((a, b, prom), &mut game1);
        } else {
            compute_move_b_hash((a, b, prom), &mut game1);
        }
        game1.white_to_play ^= true;
        value = max(
            value,
            -_alpha_beta_neg_tt(&mut game1, depth - 1, -beta, -alpha, tt, nb_node),
        );
        alpha = max(alpha, value);
        if alpha >= beta {
            break;
        }
    }

    //tt_entry.eval = value;
    let node_t;
    if value <= alpha_orgi {
        node_t = NodeType::CUT;
    } else if value >= beta {
        node_t = NodeType::ALL;
    } else {
        node_t = NodeType::PV;
    }
    tt.set(game.hash, depth, value, 0, node_t);

    value
}
pub fn _alpha_beta_neg_tt_best(
    game: &Game,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    tt: &mut TranspositionTable,
    nb_node: &mut u64,
    first: u64,
) -> (i32, u64) {
    *nb_node += 1;
    let alpha_orgi = alpha;
    let mut hash_move = 0u64;
    /* TT look up */
    let mut best_move = 0u64;
    let tt_entry = tt.get(game.hash);
    if tt_entry.hash == game.hash {
        if tt_entry.depth >= depth {
            match tt_entry.node_type {
                NodeType::PV => return (tt_entry.eval, tt_entry.bestmove),
                NodeType::ALL => alpha = max(alpha, tt_entry.eval),
                NodeType::CUT => {
                    beta = min(beta, tt_entry.eval);
                }
            }
            if alpha >= beta {
                return (tt_entry.eval, tt_entry.bestmove);
            }
        }
        hash_move = tt_entry.bestmove;
    }

    let mut legal_moves = get_legal_move(game.white_to_play, game);

    if depth == 0 || legal_moves.is_empty() {
        let mut eval = eval(game, legal_moves.len() as i32);
        //eval *= -1 * !game.white_to_play as i32;
        if !game.white_to_play {
            eval *= -1;
        };
        return (eval, 0);
    };

    //println!("{} {}", hash_moves.len() , legal_moves.len());
    let mut value = i32::MIN >> 1;
    if hash_move != 0 {
        legal_moves.push_front((hash_move, Piece::NONE));
    }
    if first != 0 {
        legal_moves.push_front((first, Piece::NONE));
        //eprintln!("Hello {}", convert_custum_to_str(first));
    }
    let mut i = 0;
    for moveto_play in legal_moves {
        if (hash_move == moveto_play.0 || first == moveto_play.0) && i > 2 {
            continue;
        }

        //eprintln!("MOVE : {}", convert_custum_to_str(moveto_play.0));
        let (a, b, prom) = convert_custum_move(moveto_play);

        let mut game1 = *game;

        if game.white_to_play {
            compute_move_w_hash((a, b, prom), &mut game1);
        } else {
            compute_move_b_hash((a, b, prom), &mut game1);
        }
        game1.white_to_play ^= true;
        let (score, _b_move) =
            _alpha_beta_neg_tt_best(&mut game1, depth - 1, -beta, -alpha, tt, nb_node, 0);
        if value < -score {
            value = -score;
            best_move = moveto_play.0;
        }
        alpha = max(alpha, value);
        //value = max(value, );
        if alpha >= beta {
            break;
        }
        i += 1;
    }
    let node_t;
    if value <= alpha_orgi {
        node_t = NodeType::CUT;
    } else if value >= beta {
        node_t = NodeType::ALL;
    } else {
        node_t = NodeType::PV;
    }
    tt.set(game.hash, depth, value, best_move, node_t);
    //eprintln!("{}", convert_custum_to_str(best_move));
    (value, best_move)
}
pub fn alpha_beta_neg_tt_best_time(
    game: &Game,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    tool: &mut SearchTools,
    nb_node: &mut u64,
    first: u64,
) -> (Option<i32>, u64) {
    if tool.timeover.load(Ordering::Relaxed) {
        return (None, 0);
    }
    *nb_node += 1;
    let alpha_orgi = alpha;
    let mut hash_move = 0u64;
    /* TT look up */
    let mut best_move = 0u64;
    let tt_entry = tool.tt.get(game.hash);
    if tt_entry.hash == game.hash {
        if tt_entry.depth >= depth {
            match tt_entry.node_type {
                NodeType::PV => return (Some(tt_entry.eval), tt_entry.bestmove),
                NodeType::ALL => alpha = max(alpha, tt_entry.eval),
                NodeType::CUT => {
                    beta = min(beta, tt_entry.eval);
                }
            }
            if alpha >= beta {
                return (Some(tt_entry.eval), tt_entry.bestmove);
            }
        }
        hash_move = tt_entry.bestmove;
    }

    //let mut legal_moves = get_legal_moves_fast(game);
    let mut legal_moves = get_legal_move(game.white_to_play, game);

    if depth == 0 || legal_moves.is_empty() {
        let mut eval = eval(game, legal_moves.len() as i32);
        //eval *= -1 * !game.white_to_play as i32;
        if !game.white_to_play {
            eval *= -1;
        };
        return (Some(eval), 0);
    };

    //println!("{} {}", hash_moves.len() , legal_moves.len());
    let mut value = i32::MIN >> 1;
    if hash_move != 0 {
        legal_moves.push_front((hash_move, Piece::NONE));
        //legal_moves.push_front(hash_move);
    }
    if first != 0 {
        legal_moves.push_front((first, Piece::NONE));
        //eprintln!("Hello {}", convert_custum_to_str(first));
    }
    let mut i = 0;
    for moveto_play in legal_moves {
        if (hash_move == moveto_play.0 || first == moveto_play.0) && i > 2 {
            //if (hash_move == moveto_play || first == moveto_play) && i > 2 {
            continue;
        }

        //eprintln!("MOVE : {}", convert_custum_to_str(moveto_play.0));
        let (a, b, prom) = convert_custum_move(moveto_play);

        let mut game1 = *game;

        if game.white_to_play {
            compute_move_w_hash((a, b, prom), &mut game1);
        } else {
            compute_move_b_hash((a, b, prom), &mut game1);
        }
        game1.white_to_play ^= true;
        let (s, _b_move) =
            alpha_beta_neg_tt_best_time(&mut game1, depth - 1, -beta, -alpha, tool, nb_node, 0);
        match s {
            Some(score) => {
                if value < -score {
                    value = -score;
                    best_move = moveto_play.0;
                }
                alpha = max(alpha, value);
                //value = max(value, );
                if alpha >= beta {
                    break;
                }
                i += 1;
            }
            None => return (None, 0),
        }
    }
    let node_t;
    if value <= alpha_orgi {
        node_t = NodeType::CUT;
    } else if value >= beta {
        node_t = NodeType::ALL;
    } else {
        node_t = NodeType::PV;
    }
    tool.tt.set(game.hash, depth, value, best_move, node_t);
    //eprintln!("{}", convert_custum_to_str(best_move));
    (Some(value), best_move)
}
const INF: i32 = i32::MAX >> 1;
const MAX_QPLY: u8 = 16;
const MAX_MOVES: usize = 160;

/// Coups ordonnés : coup de la TT, captures (MVV-LVA décroissant), coups calmes.
/// Le coup de la TT n'est joué que s'il est légal dans cette position
/// (collision ou index partagé => on l'ignore au lieu de jouer un coup bidon).
#[inline(always)]
fn ordered_moves(
    game: &mut Game,
    hash_move: u64,
    quiets_too: bool,
) -> ([u64; MAX_MOVES], usize, usize) {
    let (mut caps, quiets, c_len, q_len) = get_legal_moves_fast_c(game);
    let mut out = [0u64; MAX_MOVES];
    let mut n = 0;
    let legal = c_len + q_len;
    let hm_ok = hash_move != 0
        && (caps[..c_len].iter().any(|c| c.1 == hash_move) || quiets[..q_len].contains(&hash_move));
    if hm_ok {
        out[0] = hash_move;
        n = 1;
    }
    caps[..c_len].sort_unstable_by(|a, b| b.0.cmp(&a.0));
    for &(_, m) in &caps[..c_len] {
        if !hm_ok || m != hash_move {
            out[n] = m;
            n += 1;
        }
    }
    for &m in &quiets[..q_len] {
        // en quiescence hors échec : seulement les promotions calmes
        if (quiets_too || m & 1 == 1) && (!hm_ok || m != hash_move) {
            out[n] = m;
            n += 1;
        }
    }
    (out, n, legal)
}

#[inline(always)]
fn play(game: &Game, m: u64) -> Game {
    let mut g = *game;
    make_move(convert_custum_move2(m), &mut g, game.white_to_play);
    g.white_to_play ^= true;
    g
}

/// Negamax fail-soft + TT. Tous les scores sont du point de vue du camp au trait.
pub fn alpha_beta_neg_tt_best_time_fast(
    game: &mut Game,
    depth: u8,
    mut alpha: i32,
    beta: i32,
    tool: &mut SearchTools,
    nb_node: &mut u64,
    _first: u64,
) -> (Option<i32>, u64) {
    if tool.timeover.load(Ordering::Relaxed) {
        return (None, 0);
    }
    if depth == 0 {
        // avant : eval du point de vue de l'ADVERSAIRE (trait inversé) et pas de quiescence
        return (quiescence(game, alpha, beta, tool, nb_node, 0), 0);
    }
    *nb_node += 1;
    let alpha_orig = alpha;
    let mut hash_move = 0u64;
    let tt_entry = *tool.tt.get(game.hash);
    if tt_entry.hash == game.hash {
        if tt_entry.depth >= depth {
            // CUT = borne inférieure, ALL = borne supérieure
            match tt_entry.node_type {
                NodeType::PV => return (Some(tt_entry.eval), tt_entry.bestmove),
                NodeType::CUT if tt_entry.eval >= beta => {
                    return (Some(tt_entry.eval), tt_entry.bestmove);
                }
                NodeType::ALL if tt_entry.eval <= alpha => return (Some(tt_entry.eval), 0),
                _ => {}
            }
        }
        hash_move = tt_entry.bestmove;
    }

    let (moves, n, legal) = ordered_moves(game, hash_move, true);
    if legal == 0 {
        return (Some(eval(game, 0)), 0); // mat ou pat
    }

    let mut value = -INF;
    let mut best_move = 0u64;
    for &m in &moves[..n] {
        let mut child = play(game, m);
        let (s, _) = alpha_beta_neg_tt_best_time_fast(
            &mut child,
            depth - 1,
            -beta,
            -alpha,
            tool,
            nb_node,
            0,
        );
        let Some(score) = s else { return (None, 0) };
        let score = -score;
        if score > value {
            value = score;
            best_move = m;
        }
        alpha = max(alpha, value);
        if alpha >= beta {
            break;
        }
    }

    let node_t = if value <= alpha_orig {
        NodeType::ALL
    } else if value >= beta {
        NodeType::CUT
    } else {
        NodeType::PV
    };
    let stored_move = if node_t == NodeType::ALL {
        hash_move
    } else {
        best_move
    };
    tool.tt.set(game.hash, depth, value, stored_move, node_t);
    (Some(value), best_move)
}

pub fn mtd_f(
    game: &mut Game,
    f: i32,
    depth: u8,
    tool: &mut SearchTools,
    nb_node: &mut u64,
    first: u64,
) -> (Option<i32>, u64) {
    let mut g = f;
    let mut bmove = first;
    let mut upperbound = INF;
    let mut lowerbound = -INF;
    let mut pass = 0;
    while lowerbound < upperbound {
        // MTD(f) classique autour de la première estimation (avant : la
        // bissection démarrait à 0 et ignorait f), bissection si ça traîne.
        let beta = if pass < 6 {
            if g == lowerbound { g + 1 } else { g }
        } else {
            lowerbound + (upperbound - lowerbound + 1) / 2
        };
        pass += 1;
        let (x, i) =
            alpha_beta_neg_tt_best_time_fast(game, depth, beta - 1, beta, tool, nb_node, bmove);
        let Some(s) = x else { return (None, 0) };
        g = s;
        if g < beta {
            upperbound = g;
        } else {
            lowerbound = g;
            // seul un fail-high donne un meilleur coup fiable
            if i != 0 {
                bmove = i;
            }
        }
    }
    (Some(g), bmove)
}

/// Recherche de quiescence fail-soft : captures et promotions, toutes les
/// parades si le camp au trait est en échec (pas de stand-pat en échec).
fn quiescence(
    game: &mut Game,
    mut alpha: i32,
    beta: i32,
    tool: &mut SearchTools,
    nb_node: &mut u64,
    qply: u8,
) -> Option<i32> {
    if tool.timeover.load(Ordering::Relaxed) {
        return None;
    }
    *nb_node += 1;
    let in_check = is_attacked(game.white_to_play, game);
    let (moves, n, legal) = ordered_moves(game, 0, in_check);
    if legal == 0 {
        return Some(eval(game, 0));
    }
    let mut best = -INF;
    if !in_check {
        let stand_pat = eval(game, legal as i32);
        if stand_pat >= beta || qply >= MAX_QPLY {
            return Some(stand_pat);
        }
        best = stand_pat;
        alpha = max(alpha, stand_pat);
    } else if qply >= MAX_QPLY {
        return Some(eval(game, legal as i32));
    }
    for &m in &moves[..n] {
        let mut child = play(game, m);
        let score = -quiescence(&mut child, -beta, -alpha, tool, nb_node, qply + 1)?;
        if score > best {
            best = score;
            if score > alpha {
                alpha = score;
                if alpha >= beta {
                    break;
                }
            }
        }
    }
    Some(best)
}
