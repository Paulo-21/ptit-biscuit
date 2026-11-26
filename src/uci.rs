use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use std::{io, sync::RwLock};

use crate::chess::*;
use crate::perft::*;
use crate::search::*;
use crate::search_tools::SearchTools;
use crate::table_transposition::TranspositionTable;
use crate::zobrist::init_zobrist_key;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn uci() {
    let mut game = Game::default();
    let mut tt = TranspositionTable::with_memory(8 << 22);
    /*let piece = 1<<40;
    _draw_bitboard(piece);

    _draw_bitboard(1<<flip((piece).tzcnt()));
    println!("{}", MG_BISHOP_TABLE[(piece).tzcnt() as usize]);
    println!("{}", MG_BISHOP_TABLE[flip(piece.tzcnt()) as usize]);
    */
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let command: Vec<&str> = buffer.trim().split_ascii_whitespace().collect();
        match command[0] {
            "uci" => {
                input_uci();
            }
            "isready" => {
                input_ready();
            }
            "position" => {
                game = input_position(command);
                //_draw_board(&game);
                /*let (cap, quiet, n, v) = get_legal_moves_fast_c(&mut game);
                println!("PLAY W {}", game.white_to_play);
                for c in cap {
                    println!("sc {}, piece : {}", c.0, c.1);
                    _draw_bitboard(c.1);
                }
                let vev = eval(&game, 30);
                println!("{}", vev);*/
            }
            "go" => {
                input_go(command, &game, &mut tt);
            }
            "perft" => {
                //let game = Game::default();
                //let fen = "position fen r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0".trim().split_ascii_whitespace().collect();
                //let game = get_bitboard_from_fen(fen);

                let mut i = 1u8;
                loop {
                    let now = Instant::now();
                    let mut legal_moves_tab = vec![[0u64; 70]; i as usize + 1];
                    let nb_node = perft(game, i, &mut legal_moves_tab) as u128;
                    let time = now.elapsed().as_millis();
                    let nps = nb_node.checked_div(time);
                    let nps_str = match nps {
                        Some(x) => (x * 1000).to_string(),
                        None => "INF".to_string(),
                    };
                    println!(
                        "Perft <{i}> : {} {} milliseconde, {} Nps",
                        nb_node, time, nps_str
                    );
                    i += 1;
                }
            }
            "stop" => {
                //break;
            }
            "quit" => {
                break;
            }
            _ => {}
        }
    }
}

fn input_go(command: Vec<&str>, game: &Game, tt: &mut TranspositionTable) {
    let mut depth = 100 as u8;
    let mut wtime = 0;
    let mut btime = 0;
    let mut winc = 0;
    let mut binc = 0;
    let mut move_time = 3000;
    if let Some(pos) = command.iter().position(|&x| x == "depth") {
        depth = command[pos + 1].parse::<u8>().unwrap();
        move_time = 0;
    }
    if let Some(pos) = command.iter().position(|&x| x == "wtime") {
        wtime = command[pos + 1].parse::<i32>().unwrap();
    }
    if let Some(pos) = command.iter().position(|&x| x == "btime") {
        btime = command[pos + 1].parse::<i32>().unwrap();
    }
    if let Some(pos) = command.iter().position(|&x| x == "winc") {
        winc = command[pos + 1].parse::<i32>().unwrap();
    }
    if let Some(pos) = command.iter().position(|&x| x == "binc") {
        binc = command[pos + 1].parse::<i32>().unwrap();
    }
    match game.white_to_play {
        true => {
            if wtime > 0 && winc > 0 {
                if wtime < winc {
                    move_time = (wtime as f32 * 0.7) as i32;
                } else if wtime < (winc as f32 * 1.5) as i32 {
                    move_time = (wtime as f32 * 0.8) as i32;
                } else {
                    move_time = wtime / 25 + winc;
                }
            } else if wtime > 0 {
                move_time = wtime / 25;
            }
            if game.nb_coups <= 6 && wtime > 0 {
                move_time = 4000;
            }
        }
        false => {
            if btime > 0 && binc > 0 {
                if btime < binc {
                    move_time = (btime as f32 * 0.7) as i32;
                } else if btime < (binc as f32 * 1.5) as i32 {
                    move_time = (btime as f32 * 0.8) as i32;
                } else {
                    move_time = btime / 25 + binc;
                }
            } else if btime > 0 {
                move_time = btime / 25;
            }
            if game.nb_coups <= 6 && btime > 0 {
                move_time = 4000;
            }
        }
    }
    if let Some(pos) = command.iter().position(|&x| x == "movetime") {
        move_time = command[pos + 1].parse::<i32>().unwrap();
    }
    eprintln!("{}, {}", depth, move_time);
    let (a, b, prom) = compute(game, depth, move_time, tt);
    let bestmovea = convert_square_to_move(a);
    let bestmoveb = convert_square_to_move(b);
    match prom {
        Piece::NONE => {
            println!("bestmove {}{}", bestmovea, bestmoveb);
        }
        _ => {
            println!("bestmove {}{}q", bestmovea, bestmoveb);
        }
    }
}

fn compute(
    game: &Game,
    depth: u8,
    move_time: i32,
    tt: &mut TranspositionTable,
) -> (u64, u64, Piece) {
    eprintln!("START Compute");
    let now = Instant::now();
    eprintln!("Pronfondeur : {depth}");
    //draw_the_game_state(game);
    //compute_negamax(game);
    //let res = _compute_alpha_beta_neg(game, depth);
    //let res = _compute_alpha_beta_neg_tt(game, depth, tt);
    //let res = _compute_minimax(game);
    //let res = _compute_alpha_beta(game, depth );
    //let res = compute_pvs(game, depth , tt);
    //let res = _compute_pvs_iter(game, depth , tt);
    let res = compute_mdt_f_iter(game, depth, move_time, tt);

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
fn input_position(commande: Vec<&str>) -> Game {
    if commande[1] == "startpos" {
        return get_bitboard_from_startpos(commande);
    } else if commande[1] == "fen" {
        // "position fen <fen> moves e2e4 ..." : les coups étaient ignorés
        let moves_pos = commande.iter().position(|&x| x == "moves");
        let fen_part = commande[..moves_pos.unwrap_or(commande.len())].to_vec();
        let mut game = get_bitboard_from_fen(fen_part);
        if let Some(p) = moves_pos {
            for one_move in &commande[p + 1..] {
                let mv = convert_move_to_bitboard(one_move);
                let w = game.white_to_play;
                make_move(mv, &mut game, w);
                game.white_to_play ^= true;
            }
        }
        game
    } else {
        Game::default()
    }
}

pub fn get_bitboard_from_fen(fen: Vec<&str>) -> Game {
    let mut game = Game::empty();
    let board = fen[2];
    let toplay = fen[3];
    let castling_right = fen.get(4).copied().unwrap_or("-");
    let en_passant = fen.get(5).copied().unwrap_or("-");
    // let nb_coup = fen[6];

    let mut i: i32 = 56; // a8 = bit 56, coin haut-gauche du FEN

    for ligne in board.split('/') {
        let mut col: i32 = 0;
        for c in ligne.chars() {
            let sq = i + col;
            match c {
                'p' => game.bp |= 1u64 << sq,
                'n' => game.bn |= 1u64 << sq,
                'b' => game.bb |= 1u64 << sq,
                'r' => game.br |= 1u64 << sq,
                'q' => game.bq |= 1u64 << sq,
                'k' => game.bk |= 1u64 << sq,
                'P' => game.wp |= 1u64 << sq,
                'N' => game.wn |= 1u64 << sq,
                'B' => game.wb |= 1u64 << sq,
                'R' => game.wr |= 1u64 << sq,
                'Q' => game.wq |= 1u64 << sq,
                'K' => game.wk |= 1u64 << sq,
                n if n.is_ascii_digit() => {
                    col += n.to_digit(10).unwrap() as i32 - 1;
                }
                _ => {}
            }
            col += 1;
        }
        i -= 8; // rangée suivante vers le bas
    }

    game.white_to_play = match toplay.chars().next().unwrap() {
        'w' => true,
        'b' => false,
        _ => true,
    };

    if en_passant.len() == 2 && en_passant != "-" {
        game.en_passant = 1u64 << convert_move_str_to_bitboard(en_passant);
    }
    if let Some(n) = fen.get(7).and_then(|x| x.parse::<u16>().ok()) {
        game.nb_coups = 2 * n.saturating_sub(1) + (!game.white_to_play) as u16;
    }

    for right in castling_right.chars() {
        match right {
            'K' => game.wking_castle = true,
            'Q' => game.wqueen_castle = true,
            'k' => game.bking_castle = true,
            'q' => game.bqueen_castle = true,
            _ => {}
        }
    }

    game.hash = init_zobrist_key(&game);
    game
}
fn get_bitboard_from_startpos(command: Vec<&str>) -> Game {
    let move_tab = command.iter().skip(3);
    let mut game = get_game_from_basicpos();
    if move_tab.len() == 0 {
        return game;
    }
    for one_move in move_tab {
        //println!("{}", one_move);
        let (a, b, prom) = convert_move_to_bitboard(one_move);
        let reponse = if game.white_to_play {
            compute_move_w_hash((a, b, prom), &mut game)
        } else {
            compute_move_b_hash((a, b, prom), &mut game)
        };
        if reponse >= 0 {
            game.white_to_play ^= true;
        } else {
            eprintln!("ERROR Startpos");
            eprintln!("ERROR Startpos");
            eprintln!("ERROR Startpos");
        }
    }
    game
}

fn _compute_minimax(game: &Game, depth: u8) -> (u64, u64) {
    eprintln!("MINIMAX");
    let mut nb_node = 0u64;
    let maximizing_player = game.white_to_play;
    eprintln!("{maximizing_player}");
    let legal_moves = get_legal_move(game.white_to_play, game);
    eprintln!("info : {:?}", legal_moves);
    let mut score = if maximizing_player {
        i32::MIN
    } else {
        i32::MAX
    };
    let mut bestmove = 0u64;
    if !legal_moves.is_empty() {
        bestmove = legal_moves.get(0).unwrap().0;
    }
    for moveto in legal_moves {
        let mut game1 = *game;
        let (a, b, prom) = convert_custum_move(moveto);
        if game.white_to_play {
            compute_move_w((a, b, prom), &mut game1);
        } else {
            compute_move_b((a, b, prom), &mut game1);
        }
        game1.white_to_play ^= true;
        let move_score = _minimax(
            &mut game1,
            depth - 1,
            maximizing_player ^ true,
            &mut nb_node,
        );
        eprintln!(
            "{}{} : {}, ",
            convert_square_to_move(a),
            convert_square_to_move(b),
            move_score
        );
        if maximizing_player {
            if move_score > score {
                score = move_score;
                bestmove = moveto.0;
            }
        } else if move_score < score {
            score = move_score;
            bestmove = moveto.0;
        }
    }
    eprintln!();
    let a = bestmove >> 8;
    let b = bestmove & 255;
    eprintln!("NB nodes : {nb_node}");
    (a, b)
}

fn _compute_alpha_beta(game: &Game, depth: u8) -> (u64, u64, Piece) {
    eprintln!("ALPHA BETA");
    let alpha = i32::MIN;
    let beta = i32::MAX;
    let mut nb_node = 0u64;
    let legal_moves = get_legal_move(game.white_to_play, game);
    eprintln!("info : {:?}", legal_moves);
    let mut score = if game.white_to_play {
        i32::MIN
    } else {
        i32::MAX
    };
    let mut bestmove = (0u64, Piece::NONE);
    if !legal_moves.is_empty() {
        bestmove = *legal_moves.get(0).unwrap();
    }
    for moveto in legal_moves {
        let mut game1 = *game;
        let (a, b, prom) = convert_custum_move(moveto);
        if game.white_to_play {
            compute_move_w((a, b, prom), &mut game1);
        } else {
            compute_move_b((a, b, prom), &mut game1);
        }
        game1.white_to_play ^= true;
        let move_score = _alpha_beta(&mut game1, depth - 1, alpha, beta, &mut nb_node);
        eprintln!(
            "{}{} : {}, ",
            convert_square_to_move(a),
            convert_square_to_move(b),
            move_score
        );
        if game.white_to_play {
            if move_score > score {
                score = move_score;
                bestmove = moveto;
            }
        } else if move_score < score {
            score = move_score;
            bestmove = moveto;
        }
    }
    eprintln!();
    let (a, b, prom) = convert_custum_move(bestmove);
    eprintln!("NB nodes : {nb_node}");
    (a, b, prom)
}

fn _compute_alpha_beta_neg(game: &Game, depth: u8) -> (u64, u64, Piece) {
    eprintln!("NEGAMAX");
    let mut nb_node = 0u64;
    let legal_moves = get_legal_move(game.white_to_play, game);
    eprintln!("info : {:?}", legal_moves);
    let mut score = i32::MIN >> 1;
    let mut bestmove = (0u64, Piece::NONE);

    for moveto in legal_moves {
        let mut game1 = *game;
        let (a, b, prom) = convert_custum_move(moveto);
        if game.white_to_play {
            compute_move_w((a, b, prom), &mut game1);
        } else {
            compute_move_b((a, b, prom), &mut game1);
        }
        game1.white_to_play ^= true;

        let move_score = (-1)
            * _alpha_beta_neg(
                &mut game1,
                depth - 1,
                i32::MIN >> 1,
                i32::MAX >> 1,
                &mut nb_node,
            );
        eprintln!(
            "{}{} : {}, ",
            convert_square_to_move(a),
            convert_square_to_move(b),
            move_score
        );

        if move_score > score {
            score = move_score;
            bestmove = moveto;
        }
    }
    eprintln!();
    let (a, b, prom) = convert_custum_move(bestmove);
    eprintln!("NB nodes : {nb_node}");
    (a, b, prom)
}
fn _compute_alpha_beta_neg_tt(
    game: &Game,
    depth: u8,
    tt: &mut TranspositionTable,
) -> (u64, u64, Piece) {
    eprintln!("NEGAMAX TT");
    let mut nb_node = 0u64;
    let legal_moves = get_legal_move(game.white_to_play, game);
    eprintln!("info : {:?}", legal_moves);
    let mut score = i32::MIN >> 1;
    let mut bestmove = (0u64, Piece::NONE);

    for moveto in legal_moves {
        let mut game1 = *game;
        let (a, b, prom) = convert_custum_move(moveto);
        if game.white_to_play {
            compute_move_w_hash((a, b, prom), &mut game1);
        } else {
            compute_move_b_hash((a, b, prom), &mut game1);
        }
        game1.white_to_play ^= true;

        let move_score = (-1)
            * _alpha_beta_neg_tt(
                &mut game1,
                depth - 1,
                i32::MIN >> 1,
                i32::MAX >> 1,
                tt,
                &mut nb_node,
            );
        eprintln!(
            "{}{} : {}, ",
            convert_square_to_move(a),
            convert_square_to_move(b),
            move_score
        );

        if move_score > score {
            score = move_score;
            bestmove = moveto;
        }
    }
    eprintln!();
    let (a, b, prom) = convert_custum_move(bestmove);
    eprintln!("NB nodes : {nb_node}");
    (a, b, prom)
}

fn compute_mdt_f_iter(
    game: &Game,
    depth: u8,
    move_time: i32,
    _tt1: &mut TranspositionTable,
) -> (u64, u64, Piece) {
    eprintln!("MTD-F");
    let mut res = (0, 0, Piece::NONE);
    let lock = Arc::new(RwLock::new(res));
    let lock2 = lock.clone();
    let mut game1 = *game;
    let mut _stop = false;
    let arc = Arc::new(AtomicBool::new(false));
    let arc2 = arc.clone();
    if move_time == 0 {
        let tt = TranspositionTable::with_memory(8 << 25);
        let mut tool = SearchTools::new(tt, arc2);
        let mut nb_node: u64 = 0;
        /*let legal_move = get_legal_move(game.white_to_play, &game1);
        for l in legal_move {
            eprint!("{}, ", convert_custum_to_str(l.0));
        }
        eprintln!();*/

        let (mut firstguess, mut bmove) = (0, 0);
        for d in 1..depth + 1 {
            nb_node = 0;
            let (firstguess2, bmove2) =
                mtd_f(&mut game1, firstguess, d, &mut tool, &mut nb_node, bmove);
            match firstguess2 {
                Some(e) if bmove2 != 0 => {
                    bmove = bmove2;
                    firstguess = e;
                    println!(
                        "info depth {} score {} nodes {} pv {}",
                        d,
                        uci_score(e, game1.nb_coups),
                        nb_node,
                        convert_custum_to_str(bmove)
                    );
                }
                _ => {}
            }
            let (a, b, p) = convert_custum_move((bmove, Piece::QUEEN));
            let out = convert_move_to_str(a, b, p);
            eprintln!(
                " depth : {}, current : {}, eval : {}, nbNode : {}, tt hits : {}",
                d, out, firstguess, nb_node, tool.tt.stat_hint
            );
            res = convert_custum_move((bmove, Piece::NONE));
        }
        return res;
    } else {
        let t1 = thread::spawn(move || {
            let tt = TranspositionTable::with_memory(8 << 25);
            let mut tool = SearchTools::new(tt, arc2);
            let mut nb_node: u64 = 0;
            /*let legal_move = get_legal_move(game.white_to_play, &game1);
            for l in legal_move {
                eprint!("{}, ", convert_custum_to_str(l.0));
            }
            eprintln!();*/

            let (mut firstguess, mut bmove) = (0, 0);
            for d in 1..depth + 1 {
                nb_node = 0;
                let (firstguess2, bmove2) =
                    mtd_f(&mut game1, firstguess, d, &mut tool, &mut nb_node, bmove);
                match firstguess2 {
                    Some(e) if bmove2 != 0 => {
                        bmove = bmove2;
                        firstguess = e;
                        println!(
                            "info depth {} score {} nodes {} pv {}",
                            d,
                            uci_score(e, game1.nb_coups),
                            nb_node,
                            convert_custum_to_str(bmove)
                        );
                    }
                    _ => {}
                }
                let (a, b, p) = convert_custum_move((bmove, Piece::QUEEN));
                let out = convert_move_to_str(a, b, p);
                eprintln!(
                    " depth : {}, current : {}, eval : {}, nbNode : {}, tt hits : {}",
                    d, out, firstguess, nb_node, tool.tt.stat_hint
                );
                if tool.timeover.load(Ordering::Relaxed) {
                    break;
                }
            }
            //res = convert_custum_move((bmove, Piece::QUEEN));
            *lock2.write().unwrap() = convert_custum_move((bmove, Piece::NONE));
        });

        thread::sleep(Duration::from_millis(move_time as u64));
        arc.store(true, Ordering::Relaxed);
        t1.join().unwrap();
        return *lock.read().unwrap();
    }
}

/*
fn _compute_pvs(game : &Game, depth : u8, tt : &mut TranspositionTable) -> (u64 , u64, Piece) {
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
        let move_score = -_pvs_tt_best(&mut game1, depth-1, alpha, beta, &mut nb_node, tt, 0);
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

fn _compute_pvs_iter(game : &mut Game, depth : u8, tt : &mut TranspositionTable) -> (u64, u64, Piece) {
    eprintln!("PFS Iter");
    let mut nb_node : u64 = 0;
    let (mut _firstguess, bmove) = (0,0);
    let alpha = i32::MIN>>1;
    let beta = i32::MAX>>1;
    //let (mut firstguess, mut bmove) = (eval(game, legal_move.len() as i32),0);
    for d in 1..depth+1 {
        _firstguess = _pvs_tt_best(game, d, alpha, beta, &mut nb_node, tt, bmove);
        let (a, b, p) = convert_custum_move((bmove, Piece::QUEEN));
        let out = convert_move_to_str(a, b, p);
        eprintln!(" depth : {}, current : {}, eval : {}, nbNode : {}", d , out, _firstguess, nb_node);
        eprintln!(" tt hits : {}",tt.stat_hint);
        //if times_up()
        //{ break; }
    }
    let res = convert_custum_move((bmove, Piece::QUEEN));
    return res;
}*/

/// Score UCI : "cp x" ou "mate n" (les mats valent ±(999999 - 50 * ply_absolu)).
fn uci_score(e: i32, root_ply: u16) -> String {
    if e.abs() > 900_000 {
        let plies = (999_999 - e.abs()) / 50 - root_ply as i32;
        let moves = (plies + 1) / 2;
        format!("mate {}", if e > 0 { moves } else { -moves })
    } else {
        format!("cp {e}")
    }
}
