#[derive(Debug, Clone)]
pub enum ASTOperation {
    Function(String, Vec<ASTOperation>),
    CreateFunction(String, Vec<ASTOperation>),
    Create(String, Vec<ASTOperation>),
    MutateVariable(String, Vec<ASTOperation>),
    CodeBlock(Vec<ASTOperation>),
    Access(String),
    AssignVariable(String, Vec<ASTOperation>),
    LiteralString(String),
    LiteralNumber(i64),
    LiteralBool(bool),
    Set(Vec<ASTOperation>),
    AccessPart(Box<ASTOperation>),
    UseVariable(String, Box<ASTOperation>),
    If(Vec<ASTOperation>, Box<ASTOperation>),
    Operation(Box<ASTOperation>, Operator, Box<ASTOperation>)
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
    Not
}

#[derive(Debug, Clone)]
pub struct NodeStatement {
    operation: ASTOperation
}

impl NodeStatement {
    
}
