use crate::ast::expression::{BinaryOpcode, UnaryOpcode};
use crate::ast::value::Type;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("Operation {0} {1} {2} is not defined")]
    IncompatibleTypes(Type, BinaryOpcode, Type),
    #[error("Operation {0} {1} is not defined")]
    IncompatibleType(UnaryOpcode, Type),
}

macro_rules! unary_error {
    ($op:ident, $type_:ident) => {
        Err(OperationError::IncompatibleType(
            crate::ast::expression::UnaryOpcode::$op,
            crate::ast::value::Type::$type_,
        ))
    };
}

macro_rules! error {
    ($type_1:ident, $op:ident, $type_2:ident) => {
        Err(OperationError::IncompatibleTypes(
            crate::ast::value::Type::$type_1,
            crate::ast::expression::BinaryOpcode::$op,
            crate::ast::value::Type::$type_2,
        ))
    };
}
macro_rules! error_other {
    ($type_1:ident, $op:ident, $other:ident) => {
        Err(OperationError::IncompatibleTypes(
            crate::ast::value::Type::$type_1,
            crate::ast::expression::BinaryOpcode::$op,
            (&$other).into(),
        ))
    };
}

mod logical;
mod math;

pub use logical::*;
