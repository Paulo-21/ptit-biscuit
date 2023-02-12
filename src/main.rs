use std::io;

fn main() {
    println!("P'TIT BISCUIT");
    loop {
        print!(">");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        println!("{buffer}");
    }
}
