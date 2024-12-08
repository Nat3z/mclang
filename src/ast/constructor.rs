use std::{mem::discriminant, process::exit};

use crate::{ast::operations::Operator, lexer::tokens::Tokens};

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
    
    pub fn last(&self, backwardness: i32) -> Tokens {
        if self.index as i32 - backwardness < 0 {
            return Tokens::None
        }
        self.tokens[(self.index as i32 - backwardness) as usize].clone()
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

    pub fn get_tokens_until_mult(&self, tokens: Vec<Tokens>) -> (Vec<Tokens>, usize, Tokens) {
        let mut sent_tokens = vec![];
        let mut forwardness = 0;
        let mut ending_token = Tokens::EOL;
        while self.tokens.len() != self.index + forwardness {
            let current_token = self.tokens[self.index + forwardness].clone();
            if tokens.contains(&current_token) {
                ending_token = current_token;
                forwardness -= 1;
                break;
            }
            sent_tokens.push(current_token);
            forwardness += 1;
        }
        (sent_tokens, forwardness, ending_token)
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
        let mut operand: Option<Operator> = None; 
        let mut combind_ifs: Option<Operator> = None;
        while self.tokens.len() > self.index {
            let current_token = self.peek(0);
            match current_token {
                Tokens::Let(name) => {
                    if self.peek(1) != Tokens::Assignment {
                        eprintln!("Expected assignment operator.");
                        exit(1);
                    }
                    self.index += 1;
                    let (statements, forwardness) = self.get_tokens_until(Tokens::SemiColon);
                    let statements = self.get_statements_from_tokens(&statements);
                    self.index += forwardness;
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
                    if next_token == Tokens::Assignment {
                        self.index += 1;
                        let (statements, forwardness) = self.get_tokens_until(Tokens::SemiColon);
                        let statements = self.get_statements_from_tokens(&statements);
                        self.index += forwardness;
                        self.statements.push(ASTOperation::MutateVariable(reference, statements.to_vec()));
                    }
                    else if discriminant(&Tokens::Period(vec![])) == discriminant(&next_token) {
                        self.index += 1;
                        let (tokens, forwardness, _) = self.get_tokens_until_mult([ Tokens::SemiColon, Tokens::And, Tokens::Or, Tokens::Equivalence ].to_vec());

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

                Tokens::Add => {
                    let next_token = self.peek(1);
                    let last_token = self.last(1);
                    if next_token == Tokens::Assignment && discriminant(&last_token) == discriminant(&Tokens::Symbol("".to_string())) {
                        self.statements.pop();
                        if let Tokens::Symbol(reference) = last_token {
                            self.index += 1;
                            let (statements, forwardness) = self.get_tokens_until(Tokens::SemiColon);
                            let statements = self.get_statements_from_tokens(&statements);
                            self.index += forwardness;
                            self.statements.push(ASTOperation::MutateVariable(
                                reference.clone(), 
                                vec![ ASTOperation::Operation(
                                    Box::new(ASTOperation::Access(reference)),
                                    Operator::Add,
                                    Box::new(statements[0].clone())
                                )]
                            ));
                        }
                    }
                    else {
                        operand = Some(Operator::Add);
                    }
                },

                Tokens::Parens(statement_tokens) => {
                    let statements = self.get_statements_from_tokens(&statement_tokens);
                    self.statements.push(ASTOperation::Set(statements.to_vec()));
                },

                Tokens::And => {
                    combind_ifs = Some(Operator::And);
                    // priming operation for next iteration
                    position_in_line += 1;
                    self.index += 1;
                    continue;
                },
                Tokens::Or => {
                    combind_ifs = Some(Operator::Or);
                    // priming operation for next iteration
                    position_in_line += 1;
                    self.index += 1;
                    continue;
                },

                Tokens::Equivalence => {
                    operand = Some(Operator::Equal);
                    // priming operation for next iteration
                    position_in_line += 1;
                    self.index += 1;
                    continue;
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
            if operand.is_some() {
                let pop_last = self.statements.pop();
                let pop_second = self.statements.pop();

                let pop_second_discrim = discriminant(pop_second.as_ref().unwrap());

                if pop_last.is_some() && pop_second.is_some() && (
                        pop_second_discrim == discriminant(&ASTOperation::Access("".to_string())) ||
                        pop_second_discrim == discriminant(&ASTOperation::LiteralNumber(0)) ||
                        pop_second_discrim == discriminant(&ASTOperation::LiteralBool(true)) ||
                        pop_second_discrim == discriminant(&ASTOperation::UseVariable(
                            "".to_string(), Box::new(ASTOperation::LiteralNumber(0))
                        ))
                    )
                {
                    self.statements.push(ASTOperation::Operation(
                        Box::new(pop_second.unwrap()),
                        operand.as_ref().unwrap().clone(),
                        Box::new(pop_last.unwrap())
                    ));
                    operand = None;
                }
                else {
                    if pop_second.is_some() {
                        self.statements.push(pop_second.unwrap());
                    }
                    if pop_last.is_some() {
                        self.statements.push(pop_last.unwrap());
                    }
                }
            }

            if combind_ifs.is_some() {
                let pop_last = self.statements.pop();
                let pop_second = self.statements.pop();
                // check if both are operations with discriminant
                if pop_last.is_some() && pop_second.is_some() {
                    let pop_last_discrim = discriminant(pop_last.as_ref().unwrap());
                    if 
                        pop_last_discrim == discriminant(
                            &ASTOperation::Operation(
                                Box::new(ASTOperation::LiteralBool(true)),
                                Operator::Equal,
                                Box::new(ASTOperation::LiteralBool(true))
                            )
                        )
                    {
                        self.statements.push(ASTOperation::Operation(
                            Box::new(pop_second.unwrap()),
                            combind_ifs.as_ref().unwrap().clone(),
                            Box::new(pop_last.unwrap())
                        ));
                    }
                    // or if it's a Set
                    else if 
                        pop_last_discrim == discriminant(
                            &ASTOperation::Set(vec![])
                        )
                    {
                        self.statements.push(ASTOperation::Operation(
                            Box::new(pop_second.unwrap()),
                            combind_ifs.as_ref().unwrap().clone(),
                            Box::new(pop_last.unwrap())
                        ));
                    }
                    else {
                        if pop_second.is_some() {
                            self.statements.push(pop_second.unwrap());
                        }
                        if pop_last.is_some() {
                            self.statements.push(pop_last.unwrap());
                        }
                    }
                }
            }

            position_in_line += 1;
            self.index += 1;
        }

    }
}
