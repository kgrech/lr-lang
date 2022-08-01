use crate::ast::expression::{Expr, Opcode};
use crate::ast::program::Program;
use crate::ast::statement::Statement;
use crate::runtime::error::RuntimeError;
use crate::runtime::frame::Frame;

pub fn evalutate_expression(frame: &mut Frame, expr: &Expr) -> Result<i32, RuntimeError> {
    let value = match expr {
        Expr::Number(n) => *n,
        Expr::Op(exp1, opcode, exp2) => match opcode {
            Opcode::Mul => evalutate_expression(frame, exp1)? * evalutate_expression(frame, exp2)?,
            Opcode::Div => evalutate_expression(frame, exp1)? / evalutate_expression(frame, exp2)?,
            Opcode::Add => evalutate_expression(frame, exp1)? + evalutate_expression(frame, exp2)?,
            Opcode::Sub => evalutate_expression(frame, exp1)? - evalutate_expression(frame, exp2)?,
        },
        Expr::Identifier(variable) => frame.variable_value(variable)?,
    };
    Ok(value)
}

pub fn execute_statement(frame: &mut Frame, statement: &Statement) -> Result<(), RuntimeError> {
    match statement {
        Statement::Assignment(body) => {
            let value = evalutate_expression(frame, &body.expression)?;
            frame.assign_value(&body.identifier, value)?;
        }
        Statement::Definition(body) => {
            let value = evalutate_expression(frame, &body.expression)?;
            frame.define_variable(body.identifier.clone(), value);
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
