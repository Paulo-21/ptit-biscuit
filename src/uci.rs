use std::{io, cmp::Ordering};

pub fn uci () {
    println!("uciok");
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut response = String::new();
        let mut command = buffer.trim();

        if command == "uci"  {
            input_uci();
        }
        else if command == "isready" {
            input_ready();
        }
        else if command.len() >= 8 && command[..8].cmp("position") == Ordering::Equal {
            
            input_position(&command[ 9..]);
        }
        else if command.len() >= 2 && command[..2].cmp("go") == Ordering::Equal {
            compute();
            println!("bestmove e7e5");
        }
        else if command == "stop" {
            response.push_str("bestmove e2e4");
        }
        

    }
}
fn input_uci() {
    println!("id name Ptit Biscuit\n");
    println!("id author Paul Cibier\n");
    println!("uciok");
}
fn input_ready() {
    println!("readyok");
}
fn input_position(mut commande : &str) {
    if commande.contains("startpos") {
        commande = &commande[9..];
        get_bitboard_from_startpos(commande);
    }
    else if commande.contains("fen") {
        commande = &commande[3..];
        get_bitboard_from_fen(commande);
    }
}
fn compute() {
    
}
fn get_bitboard_from_startpos(command : &str)  { // -> Game {
    let move_tab = command.split_ascii_whitespace();
    let init_board = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let white_to_play = true;
    for one_move in move_tab {
        /*let (a,b) = convert_move();
        if white_to_play {
            compute_w();
        }
        else {
            compute_b();
        }
        return final_bitboard;
        */

        println!("{one_move}");
    }
}   
fn get_bitboard_from_fen(command : &str) {

}