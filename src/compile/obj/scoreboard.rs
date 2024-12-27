use std::{any::Any, collections::HashMap, rc::Rc};

use crate::compile::objects::{Object, Objects};

use super::std::VariableObject;

#[derive(Clone, Debug)]
pub struct ScoreboardObject {
    pub name: String,
    pub objective: String
}


impl Object for ScoreboardObject {
    fn get_type(&self) -> Objects {
        Objects::Scoreboard(self.name.clone(), self.objective.clone())
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


