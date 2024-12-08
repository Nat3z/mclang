use std::{any::Any, collections::HashMap, rc::Rc};

use super::mcstatements::Statements;

#[derive(Debug, Clone)]
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
    MutationVariable(Box<Objects>, Box<Objects>),
    Unknown,
}

pub trait Object {
    fn get_type(&self) -> Objects;
    fn get_variables(&self) -> HashMap<String, Rc<dyn Object>>;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone)]
pub struct NumberObject {
    pub value: i64,
}

#[derive(Clone, Debug)]
pub struct ScoreboardObject {
    pub name: String,
    pub objective: String
}

#[derive(Clone)]
pub struct BooleanObject {
    pub value: bool
}
#[derive(Clone)]
pub struct StringObject {
    pub value: String
}
#[derive(Clone)]
pub struct MinecraftStatementObject {
    value: Statements
}

#[derive(Clone)]
pub struct NullObject {
}

#[derive(Clone)]
pub struct VariableObject {
    pub value: Box<Objects>,
    pub scoreboard: Box<Objects>
}

#[derive(Clone)]
pub struct MutationVariableObject {
    pub variable: Box<Objects>,
    pub new_value: Box<Objects>
}

impl Object for VariableObject {
    fn get_type(&self) -> Objects {
        Objects::Variable(self.value.clone(), self.scoreboard.clone())
    }
    fn get_variables(&self) -> HashMap<String, Rc<dyn Object>> {
        HashMap::new()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Object for MutationVariableObject {
    fn get_type(&self) -> Objects {
        Objects::MutationVariable(self.variable.clone(), self.new_value.clone())
    }
    fn get_variables(&self) -> HashMap<String, Rc<dyn Object>> {
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
    fn get_variables(&self) -> HashMap<String, Rc<dyn Object>> {
        let mut map = HashMap::new();
        map.insert("value".to_string(), match_objects_with_struct(self.get_type()));
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
    fn get_variables(&self) -> HashMap<String, Rc<dyn Object>> {
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

    fn get_variables(&self) -> HashMap<String, Rc<dyn Object>> {
        let mut map = HashMap::new();
        map.insert("value".to_string(), match_objects_with_struct(Objects::String(self.value.clone())));
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

    fn get_variables(&self) -> HashMap<String, Rc<dyn Object>> {
        let mut map = HashMap::new();
        map.insert("value".to_string(), match_objects_with_struct(Objects::Boolean(self.value.clone())));
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
    fn get_variables(&self) -> HashMap<String, Rc<dyn Object>> {
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
    fn get_variables(&self) -> HashMap<String, Rc<dyn Object>> {
        let mut map = HashMap::new();
        map.insert("value".to_string(), match_objects_with_struct(Objects::MCStatement(self.value.clone())));
        return map;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn match_objects_with_struct(obj: Objects) -> Rc<dyn Object> {
    match obj {
        Objects::Number(num) => Rc::new(NumberObject { value: num }),
        Objects::String(str) => Rc::new(StringObject { value: str }),
        Objects::Boolean(bool) => Rc::new(BooleanObject { value: bool }),
        Objects::Unknown => Rc::new(NullObject {}),
        Objects::MCStatement(statement) => Rc::new(MinecraftStatementObject { value: statement }),
        Objects::Variable(var, scoreboard) => Rc::new(VariableObject { value: var, scoreboard }),
        Objects::MutationVariable(variable, new) => Rc::new(MutationVariableObject { variable, new_value: new }),
        _ => Rc::new(NullObject {})
    }
}
