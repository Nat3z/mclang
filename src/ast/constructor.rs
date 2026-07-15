use std::{
    mem::{discriminant, Discriminant},
    process::exit,
};

use crate::{
    ast::operations::Operator,
    errors::associate::CodeAssociate,
    lexer::{lexer::empty_associate, tokens::Tokens},
};

use super::operations::{ASTOperation, NodeStatement};

pub struct AST {
    statements: Vec<ASTOperation>,
    tokens: Vec<Tokens>,
    index: usize,
}

impl AST {
    pub fn new(tokens: Vec<Tokens>) -> AST {
        AST {
            statements: vec![],
            tokens,
            index: 0,
        }
    }
    pub fn peek(&self, forward: usize) -> Tokens {
        if self.index + forward >= self.tokens.len() {
            return Tokens::None;
        }
        self.tokens[self.index + forward].clone()
    }

    pub fn last(&self, backwardness: i32) -> Tokens {
        if self.index as i32 - backwardness < 0 {
            return Tokens::None;
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
            if discriminant(&current_token) == discriminant(&token) {
                break;
            }
            tokens.push(current_token);
            forwardness += 1;
        }
        (tokens, forwardness)
    }

    pub fn get_tokens_until_mult(
        &self,
        original_tokens: Vec<Tokens>,
    ) -> (Vec<Tokens>, usize, Tokens) {
        let mut sent_tokens = vec![];
        let mut forwardness = 0;
        let mut ending_token = Tokens::EOL;
        let mut tokens: Vec<Discriminant<Tokens>> = vec![];
        for token in original_tokens {
            tokens.push(discriminant(&token));
        }

        while self.tokens.len() != self.index + forwardness {
            let current_token = self.tokens[self.index + forwardness].clone();
            if tokens.contains(&discriminant(&current_token)) {
                ending_token = current_token;
                forwardness -= 1;
                break;
            }
            sent_tokens.push(current_token);
            forwardness += 1;
        }
        (sent_tokens, forwardness, ending_token)
    }

    pub fn get_statements_from_tokens(
        &self,
        tokens: &Vec<Tokens>,
        associate: CodeAssociate,
    ) -> Vec<ASTOperation> {
        let mut ast = AST::new(tokens.to_vec());
        ast.generate();
        let statements = ast.flush();
        if statements.len() > 1 {
            return vec![ASTOperation::Set(statements.to_vec(), associate)];
        }
        return ast.flush().to_vec();
    }
    pub fn generate(&mut self) {
        let mut position_in_line = 1;
        let mut operand: Option<Operator> = None;
        let mut combind_ifs: Option<Operator> = None;
        let mut export_next = false;
        while self.tokens.len() > self.index {
            let current_token = self.peek(0);
            match current_token {
                Tokens::Let(name, associate) => {
                    if discriminant(&self.peek(1))
                        != discriminant(&Tokens::Assignment(empty_associate()))
                    {
                        eprintln!("Expected assignment operator.");
                        exit(1);
                    }
                    // check the name to see if it's a static variable
                    self.index += 1;
                    let (statements, forwardness) =
                        self.get_tokens_until(Tokens::SemiColon(empty_associate()));
                    let statements =
                        self.get_statements_from_tokens(&statements, associate.clone());
                    self.index += forwardness;

                    let operation = if name.starts_with("*") {
                        ASTOperation::StaticVariable(
                            name.replacen("*", "", 1),
                            statements.to_vec(),
                            associate.clone(),
                        )
                    } else {
                        ASTOperation::AssignVariable(name, statements.to_vec(), associate.clone())
                    };

                    if export_next {
                        self.statements
                            .push(ASTOperation::Export(Box::new(operation), associate.clone()));
                        export_next = false;
                    } else {
                        self.statements.push(operation);
                    }
                }
                Tokens::Bracket(tokens, associate) => {
                    let statements = self.get_statements_from_tokens(&tokens, associate.clone());
                    self.statements
                        .push(ASTOperation::Set(statements, associate));
                }
                Tokens::Number(str, associate) => {
                    self.statements
                        .push(ASTOperation::LiteralNumber(str.parse().unwrap(), associate));
                }
                Tokens::DblQuote(str, associate) => {
                    self.statements
                        .push(ASTOperation::LiteralString(str, associate));
                }
                Tokens::Bool(bool, associate) => {
                    self.statements
                        .push(ASTOperation::LiteralBool(bool, associate));
                }
                Tokens::Export(associate) => {
                    export_next = true;
                }
                Tokens::Import(name, associate) => {
                    self.statements.push(ASTOperation::Import(name, associate));
                }
                Tokens::Symbol(reference, associate) => {
                    let next_token = self.peek(1);
                    if discriminant(&next_token)
                        == discriminant(&Tokens::Assignment(empty_associate()))
                    {
                        self.index += 1;
                        let (statements, forwardness) =
                            self.get_tokens_until(Tokens::SemiColon(empty_associate()));
                        let statements =
                            self.get_statements_from_tokens(&statements, associate.clone());
                        self.index += forwardness;
                        self.statements.push(ASTOperation::AssignVariable(
                            reference,
                            statements.to_vec(),
                            associate,
                        ));
                    } else if discriminant(&Tokens::Period(vec![], empty_associate()))
                        == discriminant(&next_token)
                    {
                        self.index += 1;
                        let (tokens, forwardness, _) = self.get_tokens_until_mult(
                            [
                                Tokens::SemiColon(empty_associate()),
                                Tokens::And(empty_associate()),
                                Tokens::Or(empty_associate()),
                                Tokens::LesserThan(empty_associate()),
                                Tokens::LesserThanEqual(empty_associate()),
                                Tokens::GreaterThan(empty_associate()),
                                Tokens::GreaterThanEqual(empty_associate()),
                                Tokens::NotEqual(empty_associate()),
                                Tokens::Equivalence(empty_associate()),
                            ]
                            .to_vec(),
                        );

                        let statements =
                            self.get_statements_from_tokens(&tokens, associate.clone());
                        if statements.len() > 1 || statements.len() == 0 {
                            eprintln!("Expected single statement.");
                            exit(1);
                        }
                        self.index += forwardness;

                        self.statements.push(ASTOperation::UseVariable(
                            reference,
                            Box::new(statements[0].clone()),
                            associate,
                        ));
                    } else if discriminant(&Tokens::Parens(vec![], empty_associate()))
                        == discriminant(&next_token)
                    {
                        let statements =
                            self.get_statements_from_tokens(&vec![self.peek(1)], empty_associate());
                        if statements.len() > 1 || statements.len() == 0 {
                            eprintln!("Expected single statement.");
                            exit(1);
                        }
                        self.index += 1;
                        self.statements.push(ASTOperation::Function(
                            reference,
                            statements.to_vec(),
                            associate,
                        ));
                    } else {
                        self.statements
                            .push(ASTOperation::Access(reference, associate));
                    }
                }
                Tokens::If(conditional_tokens, associate) => {
                    let conditional_statements =
                        self.get_statements_from_tokens(&conditional_tokens, associate.clone());
                    // expect a Left curly brace
                    if discriminant(&self.peek(1))
                        != discriminant(&Tokens::LBrace(empty_associate()))
                    {
                        eprintln!("Expected Left curly brace.");
                        exit(1);
                    }
                    self.index += 1;
                    let (tokens, forwardness) =
                        self.get_tokens_until(Tokens::RBrace(empty_associate()));
                    let statements = self.get_statements_from_tokens(&tokens, associate.clone());
                    self.index += forwardness;

                    // TODO: Fix the associater here
                    self.statements.push(ASTOperation::If(
                        conditional_statements.to_vec(),
                        Box::new(ASTOperation::CodeBlock(
                            statements.to_vec(),
                            associate.clone(),
                        )),
                        associate,
                    ));
                }
                Tokens::Function(name, variables, associate) => {
                    if discriminant(&self.peek(1))
                        != discriminant(&Tokens::LBrace(empty_associate()))
                    {
                        eprintln!("Expected Left curly brace.");
                        exit(1);
                    }
                    self.index += 1;
                    let (tokens, forwardness) =
                        self.get_tokens_until(Tokens::RBrace(empty_associate()));
                    let statements = self.get_statements_from_tokens(&tokens, associate.clone());
                    self.index += forwardness;
                    let mut assigned_variables: Vec<String> = vec![];
                    for variable in variables {
                        if let Tokens::Symbol(str, _) = variable {
                            assigned_variables.push(str);
                        } else {
                            eprintln!("Expected variable name.");
                            exit(1);
                        }
                    }

                    if export_next {
                        self.statements.push(ASTOperation::Export(
                            Box::new(ASTOperation::CreateFunction(
                                name,
                                assigned_variables,
                                statements.to_vec(),
                                associate.clone(),
                            )),
                            associate.clone(),
                        ));
                        export_next = false;
                    } else {
                        self.statements.push(ASTOperation::CreateFunction(
                            name,
                            assigned_variables,
                            statements.to_vec(),
                            associate.clone(),
                        ));
                    }
                }
                Tokens::While(name, iterator_tokens, associate) => {
                    let iterator_statements =
                        self.get_statements_from_tokens(&iterator_tokens, associate.clone());
                    // expect a Left curly brace
                    if discriminant(&self.peek(1))
                        != discriminant(&Tokens::LBrace(empty_associate()))
                    {
                        eprintln!("Expected Left curly brace.");
                        exit(1);
                    }
                    self.index += 1;
                    let (tokens, forwardness) =
                        self.get_tokens_until(Tokens::RBrace(empty_associate()));
                    let statements = self.get_statements_from_tokens(&tokens, associate.clone());
                    self.index += forwardness;
                    // TODO: Here too :)
                    self.statements.push(ASTOperation::While(
                        name,
                        iterator_statements.to_vec(),
                        Box::new(ASTOperation::CodeBlock(
                            statements.to_vec(),
                            associate.clone(),
                        )),
                        associate.clone(),
                    ));
                }
                Tokens::Period(statements, associate) => {
                    let statements =
                        self.get_statements_from_tokens(&statements, associate.clone());
                    self.statements.push(ASTOperation::AccessPart(
                        Box::new(statements[0].clone()),
                        associate,
                    ));
                }
                Tokens::New(obj_name, statement_tokens, associate) => {
                    let statements =
                        self.get_statements_from_tokens(&statement_tokens, associate.clone());
                    self.statements.push(ASTOperation::Create(
                        obj_name,
                        statements.to_vec(),
                        associate,
                    ));
                }

                Tokens::Add(associate) => {
                    let next_token = self.peek(1);
                    let last_token = self.last(1);
                    if discriminant(&next_token)
                        == discriminant(&Tokens::Assignment(empty_associate()))
                        && discriminant(&last_token)
                            == discriminant(&Tokens::Symbol("".to_string(), empty_associate()))
                    {
                        self.statements.pop();
                        if let Tokens::Symbol(reference, associate) = last_token {
                            self.index += 1;
                            let (statements, forwardness) =
                                self.get_tokens_until(Tokens::SemiColon(empty_associate()));
                            let statements =
                                self.get_statements_from_tokens(&statements, associate.clone());
                            self.index += forwardness;
                            self.statements.push(ASTOperation::MutateVariable(
                                reference.clone(),
                                vec![ASTOperation::Operation(
                                    Box::new(ASTOperation::Access(reference, associate.clone())),
                                    Operator::Add,
                                    Box::new(statements[0].clone()),
                                    associate.clone(),
                                )],
                                associate.clone(),
                            ));
                        }
                    } else {
                        operand = Some(Operator::Add);
                    }
                }
                Tokens::Subtract(associate) => {
                    let next_token = self.peek(1);
                    let last_token = self.last(1);
                    if discriminant(&next_token)
                        == discriminant(&Tokens::Assignment(empty_associate()))
                        && discriminant(&last_token)
                            == discriminant(&Tokens::Symbol("".to_string(), empty_associate()))
                    {
                        self.statements.pop();
                        if let Tokens::Symbol(reference, associate) = last_token {
                            self.index += 1;
                            let (statements, forwardness) =
                                self.get_tokens_until(Tokens::SemiColon(empty_associate()));
                            let statements =
                                self.get_statements_from_tokens(&statements, associate.clone());
                            self.index += forwardness;
                            self.statements.push(ASTOperation::MutateVariable(
                                reference.clone(),
                                vec![ASTOperation::Operation(
                                    Box::new(ASTOperation::Access(reference, associate.clone())),
                                    Operator::Subtract,
                                    Box::new(statements[0].clone()),
                                    associate.clone(),
                                )],
                                associate,
                            ));
                        }
                    } else {
                        operand = Some(Operator::Subtract);
                    }
                }

                Tokens::Parens(statement_tokens, associate) => {
                    let statements =
                        self.get_statements_from_tokens(&statement_tokens, associate.clone());
                    self.statements
                        .push(ASTOperation::Set(statements.to_vec(), associate));
                }

                Tokens::And(_) => {
                    combind_ifs = Some(Operator::And);
                    // priming operation for next iteration
                    position_in_line += 1;
                    self.index += 1;
                    continue;
                }
                Tokens::Or(_) => {
                    combind_ifs = Some(Operator::Or);
                    // priming operation for next iteration
                    position_in_line += 1;
                    self.index += 1;
                    continue;
                }

                Tokens::Equivalence(_) => {
                    operand = Some(Operator::Equal);
                    // priming operation for next iteration
                    position_in_line += 1;
                    self.index += 1;
                    continue;
                }
                Tokens::GreaterThan(_) => {
                    operand = Some(Operator::GreaterThan);
                    // priming operation for next iteration
                    position_in_line += 1;
                    self.index += 1;
                    continue;
                }
                Tokens::GreaterThanEqual(_) => {
                    operand = Some(Operator::GreaterThanEqual);
                    // priming operation for next iteration
                    position_in_line += 1;
                    self.index += 1;
                    continue;
                }
                Tokens::LesserThan(_) => {
                    operand = Some(Operator::LessThan);
                    // priming operation for next iteration
                    position_in_line += 1;
                    self.index += 1;
                    continue;
                }
                Tokens::LesserThanEqual(_) => {
                    operand = Some(Operator::LessThanEqual);
                    // priming operation for next iteration
                    position_in_line += 1;
                    self.index += 1;
                    continue;
                }
                Tokens::NotEqual(_) => {
                    operand = Some(Operator::NotEqual);
                    // priming operation for next iteration
                    position_in_line += 1;
                    self.index += 1;
                    continue;
                }
                Tokens::Multiply(_) => {
                    operand = Some(Operator::Multiply);
                    // priming operation for next iteration
                    position_in_line += 1;
                    self.index += 1;
                    continue;
                }
                Tokens::Divide(_) => {
                    operand = Some(Operator::Divide);
                    // priming operation for next iteration
                    position_in_line += 1;
                    self.index += 1;
                    continue;
                }
                Tokens::Modulus(_) => {
                    operand = Some(Operator::Modulus);
                    // priming operation for next iteration
                    position_in_line += 1;
                    self.index += 1;
                    continue;
                }

                Tokens::SemiColon(_) => {
                    position_in_line = 0;
                }
                Tokens::EOL | Tokens::EOF => {
                    position_in_line -= 1;
                }
                _ => {}
            }
            if operand.is_some() {
                let pop_last = self.statements.pop();
                let pop_second = self.statements.pop();

                let pop_second_discrim = discriminant(pop_second.as_ref().unwrap());

                if pop_last.is_some()
                    && pop_second.is_some()
                    && (pop_second_discrim
                        == discriminant(&ASTOperation::Access("".to_string(), empty_associate()))
                        || pop_second_discrim
                            == discriminant(&ASTOperation::LiteralNumber(0, empty_associate()))
                        || pop_second_discrim
                            == discriminant(&ASTOperation::LiteralBool(true, empty_associate()))
                        || pop_second_discrim
                            == discriminant(&ASTOperation::UseVariable(
                                "".to_string(),
                                Box::new(ASTOperation::LiteralNumber(0, empty_associate())),
                                empty_associate(),
                            )))
                {
                    self.statements.push(ASTOperation::Operation(
                        Box::new(pop_second.unwrap()),
                        operand.as_ref().unwrap().clone(),
                        Box::new(pop_last.unwrap()),
                        empty_associate(),
                    ));
                    operand = None;
                } else {
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
                    if pop_last_discrim
                        == discriminant(&ASTOperation::Operation(
                            Box::new(ASTOperation::LiteralBool(true, empty_associate())),
                            Operator::Equal,
                            Box::new(ASTOperation::LiteralBool(true, empty_associate())),
                            empty_associate(),
                        ))
                    {
                        self.statements.push(ASTOperation::Operation(
                            Box::new(pop_second.unwrap()),
                            combind_ifs.as_ref().unwrap().clone(),
                            Box::new(pop_last.unwrap()),
                            empty_associate(),
                        ));
                    }
                    // or if it's a Set
                    else if pop_last_discrim
                        == discriminant(&ASTOperation::Set(vec![], empty_associate()))
                    {
                        self.statements.push(ASTOperation::Operation(
                            Box::new(pop_second.unwrap()),
                            combind_ifs.as_ref().unwrap().clone(),
                            Box::new(pop_last.unwrap()),
                            empty_associate(),
                        ));
                    } else {
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
