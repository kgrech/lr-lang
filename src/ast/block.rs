use crate::ast::expression::Expr;
use crate::ast::statement::Statement;

#[derive(PartialEq, Debug)]
pub enum Block {
    StatementsBlock(Vec<Statement>),
    Condition {
        expression: Box<Expr>,
        then_block: Vec<Statement>,
        else_block: Option<Vec<Statement>>,
    },
}

impl Block {
    pub fn new_statements(statements: Vec<Statement>) -> Self {
        Self::StatementsBlock(statements)
    }

    pub fn new_condition(
        expression: Box<Expr>,
        then_block: Vec<Statement>,
        else_block: Option<Vec<Statement>>,
    ) -> Self {
        Self::Condition {
            expression,
            then_block,
            else_block,
        }
    }
}
