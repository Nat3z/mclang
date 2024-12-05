use super::tokens::Tokens;

pub struct Lexer {

}

pub struct Token {
    token_type: Tokens,
    line: i32,
    column: i32
}

impl Token {
    pub fn new(token_type: Tokens, line: i32, column: i32) -> Token {
        Token {
            token_type,
            line,
            column
        }
    }
}
