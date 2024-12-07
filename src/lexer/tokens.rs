#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Parens(Vec<Tokens>),
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

    If(Vec<Tokens>),
    Else(Vec<Tokens>),
    ElseIf(Vec<Tokens>),
    Comma,

    Equivalence,
    New(String, Vec<Tokens>),

    EOL,
    EOF,
    None,
}

