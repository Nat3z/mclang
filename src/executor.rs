use std::{fs, process::exit};

use crate::{
    ast::constructor::AST,
    compile::compiler::Compiler,
    errors::error::{std_error, StdErrors},
    lexer::lexer::Lexer,
};

pub fn run() {
    let code = fs::read_to_string("code.mc");
    if code.is_err() {
        std_error(StdErrors::IOError(
            "Failed to access file. Does it exist or lacking permissions?",
        ));
        exit(1);
    }
    let code: String = code.unwrap();
    let mut lexer = Lexer::new(code);
    lexer.tokenizer();

    let tokens = lexer.flush();
    let mut ast = AST::new(tokens.to_vec());
    ast.generate();
    println!("{:#?}", ast.flush());
    let mut compiler = Compiler::new(ast.flush().to_vec(), "test");
    let mut scope = compiler.scopes[0].clone();
    compiler.compile(&mut scope);

    // make directory "outputs"
    let current_path = std::env::current_dir().unwrap();

    if current_path.join("outputs").exists() {
        fs::remove_dir_all(current_path.join("outputs")).unwrap_or_else(|_| {
            std_error(StdErrors::IOError(
                "Failed to remove directory. Does it exist?",
            ));
            exit(1);
        });
    }
    fs::create_dir(current_path.join("outputs")).unwrap_or_else(|_| {
        std_error(StdErrors::IOError(
            "Failed to create directory. Does it already exist?",
        ));
        exit(1);
    });
    for (name, item) in compiler.flush() {
        // clean all unnecessary new lines
        let item = item.replace("\n\n", "\n");
        // remove the first new line
        let item = item.trim_start_matches("\n");
        let name = name.replace(" ", "_") + ".mcfunction";
        fs::write(current_path.join("outputs").join(name), item).unwrap_or_else(|_| {
            std_error(StdErrors::IOError(
                "Failed to write to file. Does it already exist?",
            ));
            exit(1);
        });
    }
}
