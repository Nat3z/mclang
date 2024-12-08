use crate::ast::operations::Operator;

use super::objects::Objects;

#[derive(Debug, Clone)]
pub enum Statements {
    Execute(Vec<ExecuteSteps>),
}
#[derive(Debug, Clone)]
pub enum ExecuteSteps {
    As(Objects),
    At(Objects),
    In(Objects),
    Compare(Objects, Operator, Objects)
}
