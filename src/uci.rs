use std::{io, cmp::Ordering};

pub fn uci () {
    println!("uciok");
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut response = String::new();
        let command = buffer.trim();
       
        if command == "uci"  {
            input_uci();
        }
        else if command == "isready" {
            println!("readyok");
        }
        else if command == "position" {

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
fn compute() {
    
}