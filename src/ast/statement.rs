use crate::ast::expression::Expr;

pub struct StatementBody {
    pub identifier: String,
    pub expression: Box<Expr>,
}

pub enum Statement {
    Assignment(StatementBody),
    Definition(StatementBody),
}

impl Statement {
    pub fn new_assignment(identifier: String, expression: Box<Expr>) -> Self {
        Self::Assignment(StatementBody {
            identifier,
            expression,
        })
    }

    pub fn new_definition(identifier: String, expression: Box<Expr>) -> Self {
        Self::Definition(StatementBody {
            identifier,
            expression,
        })
    }
}
