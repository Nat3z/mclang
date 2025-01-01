use std::{any::Any, collections::HashMap, process::exit, rc::Rc};

use crate::{
    ast::operations::{ASTOperation, Operator},
    compile::{
        compiler::{Compiler, Scope, Variable},
        mcstatements::{execute_step_str, MinecraftStatementObject, Statements},
        objects::{match_objects, Object, Objects},
    },
};

use super::scoreboard::ScoreboardPlayerPairObject;

#[derive(Clone, Debug)]
pub struct VariableObject {
    pub value: Box<Objects>,
    pub scoreboard: Box<Objects>,
}

#[derive(Clone, Debug)]
pub struct CreatedFunctionObject {}

#[derive(Clone, Debug)]
pub struct MutationVariableObject {
    pub variable_obj: Box<Objects>,
    pub variable: ScoreboardPlayerPairObject,
    pub operator: Operator,
    pub mutation: ScoreboardPlayerPairObject,
    pub mutation_value: Box<Objects>,
}

#[derive(Clone, Debug)]
pub struct IfStatementObject {
    pub operations: Vec<Rc<dyn Object>>,
    pub code_block: Box<ASTOperation>,
}

#[derive(Clone, Debug)]
pub struct WhileObject {
    pub name: String,
    pub iterator: Vec<Rc<dyn Object>>,
    pub code_block: Box<ASTOperation>,
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

    fn get_functions(
        &self,
    ) -> HashMap<
        String,
        Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>,
    > {
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

    fn get_functions(
        &self,
    ) -> HashMap<
        String,
        Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>,
    > {
        HashMap::new()
    }
}

impl Object for CreatedFunctionObject {
    fn get_type(&self) -> Objects {
        Objects::CreatedFunction
    }
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        HashMap::new()
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

impl Object for WhileObject {
    fn get_type(&self) -> Objects {
        Objects::While(
            self.name.clone(),
            self.iterator.clone(),
            self.code_block.clone(),
        )
    }
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        HashMap::new()
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

impl Object for MutationVariableObject {
    fn get_type(&self) -> Objects {
        Objects::MutationVariable(
            Rc::new(self.variable.clone()),
            self.variable_obj.clone(),
            self.operator.clone(),
            Rc::new(self.mutation.clone()),
            self.mutation_value.clone(),
        )
    }
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        HashMap::new()
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

pub fn compile_into_variable(var: Objects, scoreboard: Objects) -> (String, Option<Scope>) {
    if let Objects::Number(num) = var {
        if let Objects::Scoreboard(name, objective, _) = scoreboard {
            let mut built_str = String::new();
            built_str.push_str(&format!(
                "scoreboard objective add {} {}\n",
                name, objective
            ));
            built_str.push_str(&format!("scoreboard players set value {} {}\n", name, num));
            return (built_str, None);
        } else {
            eprintln!("Invalid scoreboard");
            exit(1);
        }
    } else if let Objects::Boolean(bool) = var {
        if let Objects::Scoreboard(name, objective, _) = scoreboard {
            let mut built_str = String::new();
            built_str.push_str(&format!(
                "scoreboard objective add {} {}\n",
                name, objective
            ));
            built_str.push_str(&format!(
                "scoreboard players set value {} {}\n",
                objective,
                if bool { 1 } else { 0 }
            ));
            return (built_str, None);
        } else {
            eprintln!("Invalid scoreboard");
            exit(1);
        }
    } else if let Objects::Scoreboard(name, objective, _) = var {
        return (
            format!("scoreboard objective add {} {}\n", name, objective),
            None,
        );
    } else {
        // not a variable that should be compiled to text.
    }

    return (String::new(), None);
}

pub fn compile_into_mutation_variable(
    variable: &ScoreboardPlayerPairObject,
    variable_object: Objects,
    operation: Operator,
    mutation: Option<&ScoreboardPlayerPairObject>,
    mutation_object: Objects,
) -> (String, Option<Scope>) {
    println!("{:?}", variable);
    match mutation_object {
        Objects::Number(num) => {
            let mut built_str = String::new();
            if let Operator::Assignment = operation {
                built_str.push_str(&format!(
                    "scoreboard players set {} {} {}\n",
                    variable.player_name, variable.objective_name, num
                ));
            } else if let Operator::Add = operation {
                built_str.push_str(&format!(
                    "scoreboard players add {} {} {}\n",
                    variable.player_name, variable.objective_name, num
                ));
            } else if let Operator::Subtract = operation {
                built_str.push_str(&format!(
                    "scoreboard players remove {} {} {}\n",
                    variable.player_name, variable.objective_name, num
                ));
            } else if let Operator::Multiply = operation {
                built_str.push_str(&format!(
                    "scoreboard players operation {} {} *= {}\n",
                    variable.player_name, variable.objective_name, num
                ));
            } else if let Operator::Divide = operation {
                built_str.push_str(&format!(
                    "scoreboard players operation {} {} /= {}\n",
                    variable.player_name, variable.objective_name, num
                ));
            } else if let Operator::Modulus = operation {
                built_str.push_str(&format!(
                    "scoreboard players operation {} {} %= {}\n",
                    variable.player_name, variable.objective_name, num
                ));
            }
            return (built_str, None);
        }
        Objects::Variable(_, scoreboard_second) => {
            if let Objects::Scoreboard(_, _, _) = *scoreboard_second {
                if mutation.is_none() {
                    eprintln!("No mutation found");
                    exit(1);
                }

                let mutation = mutation.unwrap();
                let mut built_str = String::new();
                if let Operator::Assignment = operation {
                    built_str.push_str(&format!(
                        "scoreboard players operation {} {} = {} {}",
                        variable.player_name,
                        variable.objective_name,
                        mutation.player_name,
                        mutation.objective_name
                    ));
                } else if let Operator::Add = operation {
                    built_str.push_str(&format!(
                        "scoreboard players operation {} {} += {} {}",
                        variable.player_name,
                        variable.objective_name,
                        mutation.player_name,
                        mutation.objective_name
                    ));
                } else if let Operator::Subtract = operation {
                    built_str.push_str(&format!(
                        "scoreboard players operation {} {} -= {} {}",
                        variable.player_name,
                        variable.objective_name,
                        mutation.player_name,
                        mutation.objective_name
                    ));
                } else if let Operator::Multiply = operation {
                    built_str.push_str(&format!(
                        "scoreboard players operation {} {} *= {} {}",
                        variable.player_name,
                        variable.objective_name,
                        mutation.player_name,
                        mutation.objective_name
                    ));
                } else if let Operator::Divide = operation {
                    built_str.push_str(&format!(
                        "scoreboard players operation {} {} /= {} {}",
                        variable.player_name,
                        variable.objective_name,
                        mutation.player_name,
                        mutation.objective_name
                    ));
                } else if let Operator::Modulus = operation {
                    built_str.push_str(&format!(
                        "scoreboard players operation {} {} %= {} {}",
                        variable.player_name,
                        variable.objective_name,
                        mutation.player_name,
                        mutation.objective_name
                    ));
                }
                return (built_str, None);
            }
        }
        Objects::Boolean(bool) => {
            let mut built_str = String::new();
            built_str.push_str(&format!(
                "scoreboard players set {} {} {}\n",
                variable.player_name,
                variable.objective_name,
                if bool { 1 } else { 0 }
            ));
            return (built_str, None);
        }

        Objects::ScoreboardPlayerPair(new_player_name, new_objective, _) => {
            if mutation.is_none() {
                eprintln!("No mutation found");
                exit(1);
            }
            let mutation = mutation.unwrap();
            let mut built_str = String::new();

            if let Operator::Assignment = operation {
                built_str.push_str(&format!(
                    "scoreboard players operation {} {} = {} {}",
                    variable.player_name,
                    variable.objective_name,
                    mutation.player_name,
                    mutation.objective_name
                ));
            } else if let Operator::Add = operation {
                built_str.push_str(&format!(
                    "scoreboard players operation {} {} += {} {}",
                    variable.player_name,
                    variable.objective_name,
                    mutation.player_name,
                    mutation.objective_name
                ));
            } else if let Operator::Subtract = operation {
                built_str.push_str(&format!(
                    "scoreboard players operation {} {} -= {} {}",
                    variable.player_name,
                    variable.objective_name,
                    mutation.player_name,
                    mutation.objective_name
                ));
            } else if let Operator::Multiply = operation {
                built_str.push_str(&format!(
                    "scoreboard players operation {} {} *= {} {}",
                    variable.player_name,
                    variable.objective_name,
                    mutation.player_name,
                    mutation.objective_name
                ));
            } else if let Operator::Divide = operation {
                built_str.push_str(&format!(
                    "scoreboard players operation {} {} /= {} {}",
                    variable.player_name,
                    variable.objective_name,
                    mutation.player_name,
                    mutation.objective_name
                ));
            } else if let Operator::Modulus = operation {
                built_str.push_str(&format!(
                    "scoreboard players operation {} {} %= {} {}",
                    variable.player_name,
                    variable.objective_name,
                    mutation.player_name,
                    mutation.objective_name
                ));
            }

            return (built_str, None);
        }
        _ => {
            eprintln!("Invalid mutation {:?}", mutation_object);
            exit(1);
        }
    }
    return (String::new(), None);
}
pub fn compile_into_while_loop(
    name: String,
    set: Vec<Rc<dyn Object>>,
    code_block: Box<ASTOperation>,
    scope: &mut Scope,
    compiler: &mut Compiler,
) -> String {
    let mut built_str = String::new();
    if let ASTOperation::CodeBlock(operations) = *code_block {
        let mut codes: Vec<ASTOperation> = operations.clone();
        if codes.len() == 0 {
            eprintln!("Empty code block.");
            exit(1);
        }
        if let ASTOperation::Set(mult) = operations[0].clone() {
            codes.clear();
            for code in mult {
                codes.push(code);
            }
        }

        if let Objects::Array(set) = set[0].get_type() {
            for item in set {
                let mut inline_scope = Scope::new(
                    format!("{}.{}", scope.name, scope.scopes.len()),
                    compiler.namespace.clone(),
                    codes.clone(),
                    scope.functions.clone(),
                );
                // add scoped variables
                inline_scope.variables = scope.variables.clone();
                println!("{:?}", item);
                let variable = Objects::Variable(
                    Box::new(item.get_type()),
                    Box::new(Objects::Scoreboard(
                        format!("v_{}_{}", inline_scope.name, inline_scope.variables.len()),
                        "dummy".to_string(),
                        Box::new(item.get_type()),
                    )),
                );
                let variable = match_objects(variable);
                inline_scope.variables.insert(
                    name.clone(),
                    Variable {
                        name: name.clone(),
                        value: variable.clone(),
                        static_variable: false,
                    },
                );
                compiler.compile(&mut inline_scope);
                scope.scopes.push(inline_scope.clone());
                built_str.push_str(&format!(
                    "\nfunction {}:{}",
                    compiler.namespace, inline_scope.name
                ));
            }
        } else {
            eprintln!("Not an array");
            exit(1);
        }
    }

    return built_str;
}

pub fn compile_into_if_statement(
    statements: Vec<Rc<dyn Object>>,
    code_block: Box<ASTOperation>,
    scope: &Scope,
    compiler: &Compiler,
) -> (String, Option<Scope>) {
    if statements.len() > 1 || statements.len() == 0 {
        eprintln!("More than 1 statement");
        exit(1);
    }
    let statement = statements[0]
        .as_any()
        .downcast_ref::<MinecraftStatementObject>();
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
            let mut inline_scope = Scope::new(
                format!("{}.{}", scope.name, scope.scopes.len()),
                compiler.namespace.clone(),
                codes,
                scope.functions.clone(),
            );
            // add scoped variables
            inline_scope.variables = scope.variables.clone();

            let mut full_statement = String::new();
            for or_part in execute_statements.split("[OR]").into_iter() {
                let or_part = or_part.trim();
                if or_part.is_empty() {
                    continue;
                }
                full_statement.push_str(&format!(
                    "execute {} run function {}:{}\n",
                    or_part, compiler.namespace, inline_scope.name
                ));
            }

            return (full_statement, Some(inline_scope));
        }
    }

    return (String::new(), None);
}
