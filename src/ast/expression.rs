use crate::ast::value::Value;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum Expr {
    Constant(Value),
    Identifier(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Constant(v) => write!(f, "{}", v),
            Expr::Identifier(id) => write!(f, "{}", id),
            Expr::Op(e1, op, e2) => write!(f, "({} {} {})", e1, op, e2),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::Mul => write!(f, "*"),
            Opcode::Div => write!(f, "/"),
            Opcode::Add => write!(f, "+"),
            Opcode::Sub => write!(f, "-"),
        }
    }
}
