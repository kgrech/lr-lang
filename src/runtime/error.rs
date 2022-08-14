use crate::ast::value::Type;
use crate::runtime::executor::ExpressionError;
use crate::runtime::frame::VariableError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("{0}")]
    ExpressionError(#[from] ExpressionError),
    #[error("{0}")]
    VariableError(#[from] VariableError),
    #[error("The result type of the {0} expression is not Bool, but {1}")]
    NonBooleanCondition(String, Type),
}
