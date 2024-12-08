use std::{any::Any, fmt::Debug, collections::HashMap, rc::Rc};

use crate::ast::operations::{ASTOperation, Operator};

use super::mcstatements::Statements;

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
    Unknown,
}

pub trait Object: Debug {
    fn get_type(&self) -> Objects;
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>>;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Debug)]
pub struct NumberObject {
    pub value: i64,
}

#[derive(Clone, Debug)]
pub struct ScoreboardObject {
    pub name: String,
    pub objective: String
}

#[derive(Clone, Debug)]
pub struct BooleanObject {
    pub value: bool
}
#[derive(Clone, Debug)]
pub struct StringObject {
    pub value: String
}
#[derive(Clone, Debug)]
pub struct MinecraftStatementObject {
    pub value: Statements
}

#[derive(Clone, Debug)]
pub struct NullObject {
}

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
}

impl Object for NumberObject {
    fn get_type(&self) -> Objects {
        Objects::Number(self.value)
    }
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        let mut map = HashMap::new();
        map.insert("value".to_string(), mk_variable(self.get_type(), Objects::Unknown));
        return map;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Object for ScoreboardObject {
    fn get_type(&self) -> Objects {
        Objects::Scoreboard(self.name.clone(), self.objective.clone())
    }
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        let mut map = HashMap::new();
        return map;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


impl Object for StringObject {
    fn get_type(&self) -> Objects {
        Objects::String(self.value.clone())
    }

    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        let mut map = HashMap::new();
        map.insert("value".to_string(), mk_variable(Objects::String(self.value.clone()), Objects::Unknown));
        return map;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Object for BooleanObject {
    fn get_type(&self) -> Objects {
        Objects::Boolean(self.value.clone())
    }

    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        let mut map = HashMap::new();
        map.insert("value".to_string(), mk_variable(Objects::Boolean(self.value.clone()), Objects::Unknown));
        return map;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Object for NullObject {
    fn get_type(&self) -> Objects {
        Objects::Unknown
    }
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        HashMap::new()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
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
        _ => Rc::new(NullObject {})
    }
}

pub fn mk_variable(obj: Objects, scoreboard: Objects) -> Rc<VariableObject> {
    Rc::new(VariableObject {
        value: Box::new(obj),
        scoreboard: Box::new(scoreboard)
    })
}
