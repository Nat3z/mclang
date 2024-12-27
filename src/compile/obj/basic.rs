use std::{any::Any, collections::HashMap, rc::Rc};

use crate::compile::objects::{match_objects, mk_function_map, mk_variable, Object, Objects};

use super::std::VariableObject;

#[derive(Clone, Debug)]
pub struct BooleanObject {
    pub value: bool
}
#[derive(Clone, Debug)]
pub struct StringObject {
    pub value: String
}

#[derive(Clone, Debug)]
pub struct NullObject {
}

#[derive(Clone, Debug)]
pub struct NumberObject {
    pub value: i64,
}

#[derive(Clone, Debug)]
pub struct SetObject {
    pub values: Vec<Rc<dyn Object>>,
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

    fn get_functions(&self) -> HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>> {
        HashMap::new()
    }
}

impl Object for SetObject {
    fn get_type(&self) -> Objects {
        Objects::Array(self.values.clone())
    }

    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        let mut map = HashMap::new();
        let mut index = 0;
        for value in self.values.clone() {
            map.insert(format!("_{}", index), mk_variable(value.get_type(), Objects::Unknown));
            println!("Made {}", index);
            index += 1;
        }
        return map;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_functions(&self) -> HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>> {
        let mut map = mk_function_map();
        map.insert("get".to_string(), Box::new(|params, variable| {
            let index = params[0].get_type().clone();
            let index = match index {
                Objects::Number(num) => num,
                _ => panic!("Index must be a number")
            };
            let value = variable.unwrap().value.clone();
            let value = match *value {
                Objects::Array(arr) => arr,
                _ => panic!("Value must be an array")
            };

            let value = value.get(0).unwrap().as_any().downcast_ref::<SetObject>();
            if value.is_none() {
                panic!("Value must be an array (must have at least 2 values)");
            }
            let value = value.unwrap();
            println!("{:?}", value.values[index as usize]);
            value.values[index as usize].clone()
        }));
        map
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

    fn get_functions(&self) -> HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>> {
        let map = mk_function_map();
        map
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
    fn get_functions(&self) -> HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>> {
        HashMap::new()
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

    fn get_functions(&self) -> HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>> {
        HashMap::new()
    }
}



