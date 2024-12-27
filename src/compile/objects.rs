use std::{any::Any, collections::HashMap, fmt::Debug, rc::Rc};

use crate::ast::operations::{ASTOperation, Operator};

use super::{mcstatements::{MinecraftStatementObject, Statements}, obj::{basic::{BooleanObject, NullObject, NumberObject, SetObject, StringObject}, blockpos::BlockPosObject, entity::EntityObject, std::{IfStatementObject, MutationVariableObject, VariableObject}}};

#[derive(Clone, Debug)]
pub enum Objects {
    Entity(String),
    Dimension(String),
    BlockPos(i64, i64, i64),
    String(String),
    Number(i64),
    Boolean(bool),
    MCStatement(Statements),
    Scoreboard(String, String),
    Variable(Box<Objects>, Box<Objects>),
    MutationVariable(Box<Objects>, Operator, Box<Objects>),
    IfStatement(Vec<Rc<dyn Object>>, Box<ASTOperation>),
    Array(Vec<Rc<dyn Object>>),
    Unknown,
}

pub trait Object: Debug {
    fn get_type(&self) -> Objects;
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>>;
    fn get_functions(&self) -> HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>>;
    fn as_any(&self) -> &dyn Any;
}

pub fn name_into_object(str: &str) -> Rc<dyn Object> {
    match str {
        "Entity" => match_objects(Objects::Entity("".to_string())),
        "Dimension" => match_objects(Objects::Dimension("".to_string())),
        "BlockPos" => match_objects(Objects::BlockPos(0, 0, 0)),
        // "string" => Objects::String("".to_string()),
        // "number" => Objects::Number(0),
        // "boolean" => Objects::Boolean(false),
        // "mcstatement" => Objects::MCStatement(Statements::Raw("".to_string())),
        "Scoreboard" => match_objects(Objects::Scoreboard("".to_string(), "".to_string())),
        // "variable" => Objects::Variable(Box::new(Objects::Unknown), Box::new(Objects::Unknown)),
        // "mutation_variable" => Objects::MutationVariable(Box::new(Objects::Unknown), Operator::Add, Box::new(Objects::Unknown)),
        // "if_statement" => Objects::IfStatement(vec![], Box::new(ASTOperation::Access("".to_string()))),
        _ => match_objects(Objects::Unknown)
    }
}
pub fn match_objects(obj: Objects) -> Rc<dyn Object> {
    match obj {
        Objects::Number(num) => Rc::new(NumberObject { value: num }),
        Objects::String(str) => Rc::new(StringObject { value: str }),
        Objects::Boolean(bool) => Rc::new(BooleanObject { value: bool }),
        Objects::Unknown => Rc::new(NullObject {}),
        Objects::MCStatement(statement) => Rc::new(MinecraftStatementObject { value: statement }),
        Objects::Variable(var, scoreboard) => Rc::new(VariableObject { value: var, scoreboard }),
        Objects::MutationVariable(variable, operator, new) => Rc::new(MutationVariableObject { variable, new_value: new, operator }),
        Objects::IfStatement(boolean_statements, code_block) => Rc::new(IfStatementObject { code_block, operations: boolean_statements }),
        Objects::Entity(selector) => Rc::new(EntityObject { selector }),
        Objects::BlockPos(x, y, z) => Rc::new(BlockPosObject { x, y, z }),
        Objects::Array(values) => Rc::new(SetObject { values }),
        _ => Rc::new(NullObject {})
    }
}

pub fn mk_variable(obj: Objects, scoreboard: Objects) -> Rc<VariableObject> {
    Rc::new(VariableObject {
        value: Box::new(obj),
        scoreboard: Box::new(scoreboard)
    })
}


pub fn mk_function_map() -> HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>> {
    HashMap::new()
}
