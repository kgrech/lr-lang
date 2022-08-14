use crate::ast::block::Block;
use crate::ast::expression::Expr;
use crate::ast::value::Type;

#[derive(PartialEq, Debug)]
pub enum Statement {
    Assignment {
        identifier: String,
        expression: Box<Expr>,
    },
    Definition {
        identifier: String,
        expression: Box<Expr>,
        value_type: Type,
    },
    Block(Block),
}

impl Statement {
    pub fn new_assignment(identifier: String, expression: Box<Expr>) -> Self {
        Self::Assignment {
            identifier,
            expression,
        }
    }

    pub fn new_definition(identifier: String, value_type: Type, expression: Box<Expr>) -> Self {
        Self::Definition {
            identifier,
            value_type,
            expression,
        }
    }

    pub fn new_block(block: Block) -> Self {
        Self::Block(block)
    }
}
