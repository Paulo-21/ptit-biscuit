use std::{io, sync::RwLock};
use std::sync::Arc;
use crate::chess::*;
use crate::eval::eval;
use crate::search::*;
use crate::table_transposition::TranspositionTable;
use std::time::{Instant, Duration};
use crate::perft::*;
use std::thread;

pub fn uci () {
    let mut game = Game::default();
    let mut tt = TranspositionTable::with_memory(8<<22);
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let command : Vec<&str> = buffer.trim().split_ascii_whitespace().collect();
        match command[0] {
            "uci" => {
                input_uci();
            },
            "isready" => {
                input_ready();
            },
            "position" => {
                game = input_position(command);
            },
            "go" => {
                tt = TranspositionTable::with_memory(8<<22);
                let (a, b, prom) = compute(&game, &mut tt);
                let bestmovea = convert_square_to_move(a);
                let bestmoveb = convert_square_to_move(b);
                match prom {
                    Piece::NONE => {
                        println!("bestmove {}{}", bestmovea, bestmoveb);
                    },
                    _ => {
                        println!("bestmove {}{}q", bestmovea, bestmoveb);
                    }
                }
            },
            "perft" => {
                let game = Game::default();
                let mut i = 1;
                loop {
                    let now = Instant::now();
                    println!("Perft <{i}> : {} {} milliseconde", perft(game, i), now.elapsed().as_millis());
                    i+=1;
                }
            },
            "stop" => {
                //break;
            },
            "quit" => {
                break;
            }
            _ => {

            }
        }
    }
}
fn compute(game : &Game, tt : &mut TranspositionTable) -> (u64, u64, Piece) {
    eprintln!("START Compute");
    let now = Instant::now();
    let depth = 7;
    eprintln!("Pronfondeur : {depth}");
    //draw_the_game_state(game);
    //compute_negamax(game)
    //let res = compute_alpha_beta_neg(game, depth);
    //let res = compute_alpha_beta_neg_tt(game, depth, tt);
    //let res = compute_minimax(game);
    //let res = compute_alpha_beta(game, depth );
    //let res = compute_pvs(game, depth , tt);
    let res = compute_mdt_f_iter(game, depth, tt);

    eprintln!("Compute in : {} milli seconde", now.elapsed().as_millis());
    res
}
fn input_uci() {
    println!("id name Ptit Biscuit\n");
    println!("id author Paul Cibier\n");
    println!("uciok");
}
fn input_ready() {
    println!("readyok");
}
fn input_position(commande : Vec<&str>) -> Game {
    
    if commande[1] == "startpos" {
        return get_bitboard_from_startpos(commande);
    }
    else {// if commande.contains("fen") {
        
        get_bitboard_from_startpos(commande)
        //get_bitboard_from_fen(commande)
    }
}
fn get_bitboard_from_startpos(command : Vec<&str>) -> Game {
    let move_tab = command.iter().skip(3);
    let mut game = get_game_from_basicpos();
    if move_tab.len() <= 0 {
        return game;
    }
    for one_move in move_tab {
        //println!("{}", one_move);
        let (a,b, prom) = convert_move_to_bitboard(one_move);
        let reponse = if game.white_to_play {
            compute_move_w((a, b, prom), &mut game)
        }
        else {
            compute_move_b((a, b, prom), &mut game)
        };
        if reponse >= 0 {
            game.white_to_play ^= true;
        }
        else {
            eprintln!("ERROR Startpos");
            eprintln!("ERROR Startpos");
            eprintln!("ERROR Startpos");
        }
    }
    game
}

fn compute_minimax(game : &Game, depth : i8) -> (u64 , u64) {
    eprintln!("MINIMAX");
    let mut nb_node = 0u64;
    let maximizing_player = game.white_to_play;
    eprintln!("{maximizing_player}");
    let legal_moves = get_legal_move(game.white_to_play, game);
    eprintln!("info : {:?}", legal_moves);
    let mut score = if maximizing_player { i32::MIN } else { i32::MAX };
    let mut bestmove = 0u64;
    if !legal_moves.is_empty() {
        bestmove = legal_moves.get(0).unwrap().0;
    }
    for moveto in legal_moves {
        let mut game1 = *game;
        let (a , b, prom) = convert_custum_move(moveto);
        if game.white_to_play { compute_move_w((a, b, prom), &mut game1); }
        else { compute_move_b((a, b, prom), &mut game1); }
        game1.white_to_play ^= true;
        let move_score = minimax(&mut game1, depth-1, maximizing_player^true, &mut nb_node);
        eprintln!("{}{} : {}, ", convert_square_to_move(a), convert_square_to_move(b), move_score);
        if maximizing_player {
            if move_score > score {
                score = move_score;
                bestmove = moveto.0;
            }
        }
        else if move_score < score {
            score = move_score;
            bestmove = moveto.0;
        }
    }
    eprintln!();
    let a = bestmove >> 8;
    let b = bestmove & 255;
    eprintln!("NB nodes : {nb_node}");
    (a,b)
}

fn compute_alpha_beta(game : &Game, depth : i8) -> (u64 , u64, Piece) {
    eprintln!("ALPHA BETA");
    let alpha = i32::MIN;
    let beta = i32::MAX;
    let mut nb_node = 0u64;
    let legal_moves = get_legal_move(game.white_to_play, game);
    eprintln!("info : {:?}", legal_moves);
    let mut score = if game.white_to_play { i32::MIN } else { i32::MAX };
    let mut bestmove = (0u64, Piece::NONE);
    if !legal_moves.is_empty() {
        bestmove = *legal_moves.get(0).unwrap();
    }
    for moveto in legal_moves {
        let mut game1 = *game;
        let (a, b, prom) = convert_custum_move(moveto);
        if game.white_to_play { compute_move_w((a, b, prom), &mut game1); }
        else { compute_move_b((a, b, prom), &mut game1); }
        game1.white_to_play ^= true;
        let move_score = alpha_beta(&mut game1, depth-1, alpha, beta, &mut nb_node);
        eprintln!("{}{} : {}, ", convert_square_to_move(a), convert_square_to_move(b), move_score);
        if game.white_to_play {
            if move_score > score {
                score = move_score;
                bestmove = moveto;
            }
        }
        else if move_score < score {
            score = move_score;
            bestmove = moveto;
        }
    }
    eprintln!();
    let (a, b, prom) = convert_custum_move(bestmove);
    eprintln!("NB nodes : {nb_node}");
    (a,b, prom)
}

fn compute_alpha_beta_neg(game : &Game, depth : i8) -> (u64, u64, Piece) {
    eprintln!("NEGAMAX");
    let mut nb_node = 0u64;
    let legal_moves = get_legal_move(game.white_to_play, game);
    eprintln!("info : {:?}", legal_moves);
    let mut score = i32::MIN>>1;
    let mut bestmove = (0u64, Piece::NONE);
    
    for moveto in legal_moves {
        let mut game1 = *game;
        let (a, b, prom) = convert_custum_move(moveto);
        if game.white_to_play { compute_move_w((a, b, prom), &mut game1); }
        else { compute_move_b((a, b, prom), &mut game1); }
        game1.white_to_play ^= true;

        let move_score = (-1)*alpha_beta_neg(&mut game1, depth-1, i32::MIN>>1, i32::MAX>>1, &mut nb_node);
        eprintln!("{}{} : {}, ", convert_square_to_move(a), convert_square_to_move(b), move_score);
        
        if move_score > score {
            score = move_score;
            bestmove = moveto;
        }
    }
    eprintln!();
    let (a, b, prom ) = convert_custum_move(bestmove);
    println!("NB nodes : {nb_node}");
    (a,b, prom)
}
fn compute_alpha_beta_neg_tt(game : &Game, depth : i8, tt : &mut TranspositionTable) -> (u64, u64, Piece) {
    eprintln!("NEGAMAX TT");
    let mut nb_node = 0u64;
    let legal_moves = get_legal_move(game.white_to_play, game);
    eprintln!("info : {:?}", legal_moves);
    let mut score = i32::MIN>>1;
    let mut bestmove = (0u64, Piece::NONE);
    
    for moveto in legal_moves {
        let mut game1 = *game;
        let (a, b, prom) = convert_custum_move(moveto);
        if game.white_to_play { compute_move_w_hash((a, b, prom), &mut game1); }
        else { compute_move_b_hash((a, b, prom), &mut game1); }
        game1.white_to_play ^= true;

        let move_score = (-1)*alpha_beta_neg_tt(&mut game1, depth-1, i32::MIN>>1, i32::MAX>>1, tt, &mut nb_node);
        eprintln!("{}{} : {}, ", convert_square_to_move(a), convert_square_to_move(b), move_score);
        
        if move_score > score {
            score = move_score;
            bestmove = moveto;
        }
    }
    eprintln!();
    let (a, b, prom ) = convert_custum_move(bestmove);
    eprintln!("NB nodes : {nb_node}");
    (a,b, prom)
}


fn compute_mdt_f_iter(game : &Game, depth : i8, tt1 : &mut TranspositionTable) -> (u64, u64, Piece) {
    eprintln!("MTD-F");
    let mut res = (0,0,Piece::NONE);
    
    let game1 = *game;
    let mut stop = false;
    let lock = Arc::new(RwLock::new((0,0, Piece::NONE)));
    let lock2 = lock.clone();
    //let t1 = thread::spawn(move || {
        let mut tt = TranspositionTable::with_memory(8<<25);
        let mut nb_node : u64 = 0;
        //let legal_move = get_legal_move(game.white_to_play, game);
        let (mut firstguess, mut bmove) = (0,0);
        //let (mut firstguess, mut bmove) = (eval(game, legal_move.len() as i32),0);
        for d in 1..depth+1 {
            (firstguess, bmove) = mtd_f(&game1, firstguess, d, &mut tt, &mut nb_node, bmove);
            let (a, b, p) = convert_custum_move((bmove, Piece::QUEEN));
            let out = convert_move_to_str(a, b, p);
            eprintln!(" depth : {}, current : {}, eval : {}, nbNode : {}", d , out, firstguess, nb_node);
            eprintln!(" tt hits : {}",tt.stat_hint);
            
            /*if *stopref
            { break; }*/
        }
        res = convert_custum_move((bmove, Piece::QUEEN));
    //    *lock2.write().unwrap() = convert_custum_move((bmove, Piece::QUEEN));
    //});

    //thread::sleep(Duration::from_secs(5));
    
    //let rt = lock.read().unwrap();
    //t1.join();
    //return *rt;
    return res;
}
fn compute_pvs(game : &Game, depth : i8, tt : &mut TranspositionTable) -> (u64 , u64, Piece) {
    eprintln!("PRINCIPAL VARIATION SEARCH");
    let alpha = i32::MIN>>1;
    let beta = i32::MAX>>1;
    let mut nb_node = 0u64;
    let legal_moves = get_legal_move(game.white_to_play, game);
    eprintln!("info : {:?}", legal_moves);
    let mut score = if game.white_to_play { i32::MIN } else { i32::MAX };
    let mut bestmove = (0u64, Piece::NONE);
    
    for moveto in legal_moves {
        let mut game1 = *game;
        let (a, b, prom) = convert_custum_move(moveto);
        if game.white_to_play { compute_move_w((a, b, prom), &mut game1); }
        else { compute_move_b((a, b, prom), &mut game1); }
        game1.white_to_play ^= true;
        let move_score = -pvs_tt_best(&mut game1, depth-1, alpha, beta, &mut nb_node, tt, 0);
        eprintln!("{}{} : {}, ", convert_square_to_move(a), convert_square_to_move(b), move_score);
        if move_score > score {
            score = move_score;
            bestmove = moveto;
        }
    }
    eprintln!();
    let (a, b, prom) = convert_custum_move(bestmove);
    eprintln!("NB nodes : {nb_node}");
    (a,b, prom)
}
/*
fn compute_pvs_iter(game : &Game, depth : i8, tt : &mut TranspositionTable) -> (u64, u64, Piece) {
    eprintln!("PFS Iter");
    let mut nb_node : u64 = 0;
    let (mut firstguess, mut bmove) = (0,0);
    //let (mut firstguess, mut bmove) = (eval(game, legal_move.len() as i32),0);
    for d in 1..depth+1 {
        (firstguess,bmove) = pvs_tt_best(game, d, tt, &mut nb_node, bmove);
        let (a, b, p) = convert_custum_move((bmove, Piece::QUEEN));
        let out = convert_move_to_str(a, b, p);
        eprintln!(" depth : {}, current : {}, eval : {}, nbNode : {}", d , out, firstguess, nb_node);
        eprintln!(" tt hits : {}",tt.stat_hint);
        //if times_up()
        //{ break; }
    }
    let res = convert_custum_move((bmove, Piece::QUEEN));
    return res;
}*/