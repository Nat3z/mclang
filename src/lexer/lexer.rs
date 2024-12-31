use std::process::exit;

use crate::errors::error::{std_error, StdErrors};

use super::tokens::Tokens;

pub struct Lexer {
    raw_tokens: Vec<Vec<char>>,
    line: usize,
    column: usize,
    tokens: Vec<Tokens>,
}

impl Lexer {
    pub fn new(file: String) -> Lexer {
        let mut raw_tokens: Vec<Vec<char>> = vec![];
        for line in file.split("\n").collect::<Vec<&str>>() {
            raw_tokens.push(line.chars().collect());
        }
        Lexer {
            raw_tokens,
            line: 0usize,
            column: 0usize,
            tokens: vec![],
        }
    }
    pub fn flush(&self) -> &Vec<Tokens> {
        &self.tokens
    }

    pub fn peek(&self, forward: usize) -> char {
        if self.column + forward > self.raw_tokens[self.line - 1].len() {
            return '\0';
        }
        self.raw_tokens[self.line - 1][self.column - 1 + forward]
    }

    pub fn read_until(&self, token: &str) -> (String, usize) {
        let mut tracked_col = 0;
        let mut built_str = String::new();
        while !built_str.ends_with(token) {
            tracked_col += 1;
            let token = self.peek(tracked_col);
            if token == '\0' {
                break;
            }
            built_str.push(token);
        }

        if !built_str.contains(token) {
            std_error(StdErrors::SyntaxError(
                format!("Expected: {}", token),
                self.raw_tokens[self.line - 1].iter().collect(),
                self.line,
                self.column,
            ));
            exit(1);
        }
        return (built_str.replace(token, ""), tracked_col - 1);
    }

    pub fn read_until_end(&self) -> (String, usize) {
        let mut tracked_col = 0;
        let mut built_str = String::new();
        while self.column + tracked_col < self.raw_tokens[self.line - 1].len() {
            tracked_col += 1;
            let token = self.peek(tracked_col);
            if token == '\0' {
                break;
            }
            built_str.push(token);
        }
        return (built_str, tracked_col);
    }

    pub fn read_until_last(&self, open_token: char, opposite_token: char) -> (String, usize) {
        let mut tracked_col = 0;
        let mut built_str = String::new();
        let mut counted_opens = 1;
        while !built_str.ends_with(opposite_token) || counted_opens != 0 {
            tracked_col += 1;
            let token = self.peek(tracked_col);
            if token == '\0' {
                break;
            }

            if token == opposite_token {
                counted_opens -= 1;
            } else if token == open_token {
                counted_opens += 1;
            }
            built_str.push(token);
        }

        // if !built_str.contains(open_token) {
        //     std_error(StdErrors::SyntaxError(format!("Expected: {}", open_token), self.raw_tokens[self.line - 1].iter().collect(), self.line, self.column));
        //     exit(1);
        // }
        //
        if counted_opens > 0 {
            std_error(StdErrors::SyntaxError(
                format!(
                    "Opened token {} but did not close with {}",
                    open_token, opposite_token
                ),
                self.raw_tokens[self.line - 1].iter().collect(),
                self.line,
                self.column,
            ));
            exit(1);
        }
        if counted_opens < 0 {
            std_error(StdErrors::SyntaxError(
                format!(
                    "Closed token {} but did not open with {}",
                    opposite_token, open_token
                ),
                self.raw_tokens[self.line - 1].iter().collect(),
                self.line,
                self.column,
            ));
            exit(1);
        }

        let new_str = built_str
            .chars()
            .rev()
            .collect::<String>()
            .replacen(opposite_token, "", 1)
            .chars()
            .rev()
            .collect::<String>();
        return (new_str.trim().to_string(), tracked_col);
    }

    pub fn tokenizer(&mut self) {
        for raw_line in &self.raw_tokens {
            self.line += 1;
            self.column = 0;
            let mut built_str = String::new();

            while self.column != raw_line.len() {
                self.column += 1;
                let char = self.peek(0);
                if char == '\0' {
                    break;
                }
                built_str.push(char);
                match built_str.trim_start() {
                    "let " | "const " => {
                        let constant = if built_str.trim_start() == "const " {
                            true
                        } else {
                            false
                        };

                        built_str.clear();
                        let (var_name, forwardness) = self.read_until("=");

                        self.column += forwardness;

                        let var_name = if constant {
                            "*".to_string() + &var_name.trim().to_string()
                        } else {
                            var_name.trim().to_string()
                        };

                        self.tokens.push(Tokens::Let(var_name));
                    }
                    "=" => {
                        // this means that this is an equivalence operator
                        built_str.clear();
                        if self.peek(1) == '=' {
                            self.column += 1;
                            self.tokens.push(Tokens::Equivalence);
                        }
                        // otherwise, this is just a single assignment operator
                        else {
                            self.tokens.push(Tokens::Assignment);
                        }
                    }
                    "!=" => {
                        built_str.clear();
                        self.tokens.push(Tokens::NotEqual);
                    }
                    ">" => {
                        built_str.clear();
                        if self.peek(1) == '=' {
                            self.column += 1;
                            self.tokens.push(Tokens::GreaterThanEqual);
                        } else {
                            self.tokens.push(Tokens::GreaterThan);
                        }
                    }
                    "<" => {
                        built_str.clear();
                        if self.peek(1) == '=' {
                            self.column += 1;
                            self.tokens.push(Tokens::LesserThanEqual);
                        } else {
                            self.tokens.push(Tokens::LesserThan);
                        }
                    }
                    "if " => {
                        built_str.clear();
                        let (boolean, forwardness) = self.read_until("{");
                        self.column += forwardness;
                        let mut lexer = Lexer::new(boolean.trim().to_string());
                        lexer.tokenizer();
                        let mut tokens = lexer.flush().to_vec();
                        // remove the last 2 tokens as those are just EOL EOF
                        tokens.remove(tokens.len() - 1);
                        tokens.remove(tokens.len() - 1);
                        self.tokens.push(Tokens::If(tokens));
                    }
                    "while " => {
                        built_str.clear();
                        let (statements, forwardness) = self.read_until("{");
                        self.column += forwardness;
                        // now split statements between the first colon
                        let statements = statements.splitn(2, "=").collect::<Vec<&str>>();
                        // make parser just get the name
                        let name_statement = format!("{} =", statements[0]);
                        // name parser
                        let mut lexer = Lexer::new(name_statement.trim().to_string());
                        lexer.tokenizer();
                        let mut tokens = lexer.flush().to_vec();
                        tokens.remove(tokens.len() - 1);
                        tokens.remove(tokens.len() - 1);
                        if tokens.len() != 2 {
                            eprintln!("More than 1 token in while loop declaration.");
                            exit(1);
                        }
                        let let_name: String = match &tokens[0] {
                            Tokens::Let(name) => name,
                            _ => "",
                        }
                        .to_string();
                        if let_name.is_empty() {
                            eprintln!("Missing let token name.");
                            exit(1);
                        }
                        let mut lexer = Lexer::new(statements[1].trim().to_string());
                        lexer.tokenizer();
                        let mut tokens = lexer.flush().to_vec();
                        tokens.remove(tokens.len() - 1);
                        tokens.remove(tokens.len() - 1);
                        self.tokens.push(Tokens::While(let_name, tokens));
                    }
                    "{" => {
                        built_str.clear();
                        self.tokens.push(Tokens::LBrace);
                    }
                    "}" => {
                        built_str.clear();
                        self.tokens.push(Tokens::RBrace);
                    }
                    "(" => {
                        built_str.clear();
                        let (boolean, forwardness) = self.read_until_last('(', ')');
                        self.column += forwardness;
                        let mut lexer = Lexer::new(boolean.trim().to_string());
                        lexer.tokenizer();
                        let mut tokens = lexer.flush().to_vec();
                        // remove the last 2 tokens as those are just EOL EOF
                        tokens.remove(tokens.len() - 1);
                        tokens.remove(tokens.len() - 1);
                        self.tokens.push(Tokens::Parens(tokens));
                    }
                    "\"" => {
                        built_str.clear();
                        let (string, forwardness) = self.read_until("\"");
                        self.column += forwardness + 1;
                        self.tokens.push(Tokens::DblQuote(string));
                    }
                    "[" => {
                        built_str.clear();
                        let (inside_parens, forwardness) = self.read_until_last('[', ']');

                        self.column += forwardness;
                        let mut lexer = Lexer::new(inside_parens.trim().to_string());
                        lexer.tokenizer();
                        let mut tokens = lexer.flush().to_vec();
                        // remove the last 2 tokens as those are just EOL EOF
                        tokens.remove(tokens.len() - 1);
                        tokens.remove(tokens.len() - 1);
                        self.tokens.push(Tokens::Bracket(tokens));
                    }
                    "new " => {
                        built_str.clear();
                        let (object_name, forwardness) = self.read_until("(");
                        // add forwardness including the "(" as read_until does not include it but
                        // we know it will be there.

                        self.column += forwardness + 1;
                        let (inside_parens, forwardness) = self.read_until_last('(', ')');
                        self.column += forwardness;
                        let mut lexer = Lexer::new(inside_parens.trim().to_string());
                        lexer.tokenizer();
                        let mut tokens = lexer.flush().to_vec();
                        // remove the last 2 tokens as those are just EOL EOF
                        tokens.remove(tokens.len() - 1);
                        tokens.remove(tokens.len() - 1);
                        // tokens.remove(tokens.len() - 1);
                        self.tokens
                            .push(Tokens::New(object_name.trim().to_string(), tokens));
                    }
                    "." => {
                        built_str.clear();
                        let (mut statements, forwardness) = self.read_until_end();
                        // exclude the semicolon
                        // remove operands starting at && and to the end
                        let mut point_to_grab = forwardness;
                        for state in [
                            "&&", "||", "==", ">", "<", ">=", "<=", "+", "-", "*", "/", "%", "^",
                        ] {
                            if statements.contains(state) {
                                point_to_grab = statements.find(state).unwrap();
                                statements = statements.split_at(point_to_grab).0.to_string();
                            }
                        }

                        // remove point to grab from forwardness
                        let forwardness = point_to_grab;
                        // let statements = statements.split_at(point_to_grab).0;
                        println!("Statements: {}", statements);
                        let rev_string = statements.chars().rev().collect::<String>();
                        let has_semicolon = rev_string.trim().chars().next() == Some(';');
                        let statements: String =
                            rev_string.replacen(';', "", 1).chars().rev().collect();

                        self.column += forwardness;
                        let mut lexer = Lexer::new(statements.trim().to_string());
                        lexer.tokenizer();
                        let mut tokens = lexer.flush().to_vec();
                        // remove the last 2 tokens as those are just EOL EOF
                        tokens.remove(tokens.len() - 1);
                        tokens.remove(tokens.len() - 1);
                        self.tokens.push(Tokens::Period(tokens));
                        if has_semicolon {
                            self.tokens.push(Tokens::SemiColon);
                        }
                    }
                    "&&" => {
                        built_str.clear();
                        self.tokens.push(Tokens::And);
                    }
                    "||" => {
                        built_str.clear();
                        self.tokens.push(Tokens::Or);
                    }
                    "," => {
                        built_str.clear();
                        self.tokens.push(Tokens::Comma);
                    }
                    "+" => {
                        built_str.clear();
                        self.tokens.push(Tokens::Add);
                    }
                    "-" => {
                        built_str.clear();
                        self.tokens.push(Tokens::Subtract);
                    }
                    "*" => {
                        built_str.clear();
                        self.tokens.push(Tokens::Multiply);
                    }
                    "/" => {
                        built_str.clear();
                        self.tokens.push(Tokens::Divide);
                    }
                    "%" => {
                        built_str.clear();
                        self.tokens.push(Tokens::Modulus);
                    }
                    "true" => {
                        built_str.clear();
                        self.tokens.push(Tokens::Bool(true));
                    }
                    "false" => {
                        built_str.clear();
                        self.tokens.push(Tokens::Bool(false));
                    }
                    ";" => {
                        built_str.clear();
                        self.tokens.push(Tokens::SemiColon);
                    }
                    _ => {}
                }
                // additionally check if the built_str is a number
                if built_str.trim_start().parse::<i32>().is_ok() {
                    // continue peeking until it is not a number
                    let mut okay_number = String::new();
                    let mut tracked_col = 0usize;
                    while built_str.trim().parse::<i32>().is_ok() {
                        okay_number = String::from(&built_str);
                        tracked_col += 1;
                        let token = self.peek(tracked_col);
                        if token == '\0' {
                            break;
                        }
                        built_str.push(token);
                    }
                    self.column += tracked_col - 1;
                    self.tokens
                        .push(Tokens::Number(okay_number.trim().to_string()));
                    built_str.clear();
                }

                // now peek to see if this is just a big symbol
                let char = self.peek(1);
                if built_str.trim().len() > 0
                    && (self.column == self.raw_tokens[self.line - 1].len()
                        || char == '.'
                        || char == '('
                        || char == ';'
                        || char == ')'
                        || char == '='
                        || char == '>'
                        || char == '<'
                        || char == ','
                        || char == '+'
                        || char == '-'
                        || char == '*'
                        || char == '/'
                        || char == '%'
                        || (char == '&' && self.peek(2) == '&')
                        || (char == '!' && self.peek(2) == '=')
                        || (char == '|' && self.peek(2) == '|'))
                {
                    self.tokens
                        .push(Tokens::Symbol(built_str.trim().to_string()));
                    built_str.clear();
                }
            }
            self.tokens.push(Tokens::EOL);
            if built_str.trim().len() > 0 {
                println!("{:?}", self.tokens);
                std_error(StdErrors::SyntaxError(
                    "Unknown token".to_string(),
                    raw_line.iter().collect(),
                    self.line,
                    self.column,
                ));
                eprintln!("{}", built_str);
                exit(1);
            }
        }

        self.tokens.push(Tokens::EOF);
        // pretty print
        println!("\n--\n");
        for token in &self.tokens {
            if *token == Tokens::EOL {
                print!(" {:?} \n", token);
                continue;
            }
            print!(" {:?} ", token);
        }
        println!("\n--\n");
        // println!("--\n{:?}", self.tokens);
    }
}
