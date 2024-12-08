use std::{mem::discriminant, process::exit, rc::Rc};

use crate::ast::operations::Operator;

use super::objects::{match_objects, Object, Objects};

#[derive(Clone, Debug)]
pub enum Statements {
    Execute(Vec<ExecuteSteps>),
}
#[derive(Clone, Debug)]
pub enum ExecuteSteps {
    As(Objects),
    At(Objects),
    In(Objects),
    Compare(Objects, Operator, Objects)
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


