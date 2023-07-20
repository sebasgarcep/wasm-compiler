mod assembler;
mod ast;
mod cli;
mod lexer;
mod parser;

use std::fs;

use clap::Parser as CLIParser;

use crate::assembler::Assembler;
use crate::cli::CLIArguments;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let args = CLIArguments::parse();
    let contents = fs::read_to_string(args.input).expect(
        "Failed to read file.",
    );
    let tokens = Lexer::tokenize(contents);
    let ast = Parser::parse(tokens);
    // FIXME: IMPLEMENT ANALYZER
    match args.mode.as_str() {
        "wat" => {
            let result = Assembler::to_wat(ast);
            fs::write(args.output, result).expect(
                "Failed to write to file."
            );
        },
        _ => {
            panic!("Invalid mode.");
        },
    }
}
