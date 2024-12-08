use std::{collections::HashMap, mem::discriminant, process::exit, rc::Rc};

use crate::ast::operations::ASTOperation;

use super::{mcstatements::{ExecuteSteps, Statements}, objects::{match_objects_with_struct, MinecraftStatementObject, NumberObject, Object, Objects, StringObject}};

pub struct Compiler {
    statements: Vec<ASTOperation>,
    index: usize,
    scopes: Vec<Scope>,
    compile_str: String,
}

#[derive(Clone)]
pub struct Scope {
    variables: HashMap<String, Variable>,
    name: String
}
#[derive(Clone)]
pub struct Variable {
    name: String,
    value: Rc<dyn Object>,
}
impl Compiler {
    pub fn new(statements: Vec<ASTOperation>) -> Compiler {
        Compiler {
            statements,
            index: 0,
            scopes: vec![
                Scope::new("global".to_string())
            ],
            compile_str: String::new()
        }
    }

    pub fn compile(&mut self) {
        let mut current_scope = self.scopes[0].clone();
        while self.statements.len() > self.index {
            let current_statement = self.statements[self.index].clone();
            let value = current_scope.execute(&current_statement, None);
            match value.get_type() {
                Objects::MCStatement(statement) => {
                    let mut built_str = String::new();
                    match statement {
                        Statements::Execute(execute_steps) => {
                            for execute_steps in execute_steps {
                                if let ExecuteSteps::Compare(left, operator, right) = execute_steps {
                                }
                            }
                        }
                    }
                },

                Objects::Variable(var, scoreboard) => {
                    if let Objects::Number(num) = *var {
                        if let Objects::Scoreboard(name, objective) = *scoreboard {
                            let mut built_str = String::new();
                            built_str.push_str(&format!("scoreboard objective add {} {}\n", name, objective));
                            built_str.push_str(&format!("scoreboard players set value {} {}\n", objective, num));
                            self.add_to_compile(built_str);
                        }
                        else {
                            eprintln!("Invalid scoreboard");
                            exit(1);
                        }
                    } else if let Objects::Boolean(bool) = *var {
                        if let Objects::Scoreboard(name, objective) = *scoreboard {
                            let mut built_str = String::new();
                            built_str.push_str(&format!("scoreboard objective add {} {}\n", name, objective));
                            built_str.push_str(&format!("scoreboard players set value {} {}\n", objective, if bool { 1 } else { 0 } ));
                            self.add_to_compile(built_str);
                        }
                        else {
                            eprintln!("Invalid scoreboard");
                            exit(1);
                        }
                    }
                    else {
                        eprintln!("Invalid variable");
                        exit(1);
                    }
                },
                Objects::MutationVariable(variable, mutation) => {
                    if let Objects::Variable(variable, scoreboard) = *variable  {
                        if let Objects::Scoreboard(name, _) = *scoreboard {
                            match *mutation {
                                Objects::Number(num) => {
                                    let mut built_str = String::new();
                                    built_str.push_str(&format!("scoreboard players set value {} {}\n", name, num));
                                    self.add_to_compile(built_str);
                                },
                                Objects::Boolean(bool) => {
                                    let mut built_str = String::new();
                                    built_str.push_str(&format!("scoreboard players set value {} {}\n", name, if bool { 1 } else { 0 } ));
                                    self.add_to_compile(built_str);
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
                },
                _ => {
                }
            }
            self.index += 1;
        }
    }
    
    pub fn add_to_compile(&mut self, str: String) {
        self.compile_str.push_str(&format!("\n{str}"));
    } 
    pub fn flush(&self) -> String {
        self.compile_str.clone()
    }
}

impl Scope {
    pub fn new(name: String) -> Scope {
        Scope {
            variables: HashMap::new(),
            name
        }
    }
    pub fn execute(&mut self, instruction: &ASTOperation, current_variable: Option<Variable>) -> Rc<dyn Object> {
        match instruction {
            ASTOperation::LiteralString(str) => {
                return match_objects_with_struct(Objects::String(str.clone()));
            },
            ASTOperation::LiteralNumber(num) => {
                return match_objects_with_struct(Objects::Number(
                    *num
                ));
            },
            ASTOperation::LiteralBool(bool) => {
                return match_objects_with_struct(Objects::Boolean(
                    *bool
                ));
            },
            ASTOperation::AssignVariable(name, operation) => {
                if operation.len() != 1 {
                    eprintln!("More than 1 operation in assign variable");
                    exit(1);
                }
                let value = self.execute(&operation[0], current_variable);

                let variable = Objects::Variable(
                    Box::new(value.clone().get_type()),
                    Box::new(Objects::Scoreboard(format!("v_{}_{}", self.name, self.variables.len()), "dummy".to_string()))
                );
                let variable = match_objects_with_struct(variable);
                self.variables.insert(name.clone(), Variable {
                    name: name.clone(),
                    value: variable.clone()
                });
                return variable;
            },
            ASTOperation::MutateVariable(name, operation) => {
                if operation.len() != 1 {
                    eprintln!("More than 1 operation in assign variable");
                    exit(1);
                }
                let value = self.execute(&operation[0], current_variable);
                if !self.variables.contains_key(name) {
                    eprintln!("Variable {} does not exist", name);
                    exit(1);
                }

                if let Objects::Variable(variable_value, scoreboard) = self.variables.get(name).unwrap().value.get_type() {
                    if discriminant(&*variable_value) != discriminant(&value.get_type()) {
                        eprintln!("Invalid mutation: Type mismatch");
                        exit(1);
                    }
                    let value_box = Box::new(value.get_type());
                    let new_variable = Variable {
                        name: name.clone(),
                        value: match_objects_with_struct(Objects::Variable(value_box.clone(), scoreboard))
                    };
                    self.variables.insert(name.clone(), new_variable.clone());
                    return match_objects_with_struct(Objects::MutationVariable(
                        Box::new(new_variable.value.get_type()), value_box.clone())
                    );
                }
                else {
                    eprintln!("Invalid variable");
                    exit(1);
                }

            },
            ASTOperation::Access(name) => {
                if current_variable.is_none() {
                    if !self.variables.contains_key(name) {
                        eprintln!("Variable {} does not exist", name);
                        exit(1);
                    }
                    return self.variables.get(name).unwrap().value.clone();
                }
                let variable = current_variable.unwrap();
                return variable.value.get_variables().get(name).expect("Unknown variable").clone();
            },
            ASTOperation::UseVariable(name, operation) => {
                let variable = self.variables.get(name).expect("Variable not found");
                let value = self.execute(&operation, Some(variable.clone()));
                return value;
            },
            // runs this inside of the variable
            ASTOperation::AccessPart(operation) => {
                let value = self.execute(&operation, current_variable);
                return value;
            },
            ASTOperation::Operation(first_statement, operator, second_statement) => {
                let first_value = self.execute(&first_statement, current_variable.clone());
                let second_value = self.execute(&second_statement, current_variable.clone());
                return match_objects_with_struct(
                    Objects::MCStatement(
                        Statements::Execute(vec![
                            ExecuteSteps::Compare(first_value.get_type().clone(), operator.clone(), second_value.get_type().clone())
                        ])
                    )
                )
            },
            _ => {
                return match_objects_with_struct(Objects::Unknown); 
            }
        }
    }
}
