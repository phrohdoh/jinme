use std::io::{self, BufRead, Write};

fn main() {
    let mut stdout = io::stdout();
    let mut line = String::new();

    print!("> ");
    stdout.flush().unwrap();

    io::stdin().lock().read_line(&mut line).unwrap();
    println!("got {:?} (len = {})", line, line.len());
}
