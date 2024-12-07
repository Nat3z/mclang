use std::{mem::discriminant, process::exit};

use crate::lexer::tokens::Tokens;

use super::operations::{NodeStatement, ASTOperation};

pub struct AST {
    statements: Vec<ASTOperation>,
    tokens: Vec<Tokens>,
    index: usize
}

impl AST {
    pub fn new(tokens: Vec<Tokens>) -> AST {
        AST {
            statements: vec![],
            tokens,
            index: 0
        }
    }
    pub fn peek(&self, forward: usize) -> Tokens {
        if self.index + forward >= self.tokens.len() {
            return Tokens::None
        }
        self.tokens[self.index + forward].clone()
    }

    pub fn flush(&self) -> &Vec<ASTOperation> {
        &self.statements
    }

    pub fn get_tokens_until(&self, token: Tokens) -> (Vec<Tokens>, usize) {
        let mut tokens = vec![];
        let mut forwardness = 0;
        while self.tokens.len() != self.index + forwardness {
            let current_token = self.tokens[self.index + forwardness].clone();
            if current_token == token {
                break;
            }
            tokens.push(current_token);
            forwardness += 1;
        }
        (tokens, forwardness)
    }

    pub fn get_statements_from_tokens(&self, tokens: &Vec<Tokens>) -> Vec<ASTOperation> {
        let mut ast = AST::new(tokens.to_vec());
        ast.generate();
        let statements = ast.flush();
        if statements.len() > 1 {
            return vec![ ASTOperation::Set(statements.to_vec()) ];
        }
        return ast.flush().to_vec();
    }
    pub fn generate(&mut self) {

        let mut position_in_line = 1;
        while self.tokens.len() > self.index {
            let current_token = self.peek(0);

            match current_token {
                Tokens::Let(name) => {
                    if self.peek(1) != Tokens::Assignment {
                        eprintln!("Expected assignment operator.");
                        exit(1);
                    }
                    self.index += 1;
                    let statements = self.get_statements_from_tokens(&vec![ self.peek(1) ]);
                    self.index += 1;
                    self.statements.push(ASTOperation::AssignVariable(name, statements.to_vec()));
                },
                Tokens::Number(str) => {
                    self.statements.push(ASTOperation::LiteralNumber(str.parse().unwrap()));
                },
                Tokens::Bool(bool) => {
                    self.statements.push(ASTOperation::LiteralBool(bool));
                },
                Tokens::Symbol(reference) => {
                    let next_token = self.peek(1);
                    println!("{:?} {}", next_token, position_in_line);
                    if next_token == Tokens::Assignment {
                        self.index += 1;
                        let statements = self.get_statements_from_tokens(&vec![ self.peek(1) ]);
                        self.index += 1;
                        self.statements.push(ASTOperation::MutateVariable(reference, statements.to_vec()));
                    }
                    else if discriminant(&Tokens::Period(vec![])) == discriminant(&next_token) {
                        self.index += 1;
                        let (tokens, forwardness) = self.get_tokens_until(Tokens::SemiColon);
                        let statements = self.get_statements_from_tokens(&tokens);
                        if statements.len() > 1 || statements.len() == 0 {
                            eprintln!("Expected single statement.");
                            exit(1);
                        }
                        self.index += forwardness;
                        self.statements.push(ASTOperation::UseVariable(
                            reference, 
                            Box::new(
                                statements[0].clone()
                            )
                        ));
                    }
                    else if next_token == Tokens::Equivalence {
                        self.index += 1;
                        let (tokens, forwardness) = self.get_tokens_until(Tokens::LBrace);
                        let statements = self.get_statements_from_tokens(&tokens);
                        if statements.len() > 1 || statements.len() == 0 {
                            eprintln!("Expected single statement.");
                            exit(1);
                        }
                        self.index += forwardness;
                        self.statements.push(ASTOperation::UseVariable(
                            reference, 
                            Box::new(
                                statements[0].clone()
                            )
                        ));
                    }
                    else if discriminant(&Tokens::Parens(vec![])) == discriminant(&next_token) {
                        let statements = self.get_statements_from_tokens(&vec![ self.peek(1) ]);
                        if statements.len() > 1 || statements.len() == 0 {
                            eprintln!("Expected single statement.");
                            exit(1);
                        }
                        self.index += 1;
                        self.statements.push(ASTOperation::Function(reference, statements.to_vec()));
                    }
                    else {
                        self.statements.push(ASTOperation::Access(reference));
                    }
                },
                Tokens::If(conditional_tokens) => {
                    let conditional_statements = self.get_statements_from_tokens(&conditional_tokens);
                    // expect a Left curly brace
                    if self.peek(1) != Tokens::LBrace {
                        eprintln!("Expected Left curly brace.");
                        exit(1);
                    }
                    self.index += 1;
                    let (tokens, forwardness) = self.get_tokens_until(Tokens::RBrace);
                    let statements = self.get_statements_from_tokens(&tokens);
                    self.index += forwardness;
                    self.statements.push(ASTOperation::If(
                        conditional_statements.to_vec(),
                        Box::new(ASTOperation::CodeBlock(statements.to_vec()))
                    ));
                },
                Tokens::Period(statements) => {
                    let statements = self.get_statements_from_tokens(&statements);
                    self.statements.push(ASTOperation::AccessPart(Box::new(statements[0].clone())));
                },
                Tokens::New(obj_name, statement_tokens) => {
                    let statements = self.get_statements_from_tokens(&statement_tokens);
                    self.statements.push(ASTOperation::Create(obj_name, statements.to_vec()));
                },

                Tokens::Parens(statement_tokens) => {
                    let statements = self.get_statements_from_tokens(&statement_tokens);
                    self.statements.push(ASTOperation::Set(statements.to_vec()));
                },

                Tokens::SemiColon => {
                    position_in_line = 0;
                },
                Tokens::EOL | Tokens::EOF => {
                    position_in_line -= 1;
                },
                _ => {
                }
            }
            position_in_line += 1;
            self.index += 1;
        }

        println!("{:?}", self.statements);
    }
}
