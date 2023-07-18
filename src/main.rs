mod assembler;
mod ast;
mod lexer;
mod parser;

use std::env;
use std::fs;

use crate::assembler::Assembler;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let filepath = &args[2];
    let contents = fs::read_to_string(filepath).expect(
        "Failed to read file.",
    );
    let tokens = Lexer::tokenize(contents);
    let ast = Parser::parse(tokens);
    // FIXME: IMPLEMENT ANALYZER
    match mode.as_str() {
        "wat" => {
            let result = Assembler::to_wat(ast);
            println!("{}", result);
        },
        _ => {
            panic!("Invalid mode.");
        },
    }
}
