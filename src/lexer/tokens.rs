pub enum Tokens {
    RParen(String),
    LParen(String),
    RBrace(String),
    LBrace(String),

    Number(String),
    DblQuote(String),

    Period(String),
    Let(String),

    If(String),
    Else(String),
    ElseIf(String),
}
