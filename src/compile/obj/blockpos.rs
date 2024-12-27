use std::{any::Any, collections::HashMap, process::exit, rc::Rc};

use crate::compile::objects::{match_objects, mk_variable, Object, Objects};

use super::{basic::NumberObject, std::VariableObject};

#[derive(Clone, Debug)]
pub struct BlockPosObject {
    pub x: i64,
    pub y: i64,
    pub z: i64
}

impl Object for BlockPosObject {
    fn get_type(&self) -> Objects {
        Objects::BlockPos(self.x, self.y, self.z)
    }
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        let mut map = HashMap::new();
        map.insert("x".to_string(), mk_variable(Objects::Number(self.x), Objects::Unknown));
        map.insert("y".to_string(), mk_variable(Objects::Number(self.y), Objects::Unknown));
        map.insert("z".to_string(), mk_variable(Objects::Number(self.z), Objects::Unknown));
        return map;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_functions(&self) -> HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>> {
        let mut map: HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>> = HashMap::new();

        map.insert("instantiate".to_string(), Box::new(|params, _| {
            if params.len() != 3 {
                eprintln!("Incorrect number of params.");
                exit(1);
            }
            let x = params[0].as_any().downcast_ref::<NumberObject>();
            let y = params[1].as_any().downcast_ref::<NumberObject>();
            let z = params[2].as_any().downcast_ref::<NumberObject>();

            if x.is_none() || y.is_none() || z.is_none() {
                eprintln!("Incorrect type of params.");
                exit(1);
            }

            return match_objects(Objects::BlockPos(x.unwrap().value, y.unwrap().value, z.unwrap().value));
        }));
        return map;
    }
}
