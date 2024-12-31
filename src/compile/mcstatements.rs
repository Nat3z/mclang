use std::{any::Any, collections::HashMap, process::exit, rc::Rc};

use crate::ast::operations::Operator;

use super::{
    compiler::Scope,
    obj::{scoreboard::ScoreboardPlayerPairObject, std::VariableObject},
    objects::{mk_variable, Object, Objects},
};

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
    Compare(Objects, Operator, Objects),
}

#[derive(Clone, Debug)]
pub struct MinecraftStatementObject {
    pub value: Statements,
}

impl Object for MinecraftStatementObject {
    fn get_type(&self) -> Objects {
        Objects::MCStatement(self.value.clone())
    }
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        let mut map = HashMap::new();
        map.insert(
            "value".to_string(),
            mk_variable(Objects::MCStatement(self.value.clone()), Objects::Unknown),
        );
        return map;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_functions(
        &self,
    ) -> HashMap<
        String,
        Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>,
    > {
        HashMap::new()
    }
}

pub fn execute_step_str(step: ExecuteSteps) -> String {
    match step {
        ExecuteSteps::As(entity) => {
            if let Objects::Entity(selector) = entity {
                return format!("as {}", selector);
            } else {
                eprintln!("Incorrect argument");
                exit(1);
            }
        }
        ExecuteSteps::At(entity) => {
            if let Objects::Entity(selector) = entity {
                return format!("at {}", selector);
            } else {
                eprintln!("Incorrect argument");
                exit(1);
            }
        }
        ExecuteSteps::Compare(first, operand, second) => {
            if let Objects::MCStatement(statement_first) = &first {
                let mut parts: Vec<String> = vec![];
                if let Statements::Execute(steps) = statement_first {
                    parts.push(execute_step_str(steps[0].clone()));
                } else {
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
                            }
                            Operator::Or => {
                                let mut full_str = String::new();
                                for part in parts {
                                    full_str.push_str("[OR]");
                                    full_str.push_str(&part);
                                }
                                return full_str.trim().to_string();
                            }
                            _ => return String::new(),
                        }
                    } else {
                        eprintln!("Invalid value");
                        exit(1);
                    }
                }
            }
            let mut first_scoreboard_pair: Option<ScoreboardPlayerPairObject> = None;

            if let Objects::Variable(value, scoreboard) = first.clone() {
                if let Objects::Scoreboard(scoreboard_first_name, _, first_objective_type) =
                    *scoreboard
                {
                    first_scoreboard_pair = match *value {
                        Objects::Number(_) | Objects::Boolean(_) => {
                            Some(ScoreboardPlayerPairObject {
                                objective_type: *first_objective_type.clone(),
                                objective_name: scoreboard_first_name.clone(),
                                player_name: "value".to_string(),
                            })
                        }
                        Objects::ScoreboardPlayerPair(
                            objective_name,
                            player_name,
                            objective_type,
                        ) => Some(ScoreboardPlayerPairObject {
                            objective_type: *objective_type.clone(),
                            objective_name: objective_name.clone(),
                            player_name: player_name.clone(),
                        }),
                        _ => None,
                    };
                }
            } else if let Objects::ScoreboardPlayerPair(
                objective_name,
                player_name,
                objective_type,
            ) = first.clone()
            {
                first_scoreboard_pair = Some(ScoreboardPlayerPairObject {
                    objective_type: *objective_type.clone(),
                    objective_name: objective_name.clone(),
                    player_name: player_name.clone(),
                });
            }

            let mut second_scoreboard_pair: Option<ScoreboardPlayerPairObject> = None;
            if let Objects::Variable(value_second, scoreboard_second) = second.clone() {
                if let Objects::Scoreboard(scoreboard_second_name, _, objective_type) =
                    *scoreboard_second.clone()
                {
                    println!("{:?} {:?}", value_second, scoreboard_second);
                    second_scoreboard_pair = match *value_second {
                        Objects::Number(_) | Objects::Boolean(_) => {
                            Some(ScoreboardPlayerPairObject {
                                objective_type: *objective_type.clone(),
                                objective_name: scoreboard_second_name.clone(),
                                player_name: "value".to_string(),
                            })
                        }
                        Objects::ScoreboardPlayerPair(
                            objective_name,
                            player_name,
                            objective_type,
                        ) => Some(ScoreboardPlayerPairObject {
                            objective_type: *objective_type.clone(),
                            objective_name: objective_name.clone(),
                            player_name: player_name.clone(),
                        }),
                        _ => None,
                    };
                }
            } else if let Objects::ScoreboardPlayerPair(
                objective_name,
                player_name,
                objective_type,
            ) = second.clone()
            {
                second_scoreboard_pair = Some(ScoreboardPlayerPairObject {
                    objective_type: *objective_type.clone(),
                    objective_name: objective_name.clone(),
                    player_name: player_name.clone(),
                });
            } else if let Objects::Number(num) = second.clone() {
                second_scoreboard_pair = Some(ScoreboardPlayerPairObject {
                    objective_type: Objects::Number(num),
                    objective_name: "value".to_string(),
                    player_name: "".to_string(),
                });
            } else if let Objects::Boolean(bool) = second.clone() {
                second_scoreboard_pair = Some(ScoreboardPlayerPairObject {
                    objective_type: Objects::Boolean(bool),
                    objective_name: "value".to_string(),
                    player_name: "".to_string(),
                });
            }

            if let Some(first_scoreboard_pair) = first_scoreboard_pair {
                println!("FIRST: {:?}", first_scoreboard_pair);
                if let Some(second_scoreboard_pair) = second_scoreboard_pair {
                    println!("SECOND: {:?}", second_scoreboard_pair);
                    match second {
                        Objects::Number(num) => {
                            let operand_equiv = if operand == Operator::Equal {
                                &format!("{}", num)
                            } else if operand == Operator::GreaterThan {
                                &format!("{}..", num + 1)
                            } else if operand == Operator::LessThan {
                                &format!("..{}", num - 1)
                            } else if operand == Operator::GreaterThanEqual {
                                &format!("{}..", num)
                            } else if operand == Operator::LessThanEqual {
                                &format!("..{}", num)
                            } else {
                                ""
                            };
                            return format!(
                                "if score {} {} matches {}",
                                first_scoreboard_pair.player_name,
                                first_scoreboard_pair.objective_name,
                                operand_equiv
                            );
                        }
                        _ => {
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
                            } else {
                                ""
                            };
                            return format!(
                                "if score {} {} {} {} {}",
                                first_scoreboard_pair.player_name,
                                first_scoreboard_pair.objective_name,
                                operand_equiv,
                                second_scoreboard_pair.player_name,
                                second_scoreboard_pair.objective_name
                            );
                        }
                    }
                }
            }

            return String::new();
        }
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
        }
        Statements::Raw(raw) => {
            return (raw, None);
        }
    }
}
