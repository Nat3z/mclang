use std::{collections::HashMap, mem::discriminant, process::exit, rc::Rc};

use crate::ast::operations::{ASTOperation, Operator};

use super::{mcstatements::{execute_step_str, ExecuteSteps, Statements}, objects::{match_objects, MinecraftStatementObject, NumberObject, Object, Objects, StringObject}};

pub struct Compiler {
    pub scopes: Vec<Scope>,
    pub namespace: String,
    pub outputs: HashMap<String, String>,
}

#[derive(Clone)]
pub struct Scope {
    variables: HashMap<String, Variable>,
    statements: Vec<ASTOperation>,
    namespace: String,
    name: String
}
#[derive(Clone)]
pub struct Variable {
    name: String,
    value: Rc<dyn Object>,
}
impl Compiler {
    pub fn new(statements: Vec<ASTOperation>, namespace: &'static str) -> Compiler {
        Compiler {
            scopes: vec![
                Scope::new("global".to_string(), namespace.to_string(), statements)
            ],
            namespace: namespace.to_string(),
            outputs: HashMap::new()
        }
    }

    pub fn compile_into(&self, scope: &Scope, value: Rc<dyn Object>) -> (String, Option<Scope>) {
        match value.get_type() {
            Objects::MCStatement(statement) => {
                let mut built_str = String::new();
                match statement {
                    Statements::Execute(execute_steps) => {
                        for execute_steps in execute_steps {
                            if let ExecuteSteps::Compare(left, operator, right) = execute_steps {
                                if let Objects::Number(num) = left {
                                    println!("{}", num);
                                }
                            }
                        }
                    }
                }

            return (built_str, None);
            },

            Objects::Variable(var, scoreboard) => {
                if let Objects::Number(num) = *var {
                    if let Objects::Scoreboard(name, objective) = *scoreboard {
                        let mut built_str = String::new();
                        built_str.push_str(&format!("scoreboard objective add {} {}\n", name, objective));
                        built_str.push_str(&format!("scoreboard players set value {} {}\n", objective, num));
                        return (built_str, None);
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
                        return (built_str, None);
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
            Objects::MutationVariable(variable, operation, mutation) => {
                if let Objects::Variable(variable, scoreboard) = *variable  {
                    if let Objects::Scoreboard(name, _) = *scoreboard {
                        match *mutation {
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
            },

            Objects::IfStatement(statements, code_block) => {
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
                        let mut inline_scope = Scope::new(format!("{}.{}", scope.name, self.scopes.len()), self.namespace.clone(), codes);
                        // add scoped variables
                        inline_scope.variables = scope.variables.clone();

                        let mut full_statement = String::new();
                        for or_part in execute_statements.split("[OR]").into_iter() {
                            let or_part = or_part.trim();
                            if or_part.is_empty() { continue; }
                            full_statement.push_str(&format!("execute {} run function {}:{}\n", or_part, self.namespace, inline_scope.name));
                        }

                        return (full_statement, Some(inline_scope));
                    }

                }
            },
            _ => {
            }
        }
        return (String::new(), None);
    }

    pub fn compile(&mut self, current_scope: &mut Scope) {
        let mut index = 0;

        println!("-----------------");

        let mut output_str = String::new();
        while current_scope.statements.len() > index {
            let current_statement = current_scope.statements[index].clone();
            let value = current_scope.execute(&current_statement, None);
            let (compiled_value, mut new_scope) = self.compile_into(&current_scope, value);
            output_str.push_str(&compiled_value);

            index += 1;
            if new_scope.is_some() {
                self.scopes.push(new_scope.clone().unwrap());
                self.compile(new_scope.as_mut().unwrap());
            }
        }

        self.outputs.insert(current_scope.name.clone(), output_str);
    }
    
    pub fn flush(&self) -> &HashMap<String, String> {
        &self.outputs
    }
}

impl Scope {
    pub fn new(name: String, namespace: String, statements: Vec<ASTOperation>) -> Scope {
        Scope {
            variables: HashMap::new(),
            statements,
            namespace,
            name
        }
    }

    pub fn execute(&mut self, instruction: &ASTOperation, current_variable: Option<Variable>) -> Rc<dyn Object> {
        match instruction {
            ASTOperation::LiteralString(str) => {
                return match_objects(Objects::String(str.clone()));
            },
            ASTOperation::LiteralNumber(num) => {
                return match_objects(Objects::Number(
                    *num
                ));
            },
            ASTOperation::LiteralBool(bool) => {
                return match_objects(Objects::Boolean(
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
                let variable = match_objects(variable);
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
                let variable = self.variables.get(name).unwrap().clone();
                if let Objects::Variable(variable_value, scoreboard) = variable.value.get_type() {
                    let discrim_variable_value = discriminant(&*variable_value);
                    let value_box = Box::new(value.get_type());
                    if discrim_variable_value != discriminant(&value.get_type()) && discriminant(&value.get_type()) != discriminant(&Objects::MutationVariable(Box::new(Objects::Unknown), Operator::Add, Box::new(Objects::Unknown))) {
                        eprintln!("Invalid mutation: Type mismatch");
                        exit(1);
                    }
                    let new_variable = Variable {
                        name: name.clone(),
                        value: match_objects(Objects::Variable(value_box.clone(), scoreboard))
                    };
                    if let Objects::MutationVariable(old, operand, new) = value.get_type() {
                        let old_discrim = discriminant(&*variable_value);
                        let new_discrim = discriminant(&*new);
                        if old_discrim != new_discrim {
                            eprintln!("Invalid mutation: Type mismatch");
                            exit(1);
                        }
                        // self.variables.insert(name.clone(), new_variable.clone());
                        return match_objects(Objects::MutationVariable(
                            Box::new(variable.clone().value.get_type()), operand.clone(), new.clone())
                        );
                    }

                    // self.variables.insert(name.clone(), new_variable.clone());
                    return match_objects(Objects::MutationVariable(
                        Box::new(new_variable.value.get_type()), Operator::Assignment, value_box.clone())
                    );
                }
                else {
                    eprintln!("Invalid variable (Variable)");
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
                let variables = variable.value.get_variables();
                let variable = variables.get(name).expect("Unknown variable");
                println!("{:?}", variable);
                return variable.clone();
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
                println!("Value: {:?}", first_value);
                let second_value = self.execute(&second_statement, current_variable.clone());
                println!("Value: {:?}", second_value);
                if *operator == Operator::Add {
                    return match_objects(Objects::MutationVariable(
                        Box::new(first_value.get_type()),
                        operator.clone(), 
                        Box::new(second_value.get_type())
                    ))
                } else {
                    return match_objects(
                        Objects::MCStatement(
                            Statements::Execute(vec![
                                ExecuteSteps::Compare(first_value.get_type().clone(), operator.clone(), second_value.get_type().clone())
                            ])
                        )
                    )
                } 
            },
            ASTOperation::If(operations, codeblock) => {
                let mut values: Vec<Rc<dyn Object>> = vec![];
                for operation in operations {
                    println!("Executing: {:?}", operation);
                    values.push(self.execute(&operation, current_variable.clone()));
                }

                return match_objects(Objects::IfStatement(values, codeblock.clone()));
            },
            _ => {
                return match_objects(Objects::Unknown); 
            }
        }
    }
}
