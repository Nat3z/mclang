use std::{fs, process::exit};

use crate::{ast::constructor::AST, compile::compiler::Compiler, errors::error::{std_error, StdErrors}, lexer::lexer::Lexer};

pub fn run() {
    let code = fs::read_to_string("code.mc");
    if code.is_err() {
        std_error(StdErrors::IOError("Failed to access file. Does it exist or lacking permissions?"));
        exit(1);
    }
    let code: String = code.unwrap();
    let mut lexer = Lexer::new(code);
    lexer.tokenizer();

    let tokens = lexer.flush();
    let mut ast = AST::new(tokens.to_vec());
    ast.generate();
    println!("{:?}", ast.flush());
    let mut compiler = Compiler::new(ast.flush().to_vec());
}
