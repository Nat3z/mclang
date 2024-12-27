use std::{any::Any, collections::HashMap, process::exit, rc::Rc };

use crate::{ast::operations::{ASTOperation, Operator}, compile::{compiler::{Compiler, Scope}, mcstatements::{execute_step_str, MinecraftStatementObject, Statements}, objects::{Object, Objects}}};

#[derive(Clone, Debug)]
pub struct VariableObject {
    pub value: Box<Objects>,
    pub scoreboard: Box<Objects>
}

#[derive(Clone, Debug)]
pub struct MutationVariableObject {
    pub variable: Box<Objects>,
    pub operator: Operator,
    pub new_value: Box<Objects>
}

#[derive(Clone, Debug)]
pub struct IfStatementObject {
    pub operations: Vec<Rc<dyn Object>>,
    pub code_block: Box<ASTOperation>
}

impl Object for VariableObject {
    fn get_type(&self) -> Objects {
        Objects::Variable(self.value.clone(), self.scoreboard.clone())
    }
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        HashMap::new()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_functions(&self) -> HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>> {
        HashMap::new()
    }
}

impl Object for IfStatementObject {
    fn get_type(&self) -> Objects {
        Objects::IfStatement(self.operations.clone(), self.code_block.clone())
    }
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        HashMap::new()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_functions(&self) -> HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>> {
        HashMap::new()
    }
}

impl Object for MutationVariableObject {
    fn get_type(&self) -> Objects {
        Objects::MutationVariable(self.variable.clone(), self.operator.clone(), self.new_value.clone())
    }
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        HashMap::new()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_functions(&self) -> HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>> {
        HashMap::new()
    }
}

pub fn compile_into_variable(var: Objects, scoreboard: Objects) -> (String, Option<Scope>) {
    if let Objects::Number(num) = var {
        if let Objects::Scoreboard(name, objective) = scoreboard {
            let mut built_str = String::new();
            built_str.push_str(&format!("scoreboard objective add {} {}\n", name, objective));
            built_str.push_str(&format!("scoreboard players set value {} {}\n", objective, num));
            return (built_str, None);
        }
        else {
            eprintln!("Invalid scoreboard");
            exit(1);
        }
    } else if let Objects::Boolean(bool) = var {
        if let Objects::Scoreboard(name, objective) = scoreboard {
            let mut built_str = String::new();
            built_str.push_str(&format!("scoreboard objective add {} {}\n", name, objective));
            built_str.push_str(&format!("scoreboard players set value {} {}\n", objective, if bool { 1 } else { 0 } ));
            return (built_str, None);
        }
        else {
            eprintln!("Invalid scoreboard");
            exit(1);
        }
    }
    else {
        // not a variable that should be compiled to text.
    }

    return (String::new(), None);
}

pub fn compile_into_mutation_variable(variable: Objects, operation: Operator, mutation: Objects) -> (String, Option<Scope>) {
    if let Objects::Variable(variable, scoreboard) = variable  {
        if let Objects::Scoreboard(name, _) = *scoreboard {
            match mutation {
                Objects::Number(num) => {
                    let mut built_str = String::new();
                    if let Operator::Assignment = operation {
                        built_str.push_str(&format!("scoreboard players set value {} {}\n", name, num));
                    }
                    else if let Operator::Add = operation {
                        built_str.push_str(&format!("scoreboard players add value {} {}\n", name, num));
                    }
                    else if let Operator::Subtract = operation {
                        built_str.push_str(&format!("scoreboard players remove value {} {}\n", name, num));
                    }
                    return (built_str, None);
                },
                Objects::Variable(_, scoreboard_second) => {
                    if let Objects::Scoreboard(scoreboard_second, _) = *scoreboard_second {
                        let mut built_str = String::new();
                        if let Operator::Assignment = operation {
                            built_str.push_str(&format!("scoreboard players operation value {} = value {}", name, scoreboard_second));
                        }
                        else if let Operator::Add = operation {
                            built_str.push_str(&format!("scoreboard players operation value {} += value {}", name, scoreboard_second));
                        }
                        else if let Operator::Subtract = operation {
                            built_str.push_str(&format!("scoreboard players operation value {} -= value {}", name, scoreboard_second));
                        }
                        return (built_str, None);
                    }
                },
                Objects::Boolean(bool) => {
                    let mut built_str = String::new();
                    built_str.push_str(&format!("scoreboard players set value {} {}\n", name, if bool { 1 } else { 0 } ));
                    return (built_str, None);
                },
                _ => {
                    eprintln!("Invalid mutation");
                    exit(1);
                }
            }

        }
        else {
            eprintln!("Invalid scoreboard");
            exit(1);
        }
    }

    return (String::new(), None);
}

pub fn compile_into_if_statement(statements: Vec<Rc<dyn Object>>, code_block: Box<ASTOperation>, scope: &Scope, compiler: &Compiler) -> (String, Option<Scope>) {
    if statements.len() > 1 || statements.len() == 0 {
        eprintln!("More than 1 statement");
        exit(1);
    }
    let statement = statements[0].as_any().downcast_ref::<MinecraftStatementObject>();
    if statement.is_none() {
        eprintln!("Not MinecraftStatementObject");
        exit(1);
    }
    let statement = statement.unwrap();
    if let Statements::Execute(steps) = &statement.value {
        let mut execute_statements = String::from("");
        for step in steps {
            execute_statements.push_str(&format!("{}", execute_step_str(step.clone())));
        }


        // generate the code_block scope
        if let ASTOperation::CodeBlock(code) = *code_block {
            let mut codes: Vec<ASTOperation> = code.clone();

            if codes.len() == 0 {
                eprintln!("Empty code block.");
                exit(1);
            }
            if let ASTOperation::Set(mult) = code[0].clone() {
                codes.clear();
                for code in mult {
                    codes.push(code);
                }
            }
            let mut inline_scope = Scope::new(format!("{}.{}", scope.name, compiler.scopes.len()), compiler.namespace.clone(), codes);
            // add scoped variables
            inline_scope.variables = scope.variables.clone();

            let mut full_statement = String::new();
            for or_part in execute_statements.split("[OR]").into_iter() {
                let or_part = or_part.trim();
                if or_part.is_empty() { continue; }
                full_statement.push_str(&format!("execute {} run function {}:{}\n", or_part, compiler.namespace, inline_scope.name));
            }

            return (full_statement, Some(inline_scope));
        }

    }

    return (String::new(), None);
}
