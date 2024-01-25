use std::process;

mod token;
mod lexer;
mod ast;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let mut parser = parser::new(filename).unwrap_or_else(|e| {
        println!("scc: {}", e);
        process::exit(2);
    });
    let ast = parser.parse().unwrap_or_else(|e| {
        println!("scc: {}", e);
        process::exit(2);
    });
    ast.print();
}
