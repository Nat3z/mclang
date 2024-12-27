use std::{any::Any, collections::HashMap, process::exit, rc::Rc};

use crate::compile::{mcstatements::{MinecraftStatementObject, Statements}, objects::{match_objects, Object, Objects}};

use super::{basic::StringObject, blockpos::BlockPosObject, std::VariableObject};

#[derive(Debug, Clone)]
pub struct EntityObject {
    pub selector: String
}

impl Object for EntityObject {
    
    fn get_type(&self) -> Objects {
        Objects::Entity(self.selector.clone())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        HashMap::new() 
    }
    fn get_functions(&self) -> HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>> {
        let mut map: HashMap<String, Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>> = HashMap::new();

        map.insert("instantiate".to_string(), Box::new(|params, _| {
            let selector = params[0].as_any().downcast_ref::<StringObject>();
            if selector.is_none() {
                eprintln!("Incorrect Type");
                exit(1);
            }
            Rc::new(EntityObject {
                selector: selector.unwrap().value.clone()
            })
        }));


        map.insert("kill".to_string(), Box::new(|params, variable| {
            if params.len() != 0 {
                eprintln!("Incorrect number of arguments for function kill");
                exit(1);
            }

            let own = match_objects(*variable.unwrap().value.clone());
            let own = own.as_any().downcast_ref::<EntityObject>().unwrap();

            Rc::new(MinecraftStatementObject {
                value: Statements::Raw(format!("kill {}", own.selector))
            })
        }));

        map.insert("tp".to_string(), Box::new(|params, variable| {
            if params.len() != 1 {
                eprintln!("Incorrect number of arguments for function tp");
                exit(1);
            }
            let params = params[0].as_any();

            let own = match_objects(*variable.unwrap().value.clone());
            let own = own.as_any().downcast_ref::<EntityObject>().unwrap();

            if let Some(params) = params.downcast_ref::<EntityObject>() {
                Rc::new(MinecraftStatementObject {
                    value: Statements::Raw(format!("tp {} {}", own.selector, params.selector))
                })
            }
            else if let Some(params) = params.downcast_ref::<BlockPosObject>() {
                Rc::new(MinecraftStatementObject {
                    value: Statements::Raw(format!("tp {} {} {} {}", own.selector, params.x, params.y, params.z))
                })
            }
            else {
                eprintln!("Incorrect argument type for function tp");
                exit(1);
            }
        }));

        return map;
    }
}
