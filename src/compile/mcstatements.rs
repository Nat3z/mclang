use std::{any::Any, collections::HashMap, process::exit, rc::Rc};

use crate::ast::operations::Operator;

use super::{compiler::Scope, obj::std::VariableObject, objects::{mk_variable, Object, Objects}};

#[derive(Clone, Debug)]
pub enum Statements {
    Execute(Vec<ExecuteSteps>),
    Raw(String),
}
#[derive(Clone, Debug)]
pub enum ExecuteSteps {
    As(Objects),
    At(Objects),
    In(Objects),
    Compare(Objects, Operator, Objects)
}

#[derive(Clone, Debug)]
pub struct MinecraftStatementObject {
    pub value: Statements
}

impl Object for MinecraftStatementObject {
    fn get_type(&self) -> Objects {
        Objects::MCStatement(self.value.clone())
    }
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        let mut map = HashMap::new();
        map.insert("value".to_string(), 
            mk_variable(Objects::MCStatement(self.value.clone()), Objects::Unknown)
        );
        return map;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_functions(&self) -> HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>> {
        HashMap::new()
    }
}

pub fn execute_step_str(step: ExecuteSteps) -> String {
    match step {
        ExecuteSteps::As(entity) => {
            if let Objects::Entity(selector) = entity {
                return format!("as {}", selector)
            }
            else {
                eprintln!("Incorrect argument");
                exit(1);
            }
        },
        ExecuteSteps::At(entity) => {
            if let Objects::Entity(selector) = entity {
                return format!("at {}", selector)
            }
            else {
                eprintln!("Incorrect argument");
                exit(1);
            }
        },
        ExecuteSteps::Compare(first, operand, second) => {
            if let Objects::MCStatement(statement_first) = &first {
                let mut parts: Vec<String> = vec![];
                if let Statements::Execute(steps) = statement_first {
                    parts.push(execute_step_str(steps[0].clone()));

                }
                else {
                    eprintln!("Invalid value");
                    exit(1);
                }

                if let Objects::MCStatement(statement_second) = second {
                    if let Statements::Execute(steps) = statement_second {
                        parts.push(execute_step_str(steps[0].clone()));
                        match operand {
                            Operator::And => {
                                let mut full_str = String::new();
                                for part in parts {
                                    full_str.push_str(" ");
                                    full_str.push_str(&part);
                                }
                                return full_str.trim().to_string();
                            },
                            Operator::Or => {
                                let mut full_str = String::new();
                                for part in parts {
                                    full_str.push_str("[OR]");
                                    full_str.push_str(&part);
                                }
                                return full_str.trim().to_string()
                            },
                            _ => {
                                return String::new()
                            }
                        }

                    }
                    else {
                        eprintln!("Invalid value");
                        exit(1);
                    }
                }
            }
            if let Objects::Variable(value, scoreboard) = first {

                if let Objects::Scoreboard(scoreboard_first_name, objective_first) = *scoreboard {
                    if let Objects::Variable(value_second, scoreboard_second) = second {

                        if let Objects::Scoreboard(scoreboard_second_name, objective_second) = *scoreboard_second {
                            match *value_second {
                                Objects::Number(_) | Objects::Boolean(_) => {
                                    let operand_equiv = if operand == Operator::Equal {
                                        "="
                                    } else if operand == Operator::GreaterThanEqual {
                                        ">="
                                    } else if operand == Operator::LessThanEqual {
                                        "<="
                                    } else if operand == Operator::LessThan {
                                        "<"
                                    } else if operand == Operator::GreaterThan {
                                        ">"
                                    } else { "" };
                                    return format!("if score value {} {} value {}", scoreboard_first_name, operand_equiv, scoreboard_second_name)
                                },
                                _ => {
                                    return "".to_string()
                                }
                            }
                        }
                        else {
                            eprintln!("Incorrect scoreboard type.");
                            exit(1);
                        }
                    }
                    else if let Objects::Number(num) = second {
                        let operand_equiv = if operand == Operator::Equal {
                            &format!("{}", num)
                        } else if operand == Operator::GreaterThan {
                            &format!("{}..", num + 1)
                        } else if operand == Operator::LessThan {
                            &format!("..{}", num + 1)
                        } else if operand == Operator::GreaterThanEqual {
                            &format!("{}..", num)
                        } else if operand == Operator::LessThanEqual {
                            &format!("..{}", num)
                        }
                        else { "" };
                        return format!("if score value {} matches {}", scoreboard_first_name, operand_equiv)
                    }
                    else if let Objects::Boolean(bool) = second {
                        let operand_equiv = if operand == Operator::Equal {
                            &format!("{}", if bool { "1" } else { "0" })
                        }
                        else { "" };
                        return format!("if score value {} matches {}", scoreboard_first_name, operand_equiv)
                    }
                    else {
                        eprintln!("Not Variable");
                        exit(1);
                    }
                }
                else {
                    eprintln!("Incorrect scoreboard type.");
                    exit(1);
                }
            }
            else {
                println!("ACTUAL: {:?}", first);
                eprintln!("Make a variable the first thing.");
                exit(1);
            }
        },
        _ => {
            return "".to_string();
        }
    }
}

pub fn compile_into_mcstatement(statement: Statements) -> (String, Option<Scope>) {
    match statement {
        Statements::Execute(steps) => {
            let mut built_str = String::new();
            for step in steps {
                built_str.push_str(&execute_step_str(step));
                built_str.push_str(" ");
            }
            return (built_str, None);
        },
        Statements::Raw(raw) => {
            return (raw, None);
        }
    }
}
