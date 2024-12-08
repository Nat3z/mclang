mod executor;

mod lexer {
    pub mod lexer;
    pub mod tokens;
}
mod ast {
    pub mod constructor;
    pub mod operations;
}

mod compile {
    pub mod compiler;
    pub mod mcstatements;
    pub mod objects;
}

mod errors {
    pub mod error;
}


fn main() {
    executor::run();
}
