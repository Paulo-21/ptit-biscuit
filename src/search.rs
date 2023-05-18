use crate::chess::*;
use crate::eval::*;
//use crate::table_transposition::Transposition;
use crate::table_transposition::TranspositionTable;
use crate::table_transposition::node_type;
use std::cmp::{ max, min };
use std::hash;

pub fn alpha_beta(game : &mut Game, depth : i8, mut alpha:i32, mut beta : i32, nb_node : &mut u64) -> i32 {
    *nb_node+=1;
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
            value = min(value, alpha_beta(&mut game1, depth-1, alpha, beta, nb_node));
            if alpha >= value {
                return value;
            }
            beta = min(beta, value);
        }
    }
    else {
        value = i32::MIN;
        for moveto in legal_move {
            let (a,b, prom) = convert_custum_move(moveto);
            let mut game1 = *game;
            compute_move_w((a, b, prom), &mut game1);
            game1.white_to_play ^= true;
            
            value = max(value, alpha_beta(&mut game1, depth-1, alpha, beta, nb_node));
            if value >= beta {
                return value;
            }
            alpha = max(alpha, value)
        }
    }
    value
}

pub fn minimax(game: &mut Game, depth : i8, maximizing_player : bool, nb_node : &mut u64) -> i32 {
    let legal_moves = get_legal_move(game.white_to_play, game);
    *nb_node+=1;
    if depth == 0 || legal_moves.is_empty() {
        return eval(game, legal_moves.len() as i32);
    };
    let mut value;

    if maximizing_player {
        value = i32::MIN;
        for _moveto in legal_moves {
            let (a,b, prom) = convert_custum_move(_moveto);
            let mut game1 = *game;
            compute_move_w((a, b, prom), &mut game1);
            
            game1.white_to_play ^= true;
            value = max(value, minimax(&mut game1, depth-1, false, nb_node));
        }
    }
    else {
        value = i32::MAX;
        for _moveto in legal_moves {
            let (a,b, prom) = convert_custum_move(_moveto);
            let mut game1 = *game;
            compute_move_b((a, b,prom), &mut game1);

            game1.white_to_play ^= true;
            value = min(value, minimax(&mut game1, depth-1, true, nb_node));
        }
    }
    value
}

pub fn pvs(game : &mut Game, depth : i8, mut alpha:i32, mut beta : i32, nb_node : &mut u64) -> i32 {
    let legal_moves = get_legal_move(game.white_to_play, game);
    *nb_node+=1;
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
            score = -pvs(&mut game1, depth- 1, -beta, -alpha, nb_node);
        }
        else {
            score = -pvs(&mut game1, depth- 1, -alpha-1, -alpha, nb_node);
            if alpha < score && score < beta {
                score = -pvs(&mut game1, depth- 1, -beta, -score, nb_node);
            }
        }
        alpha = max(alpha, score);
        if alpha > beta {
            break;
        }
    }
    alpha
}

pub fn pvs_tt(game : &mut Game, depth : i8, mut alpha:i32, mut beta : i32, nb_node : &mut u64, tt : &mut TranspositionTable) -> i32 {
    let alpha_orgi = alpha;
    let mut hash_move = 0;
    let tt_entry = tt.get(game.hash);
    if tt_entry.hash == game.hash && tt_entry.depth >= depth {
        match tt_entry.node_type {
            node_type::PV =>  return tt_entry.eval,
            node_type::ALL => alpha = max(alpha, tt_entry.eval),
            node_type::CUT => { beta = min(beta, tt_entry.eval); hash_move = tt_entry.bestmove;}
        }
        if alpha >= beta {
            return tt_entry.eval;
        }
    }
    let mut legal_moves = get_legal_move(game.white_to_play, game);
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
            score = -pvs_tt(&mut game1, depth- 1, -beta, -alpha, nb_node, tt);
            bestmove = moveto.0;
        }
        else {
            score = -pvs_tt(&mut game1, depth- 1, -alpha-1, -alpha, nb_node, tt);
            
            if alpha < score && score < beta {
                bestmove = moveto.0;
                score = -pvs_tt(&mut game1, depth- 1, -beta, -score, nb_node, tt);
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
        node_t = node_type::CUT;
    }
    else if score >= beta {
        node_t = node_type::ALL;
    }
    else {
        node_t = node_type::PV;
    }
    tt.set(game.hash, depth, score, bestmove , node_t);

    alpha
}
pub fn pvs_tt_best(game : &mut Game, depth : i8, mut alpha:i32, mut beta : i32, nb_node : &mut u64, tt : &mut TranspositionTable, first_move:u64) -> i32 {
    let alpha_orgi = alpha;
    let mut hash_move = 0;
    let tt_entry = tt.get(game.hash);
    if tt_entry.hash == game.hash && tt_entry.depth >= depth {
        match tt_entry.node_type {
            node_type::PV =>  return tt_entry.eval,
            node_type::ALL => alpha = max(alpha, tt_entry.eval),
            node_type::CUT => { beta = min(beta, tt_entry.eval); hash_move = tt_entry.bestmove;}
        }
        if alpha >= beta {
            return tt_entry.eval;
        }
    }
    let mut legal_moves = get_legal_move(game.white_to_play, game);
    *nb_node+=1;
    if depth == 0 || legal_moves.is_empty() {
        let mut eval = eval(game, legal_moves.len() as i32);
        if !game.white_to_play {
            eval *= -1;
        };
        return eval;
    };
    if hash_move != 0 {
        legal_moves.push_front((hash_move, Piece::NONE));
    }
    if first_move != 0 {
        legal_moves.push_front((first_move, Piece::NONE));
    }
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
            score = -pvs_tt_best(&mut game1, depth- 1, -beta, -alpha, nb_node, tt, 0);
            bestmove = moveto.0;
        }
        else {
            score = -pvs_tt_best(&mut game1, depth- 1, -alpha-1, -alpha, nb_node, tt, 0);
            
            if alpha < score && score < beta {
                bestmove = moveto.0;
                score = -pvs(&mut game1, depth- 1, -beta, -score, nb_node);
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
        node_t = node_type::CUT;
    }
    else if score >= beta {
        node_t = node_type::ALL;
    }
    else {
        node_t = node_type::PV;
    }
    tt.set(game.hash, depth, score, bestmove , node_t);

    alpha
}
pub fn alpha_beta_neg(game: &Game, depth : i8, mut alpha : i32, mut beta : i32, nb_node : &mut u64) -> i32 {
    let legal_moves = get_legal_move(game.white_to_play, game);
    *nb_node+=1;
    if depth == 0 || legal_moves.is_empty() {
        let mut eval = eval(game, legal_moves.len() as i32);
        if !game.white_to_play {
            eval *= -1;
        };
        return eval;
    };
    let mut value = i32::MIN>>1;
    for moveto_play in legal_moves {
        let (a,b, prom) = convert_custum_move(moveto_play);

        let mut game1 = *game;

        if game.white_to_play { compute_move_w((a, b, prom), &mut game1); }
        else { compute_move_b((a, b, prom), &mut game1); }
        game1.white_to_play^=true;
        
        value = max(value, -alpha_beta_neg(&mut game1, depth-1, -beta, -alpha, nb_node));
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

pub fn alpha_beta_neg_tt(game: &Game, depth : i8, mut alpha : i32, mut beta : i32, tt : &mut TranspositionTable, nb_node : &mut u64) -> i32 {
    *nb_node+=1;
    let alpha_orgi = alpha;
    /* TT look up */

    let tt_entry = tt.get(game.hash);
    if tt_entry.hash == game.hash && tt_entry.depth >= depth {
        match tt_entry.node_type {
            node_type::PV =>  return tt_entry.eval,
            node_type::ALL => alpha = max(alpha, tt_entry.eval),
            node_type::CUT => beta = min(beta, tt_entry.eval)
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
    let mut value = i32::MIN>>1;
    for moveto_play in legal_moves {
        let (a,b, prom) = convert_custum_move(moveto_play);

        let mut game1 = *game;

        if game.white_to_play { compute_move_w_hash((a, b, prom), &mut game1); }
        else { compute_move_b_hash((a, b, prom), &mut game1); }
        game1.white_to_play^=true;
        value = max(value, -alpha_beta_neg_tt(&mut game1, depth-1, -beta, -alpha, tt, nb_node));
        alpha = max(alpha, value);
        if alpha >= beta {
            break;
        }
    }

    //tt_entry.eval = value;
    let node_t;
    if value <= alpha_orgi {
        node_t = node_type::CUT;
    }
    else if value >= beta {
        node_t = node_type::ALL;
    }
    else {
        node_t = node_type::PV;
    }
    tt.set(game.hash, depth, value, 0, node_t);

    value
}
pub fn alpha_beta_neg_tt_best(game: &Game, depth : i8, mut alpha : i32, mut beta : i32, tt : &mut TranspositionTable, nb_node : &mut u64, first : u64) -> (i32, u64) {
    
    *nb_node+=1;
    let alpha_orgi = alpha;
    let mut hash_move = 0u64;
    /* TT look up */
    let mut best_move = 0u64;
    let tt_entry = tt.get(game.hash);
    if tt_entry.hash == game.hash {
        
        if tt_entry.depth >= depth {
            match tt_entry.node_type {
                node_type::PV =>  return (tt_entry.eval, tt_entry.bestmove),
                node_type::ALL => alpha = max(alpha, tt_entry.eval),
                node_type::CUT => { 
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
        if !game.white_to_play {
            eval *= -1;
        };
        return (eval,0);
    };
    
    //println!("{} {}", hash_moves.len() , legal_moves.len());
    let mut value = i32::MIN>>1;
    let mut hash_played = false;
    let mut first_played = false;
    if hash_move != 0 {
        hash_played = true;
        legal_moves.push_front((hash_move, Piece::NONE));
    }
    if first != 0 {
        first_played = true;
        legal_moves.push_front((first,Piece::NONE));
        //eprintln!("Hello {}", convert_custum_to_str(first));
    }
    let mut i = 0;
    for moveto_play in legal_moves{
        if (hash_move == moveto_play.0 || first == moveto_play.0) && i > 2 {
            continue;
        }
        
        //eprintln!("MOVE : {}", convert_custum_to_str(moveto_play.0));
        let (a,b, prom) = convert_custum_move(moveto_play);

        let mut game1 = *game;

        if game.white_to_play { compute_move_w_hash((a, b, prom), &mut game1); }
        else { compute_move_b_hash((a, b, prom), &mut game1); }
        game1.white_to_play^=true;
        let (score, _b_move) = alpha_beta_neg_tt_best(&mut game1, depth-1, -beta, -alpha, tt, nb_node, 0);
        if value < -score {
            value = -score;
            best_move = moveto_play.0;
        }
        alpha = max(alpha, value);
        //value = max(value, );
        if alpha >= beta {
            break;
        }
        i+=1;
    }
    let node_t;
    if value <= alpha_orgi {
        node_t = node_type::CUT;
    }
    else if value >= beta {
        node_t = node_type::ALL;
    }
    else {
        node_t = node_type::PV;
    }
    tt.set(game.hash, depth, value, best_move, node_t);
    //eprintln!("{}", convert_custum_to_str(best_move));
    (value, best_move)
}
pub fn mtd_f(game : &Game, f : i32, depth : i8, tt : &mut TranspositionTable, nb_node : &mut u64, first : u64) -> (i32, u64) {
    //eprintln!("MTD-F inside {} {}", depth, f);
    let (mut g, mut bmove) = (f,first);
    
    //let bmove = 0;
    let mut upperbound = i32::MAX;
    let mut lowerbound = i32::MIN;
    loop  {
        //eprintln!("WINDOW");
        let beta = g + (g == lowerbound) as i32;
        (g, bmove) = alpha_beta_neg_tt_best(game, depth, beta-1, beta, tt , nb_node, first);
        
        //eprintln!("{}", convert_custum_to_str(bmove));
        if g < beta { upperbound = g } else { lowerbound = g};
        if lowerbound >= upperbound {
            break;
        }
    }
    (g, bmove)
}
/*
fn nextGuess(α, β, subtreeCount) {
    return α + (β − α) × (subtreeCount − 1) / subtreeCount
}
pub fn bns(node, α, β) {
    subtreeCount := number of children of node

    do
        test := nextGuess(α, β, subtreeCount)
        betterCount := 0
        for each child of node do
            bestVal := −alphabeta(child, −test, −(test − 1))
            if bestVal ≥ test then
                betterCount := betterCount + 1
                bestNode := child
        (update number of sub-trees that exceeds separation test value)
        (update alpha-beta range)
    while not (β − α < 2 or betterCount = 1)

    return bestNode
}
*/