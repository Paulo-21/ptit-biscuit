use crate::chess::*;
use crate::eval::*;
//use crate::table_transposition::Transposition;
use crate::table_transposition::TranspositionTable;
use crate::table_transposition::node_type;
use std::cmp::{ max, min };

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
        return eval(game, legal_moves.len() as i32);
    }
    let mut first = true;
    for moveto in legal_moves {
        let mut score = 0;
        let (a, b, prom) = convert_custum_move(moveto);
        let mut game1 = *game;
        game1.white_to_play ^= true;
        if game.white_to_play {
            compute_move_w((a, b, prom), &mut game1);
        } else {
            compute_move_b((a, b, prom), &mut game1);
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
        /*println!("Hello");
        println!("{}, {}", tt_entry.hash, game.hash);*/
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
    /* TT look up */
    let mut best_move = 0u64;
    let tt_entry = tt.get(game.hash);
    if tt_entry.hash == game.hash {
        
        if tt_entry.depth >= depth {
            match tt_entry.node_type {
            node_type::PV =>  return (tt_entry.eval, tt_entry.bestmove),
            node_type::ALL => alpha = max(alpha, tt_entry.eval),
            node_type::CUT => beta = min(beta, tt_entry.eval)
            }
            if alpha >= beta {
                return (tt_entry.eval, tt_entry.bestmove);
            }
        }
        /*else {
            match tt_entry.node_type {
                node_type::PV => {
                    best_move = tt_entry.bestmove;
                    println!("BEST");
                },
                _ => {

                }
            }
        }*/
        
    }
    
    
    let mut legal_moves = get_legal_move(game.white_to_play, game);
    
    if depth == 0 || legal_moves.is_empty() {
        let mut eval = eval(game, legal_moves.len() as i32);
        if !game.white_to_play {
            eval *= -1;
        };
        return (eval,0);
    };
    if first != 0 {
        legal_moves.push_front((first,Piece::NONE));
        //eprintln!("Hello {}", convert_custum_to_str(first));
    }
    let mut value = i32::MIN>>1;
    
    for moveto_play in legal_moves {
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
            /*let (a, b, p, ) = convert_custum_move((best_move, Piece::QUEEN));
            let s = convert_move_to_str(a, b, p);

            println!("good move : {}", s);*/
        }
        alpha = max(alpha, value);
        //value = max(value, );
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
    //let beta = 9999;
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
pub fn negamax(game: &mut Game, depth : i8, color : bool, nb_node : &mut u64) -> i32 {
    let legal_moves = get_legal_move(game.white_to_play, game);
    *nb_node+=1;
    if depth == 0 || legal_moves.len() == 0 {
        let mut eval = eval(game, legal_moves.len() as i32);
        if !color {
            eval *= -1;
        };
        return eval;
    };
    let mut value = i32::MIN;
    for moveto_play in legal_moves {
        let (a,b) = convert_custum_move(moveto_play);
        let mut game1 = *game;

        if game.white_to_play { compute_move_w(a, b, &mut game1); }
        else { compute_move_b(a, b, &mut game1); }
        game1.white_to_play^=true;
        value = max(value, negamax(&mut game1, depth-1, color^true, nb_node)*(-1i32));
    }
    value
}
*/