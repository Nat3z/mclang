use std::{
    collections::HashMap,
    mem::{discriminant, Discriminant},
    process::exit,
    rc::Rc,
};

use crate::{
    ast::operations::{ASTOperation, Operator},
    compile::obj::std::VariableObject,
};

use super::{
    mcstatements::{
        compile_into_mcstatement, execute_step_str, ExecuteSteps, MinecraftStatementObject,
        Statements,
    },
    obj::{
        scoreboard::ScoreboardPlayerPairObject,
        std::{
            compile_into_if_statement, compile_into_mutation_variable, compile_into_variable,
            compile_into_while_loop,
        },
    },
    objects::{match_objects, name_into_object, Object, Objects},
};

pub struct Compiler {
    pub scopes: Vec<Scope>,
    pub namespace: String,
    pub outputs: HashMap<String, String>,
}

#[derive(Clone)]
pub struct Scope {
    pub variables: HashMap<String, Variable>,
    pub statements: Vec<ASTOperation>,
    pub namespace: String,
    pub scopes: Vec<Scope>,
    pub name: String,
}
#[derive(Clone, Debug)]
pub struct Variable {
    pub name: String,
    pub value: Rc<dyn Object>,
    pub static_variable: bool,
}
impl Compiler {
    pub fn new(statements: Vec<ASTOperation>, namespace: &'static str) -> Compiler {
        Compiler {
            scopes: vec![Scope::new(
                "global".to_string(),
                namespace.to_string(),
                statements,
            )],
            namespace: namespace.to_string(),
            outputs: HashMap::new(),
        }
    }

    pub fn compile_into(
        &mut self,
        scope: &mut Scope,
        value: Rc<dyn Object>,
    ) -> (String, Option<Scope>) {
        match value.get_type() {
            Objects::MCStatement(statement) => return compile_into_mcstatement(statement),
            Objects::MutationVariable(left, left_obj, operand, right, right_obj) => {
                eprintln!(
                    "Mutation Variable: {:#?} {:#?} {:#?} {:#?} {:#?}",
                    left, left_obj, operand, right, right_obj
                );
                return compile_into_mutation_variable(
                    left.as_any().downcast_ref().unwrap(),
                    *left_obj,
                    operand,
                    right.as_any().downcast_ref(),
                    *right_obj,
                );
            }
            Objects::Variable(object, scoreboard) => {
                return compile_into_variable(*object, *scoreboard)
            }
            Objects::IfStatement(statements, code_block) => {
                return compile_into_if_statement(statements, code_block, scope, self)
            }
            Objects::While(name, iterator, code_block) => {
                let compiled_value =
                    compile_into_while_loop(name, iterator, code_block, scope, self);

                return (compiled_value, None);
            }
            _ => {
                println!("Invalid: {:?}", value.get_type());
            }
        }
        return (String::new(), None);
    }

    pub fn compile(&mut self, current_scope: &mut Scope) {
        let mut index = 0;

        println!("-----------------");

        let mut output_str = String::new();
        while current_scope.statements.len() > index {
            let current_statement = current_scope.statements[index].clone();
            let value = current_scope.execute(&current_statement, None);
            let (compiled_value, mut new_scope) = self.compile_into(current_scope, value);
            output_str.push_str(&format!("\n{}", &compiled_value));

            index += 1;
            if new_scope.is_some() {
                current_scope.scopes.push(new_scope.clone().unwrap());
                self.compile(new_scope.as_mut().unwrap());
            }
        }

        self.outputs.insert(current_scope.name.clone(), output_str);
    }

    pub fn flush(&self) -> &HashMap<String, String> {
        &self.outputs
    }
}

impl Scope {
    pub fn new(name: String, namespace: String, statements: Vec<ASTOperation>) -> Scope {
        Scope {
            variables: HashMap::new(),
            statements,
            namespace,
            scopes: vec![],
            name,
        }
    }

    pub fn execute(
        &mut self,
        instruction: &ASTOperation,
        current_variable: Option<Variable>,
    ) -> Rc<dyn Object> {
        match instruction {
            ASTOperation::LiteralString(str) => {
                return match_objects(Objects::String(str.clone()));
            }
            ASTOperation::LiteralNumber(num) => {
                return match_objects(Objects::Number(*num));
            }
            ASTOperation::LiteralBool(bool) => {
                return match_objects(Objects::Boolean(*bool));
            }
            ASTOperation::AssignVariable(name, operation) => {
                if operation.len() != 1 {
                    eprintln!("More than 1 operation in assign variable");
                    exit(1);
                }
                let value = self.execute(&operation[0], current_variable);
                if discriminant(&value.get_type())
                    == discriminant(&Objects::Variable(
                        Box::new(Objects::Unknown),
                        Box::new(Objects::Unknown),
                    ))
                {
                    if let Objects::Variable(value, _) = value.get_type() {
                        let variable = Objects::Variable(
                            Box::new(*value.clone()),
                            Box::new(Objects::Scoreboard(
                                format!("v_{}_{}", self.name, self.variables.len()),
                                "dummy".to_string(),
                                Box::new(*value.clone()),
                            )),
                        );
                        let variable = match_objects(variable);
                        self.variables.insert(
                            name.clone(),
                            Variable {
                                name: name.clone(),
                                value: variable.clone(),
                                static_variable: false,
                            },
                        );
                        return variable;
                    }
                }
                let variable = Objects::Variable(
                    Box::new(value.clone().get_type()),
                    Box::new(Objects::Scoreboard(
                        format!("v_{}_{}", self.name, self.variables.len()),
                        "dummy".to_string(),
                        Box::new(value.clone().get_type()),
                    )),
                );
                let variable = match_objects(variable);
                self.variables.insert(
                    name.clone(),
                    Variable {
                        name: name.clone(),
                        value: variable.clone(),
                        static_variable: false,
                    },
                );
                return variable;
            }
            ASTOperation::StaticVariable(name, operation) => {
                if operation.len() != 1 {
                    eprintln!("More than 1 operation in assign variable");
                    exit(1);
                }
                let value = self.execute(&operation[0], current_variable);

                let variable = Objects::Variable(
                    Box::new(value.clone().get_type()),
                    Box::new(Objects::Scoreboard(
                        format!("v_{}_{}", self.name, self.variables.len()),
                        "dummy".to_string(),
                        Box::new(value.clone().get_type()),
                    )),
                );
                let variable = match_objects(variable);
                self.variables.insert(
                    name.clone(),
                    Variable {
                        name: name.clone(),
                        value: variable.clone(),
                        static_variable: true,
                    },
                );
                return match_objects(Objects::Unknown);
            }
            ASTOperation::MutateVariable(name, operation) => {
                if operation.len() != 1 {
                    eprintln!("More than 1 operation in assign variable");
                    exit(1);
                }
                let value = self.execute(&operation[0], current_variable);
                println!("Value: {:?}", value.get_type());
                if !self.variables.contains_key(name) {
                    eprintln!("Variable {} does not exist", name);
                    exit(1);
                }
                let variable = self.variables.get(name).unwrap().clone();

                if variable.static_variable {
                    eprintln!("Attempting to mutate static variable.");
                    exit(1);
                }

                if let Objects::Variable(variable_value, scoreboard) = variable.value.get_type() {
                    let discrim_variable_value = discriminant(&*variable_value);
                    let value_box = Box::new(value.get_type());
                    println!("Value: {:?}", variable_value);
                    if discrim_variable_value != discriminant(&value.get_type())
                        && discriminant(&value.get_type())
                            != discriminant(&Objects::MutationVariable(
                                match_objects(Objects::Unknown),
                                Box::new(Objects::Unknown),
                                Operator::Add,
                                match_objects(Objects::Unknown),
                                Box::new(Objects::Unknown),
                            ))
                        && discriminant(&value.get_type())
                            != discriminant(&Objects::Variable(
                                Box::new(Objects::Unknown),
                                Box::new(Objects::Unknown),
                            ))
                        && discriminant(&value.get_type())
                            != discriminant(&Objects::ScoreboardPlayerPair(
                                "".to_string(),
                                "".to_string(),
                                Box::new(Objects::Unknown),
                            ))
                    {
                        eprintln!(
                            "Invalid mutation: Type mismatch {:?} to {:?}",
                            variable_value,
                            value.get_type()
                        );
                        exit(1);
                    }
                    let new_variable = Variable {
                        name: name.clone(),
                        value: match_objects(Objects::Variable(
                            value_box.clone(),
                            scoreboard.clone(),
                        )),
                        static_variable: false,
                    };

                    let old_type = variable.clone().value.get_type();
                    if let Objects::Variable(_, scoreboard) = old_type {
                        if let Objects::Scoreboard(scoreboard_name, _, obj_type) = *scoreboard {
                            if let Objects::MutationVariable(_, old_obj, operand, new, new_obj) =
                                value.get_type()
                            {
                                let old_discrim = discriminant(&*old_obj);
                                let new_discrim = discriminant(&*new_obj);
                                if new_discrim
                                    == discriminant(&Objects::Variable(
                                        Box::new(Objects::Unknown),
                                        Box::new(Objects::Unknown),
                                    ))
                                {
                                    println!("bap");
                                    // this new is also a variable.
                                    let new = match_objects(*new_obj.clone());
                                    let new =
                                        new.as_any().downcast_ref::<VariableObject>().unwrap();
                                    let new_discrim = discriminant(&*new.value);

                                    if old_discrim != new_discrim {
                                        eprintln!(
                                            "Invalid mutation: Type mismatch {:?} to {:?}",
                                            variable_value, new
                                        );

                                        exit(1);
                                    }
                                    match *new_obj.clone() {
                                        Objects::Number(num) => {
                                            return match_objects(Objects::MutationVariable(
                                                match_objects(Objects::ScoreboardPlayerPair(
                                                    scoreboard_name,
                                                    "value".to_string(),
                                                    obj_type,
                                                )),
                                                Box::new(variable.clone().value.get_type()),
                                                operand.clone(),
                                                match_objects(Objects::ScoreboardPlayerPair(
                                                    "".to_string(),
                                                    "".to_string(),
                                                    Box::new(Objects::Number(num)),
                                                )),
                                                Box::new(Objects::Number(num)),
                                            ));
                                        }
                                        Objects::Boolean(bool) => {
                                            return match_objects(Objects::MutationVariable(
                                                match_objects(Objects::ScoreboardPlayerPair(
                                                    scoreboard_name,
                                                    "value".to_string(),
                                                    obj_type,
                                                )),
                                                Box::new(variable.clone().value.get_type()),
                                                operand.clone(),
                                                match_objects(Objects::ScoreboardPlayerPair(
                                                    "".to_string(),
                                                    "".to_string(),
                                                    Box::new(Objects::Boolean(bool)),
                                                )),
                                                Box::new(Objects::Boolean(bool)),
                                            ));
                                        }
                                        Objects::ScoreboardPlayerPair(
                                            new_player_name,
                                            new_objective_name,
                                            new_obj_type,
                                        ) => {
                                            return match_objects(Objects::MutationVariable(
                                                match_objects(Objects::ScoreboardPlayerPair(
                                                    scoreboard_name,
                                                    "value".to_string(),
                                                    obj_type,
                                                )),
                                                Box::new(variable.clone().value.get_type()),
                                                operand.clone(),
                                                match_objects(Objects::ScoreboardPlayerPair(
                                                    new_player_name.clone(),
                                                    new_objective_name.clone(),
                                                    new_obj_type.clone(),
                                                )),
                                                Box::new(Objects::ScoreboardPlayerPair(
                                                    new_player_name,
                                                    new_objective_name,
                                                    new_obj_type,
                                                )),
                                            ));
                                        }
                                        Objects::Variable(_, variable_scoreboard) => {
                                            if let Objects::Scoreboard(
                                                objective_name,
                                                _,
                                                new_obj_type,
                                            ) = *variable_scoreboard
                                            {
                                                match *new_obj_type.clone() {
                                                    Objects::Number(_) | Objects::Boolean(_) => {
                                                        return match_objects(
                                                            Objects::MutationVariable(
                                                                match_objects(
                                                                    Objects::ScoreboardPlayerPair(
                                                                        scoreboard_name,
                                                                        "value".to_string(),
                                                                        obj_type,
                                                                    ),
                                                                ),
                                                                Box::new(
                                                                    variable
                                                                        .clone()
                                                                        .value
                                                                        .get_type(),
                                                                ),
                                                                operand.clone(),
                                                                match_objects(
                                                                    Objects::ScoreboardPlayerPair(
                                                                        objective_name.to_string(),
                                                                        "value".to_string(),
                                                                        Box::new(new.get_type()),
                                                                    ),
                                                                ),
                                                                Box::new(new.get_type()),
                                                            ),
                                                        );
                                                    }
                                                    Objects::ScoreboardPlayerPair(
                                                        new_player_name,
                                                        new_objective_name,
                                                        new_obj_type,
                                                    ) => {
                                                        println!("MEEEEE");
                                                        return match_objects(
                                                            Objects::MutationVariable(
                                                                match_objects(
                                                                    Objects::ScoreboardPlayerPair(
                                                                        scoreboard_name,
                                                                        "value".to_string(),
                                                                        obj_type,
                                                                    ),
                                                                ),
                                                                Box::new(
                                                                    variable
                                                                        .clone()
                                                                        .value
                                                                        .get_type(),
                                                                ),
                                                                operand.clone(),
                                                                match_objects(
                                                                    Objects::ScoreboardPlayerPair(
                                                                        new_objective_name.clone(),
                                                                        new_player_name.clone(),
                                                                        new_obj_type.clone(),
                                                                    ),
                                                                ),
                                                                Box::new(
                                                                    Objects::ScoreboardPlayerPair(
                                                                        new_objective_name,
                                                                        new_player_name,
                                                                        new_obj_type,
                                                                    ),
                                                                ),
                                                            ),
                                                        );
                                                    }
                                                    _ => {
                                                        eprintln!(
                                                            "Invalid variable (Variable MATCH), {:?}",
                                                            new_obj.clone()
                                                        );
                                                        exit(1);
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            eprintln!(
                                                "Invalid variable (Variable MATCH), {:?}",
                                                new_obj.clone()
                                            );
                                            exit(1);
                                        }
                                    }
                                }
                                // TODO: FIX THIS LATER
                                // if old_discrim != new_discrim {
                                //     eprintln!(
                                //         "Invalid mutation: Type mismatch {:?} to {:?}",
                                //         old_obj,
                                //         new_obj.clone()
                                //     );
                                //     exit(1);
                                // }
                                println!("henlo sending");
                                return match_objects(Objects::MutationVariable(
                                    match_objects(Objects::ScoreboardPlayerPair(
                                        scoreboard_name,
                                        "value".to_string(),
                                        Box::new(variable.clone().value.get_type()),
                                    )),
                                    Box::new(variable.clone().value.get_type()),
                                    operand.clone(),
                                    match_objects(Objects::ScoreboardPlayerPair(
                                        "".to_string(),
                                        "value".to_string(),
                                        Box::new(*new_obj.clone()),
                                    )),
                                    Box::new(*new_obj.clone()),
                                ));
                            } else if let Objects::Variable(new_value, new_scoreboard) =
                                value.get_type()
                            {
                                let new_value = match_objects(*new_value).get_type();
                                println!("BAP");
                                if let Objects::Scoreboard(new_objective_name, objective, _) =
                                    *new_scoreboard
                                {
                                    println!("BUMP");
                                    if let Objects::Number(new_num) = new_value {
                                        let right_pair =
                                            match_objects(Objects::ScoreboardPlayerPair(
                                                new_objective_name.clone(),
                                                "value".to_string(),
                                                Box::new(Objects::Number(0)),
                                            ));

                                        return match_objects(Objects::MutationVariable(
                                            match_objects(Objects::ScoreboardPlayerPair(
                                                name.clone(),
                                                "value".to_string(),
                                                Box::new(variable.clone().value.get_type()),
                                            )),
                                            Box::new(variable.clone().value.get_type()),
                                            Operator::Assignment,
                                            right_pair.clone(),
                                            Box::new(right_pair.get_type()),
                                        ));
                                    } else if let Objects::Boolean(new_bool) = new_value {
                                        let right_pair =
                                            match_objects(Objects::ScoreboardPlayerPair(
                                                new_objective_name.clone(),
                                                "value".to_string(),
                                                Box::new(Objects::Boolean(false)),
                                            ));

                                        return match_objects(Objects::MutationVariable(
                                            match_objects(Objects::ScoreboardPlayerPair(
                                                name.clone(),
                                                "value".to_string(),
                                                Box::new(variable.clone().value.get_type()),
                                            )),
                                            Box::new(variable.clone().value.get_type()),
                                            Operator::Assignment,
                                            right_pair.clone(),
                                            Box::new(right_pair.get_type()),
                                        ));
                                    } else {
                                        panic!("Invalid mutation between {:?}", value.get_type());
                                    }
                                }
                            } else if let Objects::Number(new_num) = value.get_type() {
                                return match_objects(Objects::MutationVariable(
                                    match_objects(Objects::ScoreboardPlayerPair(
                                        scoreboard_name,
                                        "value".to_string(),
                                        Box::new(variable.clone().value.get_type()),
                                    )),
                                    Box::new(variable.clone().value.get_type()),
                                    Operator::Assignment,
                                    match_objects(Objects::ScoreboardPlayerPair(
                                        "".to_string(),
                                        "".to_string(),
                                        Box::new(Objects::Number(0)),
                                    )),
                                    Box::new(Objects::Number(new_num)),
                                ));
                            } else if let Objects::Boolean(new_bool) = value.get_type() {
                                return match_objects(Objects::MutationVariable(
                                    match_objects(Objects::ScoreboardPlayerPair(
                                        scoreboard_name,
                                        "value".to_string(),
                                        Box::new(variable.clone().value.get_type()),
                                    )),
                                    Box::new(variable.clone().value.get_type()),
                                    Operator::Assignment,
                                    match_objects(Objects::ScoreboardPlayerPair(
                                        "".to_string(),
                                        "".to_string(),
                                        Box::new(Objects::Boolean(false)),
                                    )),
                                    Box::new(Objects::Boolean(new_bool)),
                                ));
                            } else {
                                panic!("Invalid mutation between {:?}", value.get_type());
                            }
                            // self.variables.insert(name.clone(), new_variable.clone());
                        }
                    }
                } else if let Objects::ScoreboardPlayerPair(name, objective, _) =
                    variable.value.get_type()
                {
                    println!("BINK");
                    if let Objects::Number(num) = value.get_type() {
                        return match_objects(Objects::MutationVariable(
                            match_objects(Objects::ScoreboardPlayerPair(
                                name.clone(),
                                objective.clone(),
                                Box::new(Objects::Number(0)),
                            )),
                            Box::new(Objects::Number(0)),
                            Operator::Assignment,
                            match_objects(Objects::ScoreboardPlayerPair(
                                "".to_string(),
                                "".to_string(),
                                Box::new(Objects::Number(0)),
                            )),
                            Box::new(Objects::Number(num)),
                        ));
                    } else if let Objects::Boolean(bool) = value.get_type() {
                        return match_objects(Objects::MutationVariable(
                            match_objects(Objects::ScoreboardPlayerPair(
                                name.clone(),
                                objective.clone(),
                                Box::new(Objects::Boolean(false)),
                            )),
                            Box::new(Objects::Boolean(false)),
                            Operator::Assignment,
                            match_objects(Objects::ScoreboardPlayerPair(
                                "".to_string(),
                                "".to_string(),
                                Box::new(Objects::Boolean(false)),
                            )),
                            Box::new(Objects::Boolean(bool)),
                        ));
                    } else if let Objects::ScoreboardPlayerPair(
                        old_player_name,
                        old_objective_name,
                        _,
                    ) = value.get_type()
                    {
                        if let Objects::MutationVariable(_, old_obj, operand, new, new_obj) =
                            value.get_type()
                        {
                            println!("BAP BAP");
                            return match_objects(Objects::MutationVariable(
                                match_objects(Objects::ScoreboardPlayerPair(
                                    name.clone(),
                                    objective.clone(),
                                    Box::new(Objects::Number(0)),
                                )),
                                Box::new(Objects::Number(0)),
                                operand.clone(),
                                value,
                                Box::new(*new_obj),
                            ));
                        }
                    } else {
                        eprintln!("Invalid variable (Variable TYPE)");
                        exit(1);
                    }
                }
                eprintln!("Invalid variable (Variable)");
                exit(1);
            }
            ASTOperation::Access(name) => {
                if current_variable.is_none() {
                    if !self.variables.contains_key(name) {
                        eprintln!("Variable {} does not exist", name);
                        exit(1);
                    }
                    let real_variable = self.variables.get(name);
                    if real_variable.is_none() {
                        eprintln!("Variable {} does not exist", name);
                        exit(1);
                    }
                    let real_variable = real_variable.unwrap();
                    let variable = real_variable
                        .value
                        .as_any()
                        .downcast_ref::<VariableObject>();
                    if variable.is_none() {
                        eprintln!("Invalid variable access.");
                        exit(1);
                    }

                    let variable = variable.unwrap();
                    if real_variable.static_variable {
                        return match_objects(*variable.value.clone());
                    }

                    return Rc::new(variable.clone());
                }
                let variable = current_variable.unwrap();
                let variables = variable.value.as_any().downcast_ref::<VariableObject>();
                if variables.is_none() {
                    eprintln!("Invalid variable access.");
                    exit(1);
                }

                if variable.static_variable {
                    let variables =
                        match_objects(*variables.unwrap().value.clone()).get_variables();
                    let variable = variables.get(name).expect("Unknown variable");
                    return match_objects(*variable.value.clone());
                }
                let variables = match_objects(*variables.unwrap().value.clone()).get_variables();
                let variable = variables.get(name).expect("Unknown variable");
                return variable.clone();
            }
            ASTOperation::UseVariable(name, operation) => {
                if current_variable.is_none() {
                    let variable = self.variables.get(name).expect("Variable not found");
                    let value = self.execute(&operation, Some(variable.clone()));
                    return value;
                }

                let variable = current_variable.as_ref().unwrap().value.as_any();
                let variable = match_objects(
                    *variable
                        .downcast_ref::<VariableObject>()
                        .expect("Unknown Variable.")
                        .value
                        .clone(),
                );

                println!("Variable: {:?}", variable);
                let variable = variable.get_variables();
                let variable = variable.get(name).expect("Variable not found.");
                println!("Got: {:?}", variable);
                let variable = Variable {
                    name: name.clone(),
                    value: variable.clone(),
                    static_variable: true,
                };
                let value = self.execute(&operation, Some(variable));
                return value;
            }
            // runs this inside of the variable
            ASTOperation::AccessPart(operation) => {
                let value = self.execute(&operation, current_variable);
                return value;
            }
            ASTOperation::Operation(first_statement, operator, second_statement) => {
                let first_value = self.execute(&first_statement, current_variable.clone());
                let second_value = self.execute(&second_statement, current_variable.clone());
                if *operator == Operator::Add
                    || *operator == Operator::Subtract
                    || *operator == Operator::Assignment
                    || *operator == Operator::Multiply
                    || *operator == Operator::Divide
                    || *operator == Operator::Modulus
                {
                    // TODO: Make this system
                    let first_value = first_value.as_any().downcast_ref::<VariableObject>();

                    if first_value.is_none() {
                        eprintln!("Invalid variable {:?}", first_value);
                        exit(1);
                    }

                    let first_value = first_value.unwrap();

                    let mut first_scoreboard_pair: Option<Rc<dyn Object>> = None;
                    if let Objects::Scoreboard(name, objective, _) = *first_value.scoreboard.clone()
                    {
                        // assigns first player pair
                        match *first_value.value.clone() {
                            Objects::Number(_) | Objects::Boolean(_) => {
                                first_scoreboard_pair =
                                    Some(match_objects(Objects::ScoreboardPlayerPair(
                                        name,
                                        "value".to_string(),
                                        first_value.value.clone(),
                                    )));
                            }
                            Objects::ScoreboardPlayerPair(player_name, objective_name, _) => {
                                first_scoreboard_pair =
                                    Some(match_objects(Objects::ScoreboardPlayerPair(
                                        objective_name,
                                        player_name,
                                        first_value.value.clone(),
                                    )));
                            }
                            _ => {
                                eprintln!("Invalid variable Access");
                                exit(1);
                            }
                        }
                        let second_value_original = second_value.clone();
                        let second_value = second_value.as_any().downcast_ref::<VariableObject>();

                        let mut second_scoreboard_pair: Option<Rc<dyn Object>> = None;

                        if second_value.is_some() {
                            let second_value = second_value.unwrap();

                            if let Objects::Scoreboard(name, objective, _) =
                                *second_value.scoreboard.clone()
                            {
                                println!("ASSIGNING SECOND");
                                // assigns first player pair
                                match *second_value.value.clone() {
                                    Objects::Number(_) | Objects::Boolean(_) => {
                                        second_scoreboard_pair =
                                            Some(match_objects(Objects::ScoreboardPlayerPair(
                                                name,
                                                "value".to_string(),
                                                second_value.value.clone(),
                                            )));
                                    }
                                    Objects::ScoreboardPlayerPair(
                                        player_name,
                                        objective_name,
                                        obj_type,
                                    ) => {
                                        println!("MEEEEEEEEEEEEEEEEEE");
                                        second_scoreboard_pair =
                                            Some(match_objects(Objects::ScoreboardPlayerPair(
                                                objective_name,
                                                player_name,
                                                second_value.value.clone(),
                                            )));
                                    }
                                    _ => {
                                        eprintln!("Invalid variable");
                                        exit(1);
                                    }
                                }
                            } else {
                                eprintln!("Invalid variable (SB)");
                                exit(1);
                            }
                            // TODO: ADD MUTATE VARIABLE LOGIC
                        } else {
                            match second_value_original.get_type() {
                                Objects::Number(_) | Objects::Boolean(_) => {
                                    second_scoreboard_pair =
                                        Some(match_objects(Objects::ScoreboardPlayerPair(
                                            "value".to_string(),
                                            "".to_string(),
                                            Box::new(second_value_original.get_type()),
                                        )));
                                }
                                _ => {
                                    eprintln!("Invalid variable (SB1)");
                                    exit(1);
                                }
                            }
                        }
                        let second_value = if second_value.is_some() {
                            Box::new(second_value.unwrap().get_type().clone())
                        } else {
                            Box::new(second_value_original.get_type())
                        };

                        let first_scoreboard_pair = if first_scoreboard_pair.is_some() {
                            first_scoreboard_pair.unwrap().get_type()
                        } else {
                            Objects::ScoreboardPlayerPair(
                                "".to_string(),
                                "value".to_string(),
                                Box::new(Objects::Number(0)),
                            )
                        };

                        let second_scoreboard_pair = if second_scoreboard_pair.is_some() {
                            second_scoreboard_pair.unwrap().get_type()
                        } else {
                            Objects::ScoreboardPlayerPair(
                                "".to_string(),
                                "value".to_string(),
                                Box::new(Objects::Number(0)),
                            )
                        };

                        println!("im here");
                        println!("ORIGINAL SECOND: {:?}", second_value_original.get_type());
                        return match_objects(Objects::MutationVariable(
                            match_objects(first_scoreboard_pair.clone()),
                            Box::new(first_scoreboard_pair),
                            operator.clone(),
                            match_objects(second_scoreboard_pair.clone()),
                            Box::new(second_value_original.get_type()),
                        ));
                    } else {
                        eprintln!("Invalid variable (SB1)");
                        exit(1);
                    }
                } else {
                    return match_objects(Objects::MCStatement(Statements::Execute(vec![
                        ExecuteSteps::Compare(
                            first_value.get_type().clone(),
                            operator.clone(),
                            second_value.get_type().clone(),
                        ),
                    ])));
                }
            }
            ASTOperation::If(operations, codeblock) => {
                let mut values: Vec<Rc<dyn Object>> = vec![];
                for operation in operations {
                    println!("Executing: {:?}", operation);
                    values.push(self.execute(&operation, current_variable.clone()));
                }

                return match_objects(Objects::IfStatement(values, codeblock.clone()));
            }

            ASTOperation::Function(name, set) => {
                let function = current_variable.as_ref().unwrap();

                let current_variable_as_object = current_variable
                    .as_ref()
                    .unwrap()
                    .value
                    .as_any()
                    .downcast_ref::<VariableObject>()
                    .unwrap();
                let function = function.value.as_any().downcast_ref::<VariableObject>();
                if function.is_none() {
                    eprintln!("Failed to convert to variable in function call.");
                    exit(1);
                }
                println!("Function: {:?}", function);
                let function = match_objects(*function.unwrap().value.clone()).get_functions();
                let function = function.get(name);
                if function.is_none() {
                    println!("bad: {:?}", current_variable.unwrap());
                    eprintln!("Unknown function: {}", name);
                    exit(1);
                }

                let function = function.unwrap();
                if let ASTOperation::Set(operations) = &set[0] {
                    let mut items: Vec<Rc<dyn Object>> = vec![];

                    for operation in operations {
                        let execution = self.execute(&operation, None);
                        if let Objects::Variable(value, _) = execution.get_type() {
                            match *value {
                                Objects::Number(_) | Objects::Boolean(_) => {
                                    let variable_object = execution
                                        .as_any()
                                        .downcast_ref::<VariableObject>()
                                        .unwrap();
                                    if let Objects::Scoreboard(name, _, _) =
                                        *variable_object.scoreboard.clone()
                                    {
                                        items.push(match_objects(Objects::ScoreboardPlayerPair(
                                            name,
                                            "value".to_string(),
                                            Box::new(*value.clone()),
                                        )));
                                    } else {
                                        items.push(match_objects(*value));
                                    }
                                }
                                _ => {
                                    items.push(match_objects(*value));
                                }
                            }
                        } else {
                            items.push(execution);
                        }
                    }

                    return function(items, Some(Rc::new(current_variable_as_object.clone())));
                } else {
                    eprintln!("Missing set.");
                    exit(1);
                }
            }

            ASTOperation::While(name, set, code) => {
                println!("While: {:?} {:?} {:?}", name, set, code);
                // if the operation is instead an access, then we need to get the variable.
                let mut iterator = self.execute(&set[0], current_variable.clone());

                let variable_iterator = iterator.as_any().downcast_ref::<VariableObject>();
                if variable_iterator.is_some() {
                    let variable_iterator = variable_iterator.unwrap();
                    iterator = match_objects(*variable_iterator.value.clone());
                }
                if let Objects::Array(iterator) = iterator.get_type() {
                    return match_objects(Objects::While(name.clone(), iterator, code.clone()));
                } else {
                    eprintln!("Invalid iterator. {:?}", iterator.get_type());
                    exit(1);
                }
            }

            ASTOperation::Create(object_name, params) => {
                let object = name_into_object(object_name);
                let object = object.get_functions();
                let function = object.get("instantiate");
                if function.is_none() {
                    eprintln!("Cannot instantiate object (No instantiate function)");
                    exit(1);
                }
                let function = function.unwrap();
                if let ASTOperation::Set(operations) = &params[0] {
                    let mut items: Vec<Rc<dyn Object>> = vec![];

                    for operation in operations {
                        let execution = self.execute(&operation, None);
                        if let Objects::Variable(value, _) = execution.get_type() {
                            items.push(match_objects(*value));
                        } else {
                            items.push(execution);
                        }
                    }

                    return function(items, None);
                } else if params.len() > 0 {
                    let mut execution = self.execute(&params[0], None);
                    if let Objects::Variable(value, _) = execution.get_type() {
                        execution = match_objects(*value);
                    }
                    return function(vec![execution], None);
                } else {
                    eprintln!("Missing parameters.");
                    exit(1);
                }
            }
            ASTOperation::Set(multiple) => {
                let mut set_values: Vec<Rc<dyn Object>> = vec![];
                for operation in multiple {
                    set_values.push(self.execute(&operation, current_variable.clone()));
                }
                return match_objects(Objects::Array(set_values));
            }
            _ => {
                return match_objects(Objects::Unknown);
            }
        }
    }
}
