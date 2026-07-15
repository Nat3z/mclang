use crate::errors::associate::CodeAssociate;

#[derive(Debug, Clone)]
pub enum ASTOperation {
    Function(String, Vec<ASTOperation>, CodeAssociate),
    CreateFunction(String, Vec<String>, Vec<ASTOperation>, CodeAssociate),
    Create(String, Vec<ASTOperation>, CodeAssociate),
    MutateVariable(String, Vec<ASTOperation>, CodeAssociate),
    CodeBlock(Vec<ASTOperation>, CodeAssociate),
    Access(String, CodeAssociate),
    AssignVariable(String, Vec<ASTOperation>, CodeAssociate),
    StaticVariable(String, Vec<ASTOperation>, CodeAssociate),
    LiteralString(String, CodeAssociate),
    LiteralNumber(i64, CodeAssociate),
    LiteralBool(bool, CodeAssociate),
    Set(Vec<ASTOperation>, CodeAssociate),
    AccessPart(Box<ASTOperation>, CodeAssociate),
    UseVariable(String, Box<ASTOperation>, CodeAssociate),
    If(Vec<ASTOperation>, Box<ASTOperation>, CodeAssociate),
    While(String, Vec<ASTOperation>, Box<ASTOperation>, CodeAssociate),
    Operation(
        Box<ASTOperation>,
        Operator,
        Box<ASTOperation>,
        CodeAssociate,
    ),
    Export(Box<ASTOperation>, CodeAssociate),
    Import(String, CodeAssociate),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    Assignment,
    And,
    Or,
    Not,
}

#[derive(Debug, Clone)]
pub struct NodeStatement {
    operation: ASTOperation,
}

impl NodeStatement {}
