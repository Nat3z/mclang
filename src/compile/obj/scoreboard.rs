use std::{any::Any, collections::HashMap, rc::Rc};

use crate::{
    ast::operations::Operator,
    compile::objects::{match_objects, mk_function_map, mk_variable, Object, Objects},
};

use super::std::VariableObject;

#[derive(Clone, Debug)]
pub struct ScoreboardObject {
    pub name: String,
    pub objective_type: Objects,
    pub objective: String,
}

#[derive(Clone, Debug)]
pub struct ScoreboardPlayerPairObject {
    pub objective_name: String,
    pub player_name: String,
    pub objective_type: Objects,
}

impl Object for ScoreboardPlayerPairObject {
    fn get_type(&self) -> Objects {
        Objects::ScoreboardPlayerPair(
            self.objective_name.clone(),
            self.player_name.clone(),
            Box::new(self.objective_type.clone()),
        )
    }
    fn get_variables(&self) -> HashMap<String, Rc<VariableObject>> {
        let mut map = HashMap::new();
        map.insert(
            "selector",
            mk_variable(Objects::String(self.player_name.clone()), Objects::Unknown),
        );
        map.insert(
            "objective",
            mk_variable(
                Objects::String(self.objective_name.clone()),
                Objects::Unknown,
            ),
        );

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
        fn create_operator_func(
            operator: Operator,
        ) -> Box<dyn Fn(Vec<Rc<dyn Object>>, Option<Rc<VariableObject>>) -> Rc<dyn Object>>
        {
            return Box::new(move |args, var| {
                let value = args[0].get_type();
                if var.is_none() {
                    panic!("No variable found");
                }

                let var = var.unwrap();
                if let Objects::ScoreboardPlayerPair(objective_name, player_name, objective_type) =
                    *var.value.clone()
                {
                    match value {
                        Objects::Number(value) => match_objects(Objects::MutationVariable(
                            match_objects(Objects::ScoreboardPlayerPair(
                                objective_name.clone(),
                                player_name.clone(),
                                objective_type.clone(),
                            )),
                            Box::new(Objects::ScoreboardPlayerPair(
                                objective_name,
                                player_name,
                                objective_type.clone(),
                            )),
                            operator.clone(),
                            match_objects(Objects::ScoreboardPlayerPair(
                                "".to_string(),
                                "value".to_string(),
                                Box::new(Objects::Number(0)),
                            )),
                            Box::new(Objects::Number(value)),
                        )),
                        Objects::ScoreboardPlayerPair(new_objective, new_player, obj_type) => {
                            let second_scoreboard_pair = ScoreboardPlayerPairObject {
                                objective_type: *obj_type.clone(),
                                objective_name: new_objective,
                                player_name: new_player,
                            };

                            let first_scoreboard_pair = ScoreboardPlayerPairObject {
                                objective_type: *objective_type.clone(),
                                objective_name,
                                player_name,
                            };

                            match_objects(Objects::MutationVariable(
                                Rc::new(first_scoreboard_pair.clone()),
                                Box::new(first_scoreboard_pair.get_type()),
                                operator.clone(),
                                Rc::new(second_scoreboard_pair.clone()),
                                Box::new(second_scoreboard_pair.get_type()),
                            ))
                        }
                        // TODO: Add Variable support
                        _ => panic!("Invalid arguments"),
                    }
                } else {
                    panic!("Invalid arguments")
                }
            });
        }
        let mut map = mk_function_map();

        map.insert("add".to_string(), create_operator_func(Operator::Add));
        map.insert(
            "multiply".to_string(),
            create_operator_func(Operator::Multiply),
        );
        map.insert("divide".to_string(), create_operator_func(Operator::Divide));
        map.insert("sub".to_string(), create_operator_func(Operator::Subtract));
        map.insert(
            "modulus".to_string(),
            create_operator_func(Operator::Modulus),
        );
        map.insert(
            "set".to_string(),
            create_operator_func(Operator::Assignment),
        );

        map
    }
}
impl Object for ScoreboardObject {
    fn get_type(&self) -> Objects {
        Objects::Scoreboard(
            self.name.clone(),
            self.objective.clone(),
            Box::new(self.objective_type.clone()),
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
        let mut map = mk_function_map();
        map.insert(
            "get_player".to_string(),
            Box::new(|args, var| {
                if let None = var {
                    panic!("No variable found");
                }
                let var = var.unwrap();
                if let Objects::Scoreboard(sb_name, _, sb_type) = *var.value.clone() {
                    let name = args[0].get_type();
                    match name {
                        Objects::String(name) => match_objects(Objects::ScoreboardPlayerPair(
                            sb_name.clone(),
                            name,
                            sb_type,
                        )),
                        _ => panic!("Invalid arguments"),
                    }
                } else {
                    panic!("Invalid arguments")
                }
            }),
        );

        map.insert(
            "instantiate".to_string(),
            Box::new(|args, _| {
                let name = args[0].get_type();
                let objective = args[1].get_type();
                match (name, objective) {
                    (Objects::String(name), Objects::String(objective)) => match_objects(
                        Objects::Scoreboard(name, objective, Box::new(Objects::Number(0))),
                    ),
                    _ => panic!("Invalid arguments"),
                }
            }),
        );
        map
    }
}
