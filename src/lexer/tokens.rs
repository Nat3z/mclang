use crate::errors::associate::CodeAssociate;

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Parens(Vec<Tokens>, CodeAssociate),
    Bracket(Vec<Tokens>, CodeAssociate),
    RBrace(CodeAssociate),
    LBrace(CodeAssociate),
    SemiColon(CodeAssociate),

    Number(String, CodeAssociate),
    DblQuote(String, CodeAssociate),
    Bool(bool, CodeAssociate),

    Period(Vec<Tokens>, CodeAssociate),
    Symbol(String, CodeAssociate),
    Let(String, CodeAssociate),
    Assignment(CodeAssociate),
    Add(CodeAssociate),
    Subtract(CodeAssociate),
    Divide(CodeAssociate),
    Multiply(CodeAssociate),
    Modulus(CodeAssociate),

    If(Vec<Tokens>, CodeAssociate),
    While(String, Vec<Tokens>, CodeAssociate),
    And(CodeAssociate),
    Or(CodeAssociate),
    Comma(CodeAssociate),

    Equivalence(CodeAssociate),
    GreaterThan(CodeAssociate),
    LesserThan(CodeAssociate),
    Function(String, Vec<Tokens>, CodeAssociate),
    GreaterThanEqual(CodeAssociate),
    LesserThanEqual(CodeAssociate),
    NotEqual(CodeAssociate),
    New(String, Vec<Tokens>, CodeAssociate),
    Export(CodeAssociate),
    Import(String, CodeAssociate),

    EOL,
    EOF,
    None,
}
