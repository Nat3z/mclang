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
    mod obj {
        pub mod entity;
        pub mod basic;
        pub mod scoreboard;
        pub mod std;
        pub mod blockpos;
    }
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
