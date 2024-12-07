use crate::ast::operations::ASTOperation;

pub struct Compiler {
    statements: Vec<ASTOperation>,
    index: usize,
    scopes: Vec<Scope>
}

pub struct Scope {
    variables: Vec<Variable>,
    name: String
}

pub struct Variable {
    name: String,
    value: ASTOperation
}
impl Compiler {
    pub fn new(statements: Vec<ASTOperation>) -> Compiler {
        Compiler {
            statements,
            index: 0,
            scopes: vec![]
        }
    }

    pub fn compile(&self) {
        while self.statements.len() > self.index {
            let current_statement = self.statements[self.index].clone();

            match current_statement {
                _ => {}
            }
        }
    }
}
