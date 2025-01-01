#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Parens(Vec<Tokens>),
    Bracket(Vec<Tokens>),
    RBrace,
    LBrace,
    SemiColon,

    Number(String),
    DblQuote(String),
    Bool(bool),

    Period(Vec<Tokens>),
    Symbol(String),
    Let(String),
    Assignment,
    Add,
    Subtract,
    Divide,
    Multiply,
    Modulus,

    If(Vec<Tokens>),
    While(String, Vec<Tokens>),
    And,
    Or,
    Comma,

    Equivalence,
    GreaterThan,
    LesserThan,
    Function(String, Vec<Tokens>),
    GreaterThanEqual,
    LesserThanEqual,
    NotEqual,
    New(String, Vec<Tokens>),
    Export,
    Import(String),

    EOL,
    EOF,
    None,
}
