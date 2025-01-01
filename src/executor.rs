use std::{collections::HashMap, fs, process::exit};

use crate::{
    ast::constructor::AST,
    compile::compiler::{Compiler, Scope},
    errors::error::{std_error, StdErrors},
    lexer::lexer::Lexer,
};

pub fn run() {
    let files = fs::read_dir("inputs");
    let namespace = "test";
    if files.is_err() {
        std_error(StdErrors::IOError(
            "Failed to access directory. Does it exist or lacking permissions?",
        ));
        exit(1);
    }

    let files = files.unwrap();

    let mut compiler = Compiler::new(namespace);
    for file in files.into_iter() {
        let file = file.unwrap();
        let path = file.path();
        let code = fs::read_to_string(path);
        if code.is_err() {
            std_error(StdErrors::IOError(
                "Failed to access file. Does it exist or lacking permissions?",
            ));
            exit(1);
        }
        let code: String = code.unwrap();
        compiler.prepared_files.insert(
            file.file_name()
                .into_string()
                .unwrap()
                .strip_suffix(".mc")
                .unwrap()
                .to_string(),
            code,
        );
    }

    // compile the code scope
    let code = compiler.prepared_files.get("code").unwrap();

    let mut lexer = Lexer::new(code.to_string());
    lexer.tokenizer();
    let mut ast = AST::new(lexer.flush().to_vec());
    ast.generate();
    let mut scope = Scope::new(
        format!("{}", "code"),
        "test".to_string(),
        ast.flush().to_vec(),
        HashMap::new(),
    );
    compiler.scopes.push(scope.clone());
    compiler.compile(&mut scope);

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
