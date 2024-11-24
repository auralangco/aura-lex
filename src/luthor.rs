use std::env::args;

use aura_lex::lexer::lex;

fn main() {
    if args().len() != 2 {
        eprintln!("usage: luthor <filename>");
        std::process::exit(1);
    }
    let filename = args().nth(1).expect("no filename provided");
    let src = std::fs::read_to_string(filename).expect("failed to read file");
    let lexemes = lex(&src);
    println!("{:#?}", lexemes);
}