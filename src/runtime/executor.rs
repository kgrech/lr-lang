use crate::ast::expression::{Expr, Opcode};
use crate::ast::program::Program;
use crate::ast::statement::Statement;
use crate::ast::value::Value;
use crate::runtime::error::RuntimeError;
use crate::runtime::frame::{Frame, VariableError};
use crate::runtime::operations::OperationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExpressionError {
    #[error("Unable to evalutate expression {0}: {1}")]
    VariableError(String, VariableError),
    #[error("Unable to evalutate expression {0}: {1}")]
    OperationError(String, OperationError),
}

pub fn evalutate_expression(frame: &mut Frame, expr: &Expr) -> Result<Value, ExpressionError> {
    match expr {
        Expr::Constant(n) => Ok(n.clone()),
        Expr::Op(exp1, opcode, exp2) => {
            let result = match opcode {
                Opcode::Mul => {
                    evalutate_expression(frame, exp1)? * evalutate_expression(frame, exp2)?
                }
                Opcode::Div => {
                    evalutate_expression(frame, exp1)? / evalutate_expression(frame, exp2)?
                }
                Opcode::Add => {
                    evalutate_expression(frame, exp1)? + evalutate_expression(frame, exp2)?
                }
                Opcode::Sub => {
                    evalutate_expression(frame, exp1)? - evalutate_expression(frame, exp2)?
                }
            };
            result.map_err(|e| ExpressionError::OperationError(expr.to_string(), e))
        }
        Expr::Identifier(variable) => frame
            .variable_value(variable)
            .map_err(|e| ExpressionError::VariableError(expr.to_string(), e)),
    }
}

pub fn execute_statement(frame: &mut Frame, statement: &Statement) -> Result<(), RuntimeError> {
    match statement {
        Statement::Assignment {
            identifier,
            expression,
        } => {
            let value = evalutate_expression(frame, expression)?;
            frame.assign_value(identifier, value)?;
        }
        Statement::Definition {
            identifier,
            expression,
            value_type,
        } => {
            let value = evalutate_expression(frame, expression)?;
            frame.define_variable(identifier.clone(), value_type.clone(), value)?;
        }
    }
    Ok(())
}

pub fn execute_program(frame: &mut Frame, program: &Program) -> Result<(), RuntimeError> {
    for statement in &program.statements {
        execute_statement(frame, statement)?
    }
    Ok(())
}
