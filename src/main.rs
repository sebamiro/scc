use std::process;

mod lexer;
mod ast;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let tokens = lexer::lex(filename).unwrap_or_else(|e| {
        println!("scc: {}", e);
        process::exit(2);
    });
    for token in tokens {
        println!("{:?}", token);
    }
}
