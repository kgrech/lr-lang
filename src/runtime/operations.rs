use crate::ast::expression::Opcode;
use crate::ast::value::{Type, Value};
use std::ops::{Add, Div, Mul, Sub};
use thiserror::Error;

macro_rules! error {
    ($type_1:ident, $op:ident, $type_2:ident) => {
        Err(OperationError::IncompatibleTypes(
            Type::$type_1,
            Opcode::$op,
            Type::$type_2,
        ))
    };
}

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("Operation {0} {1} {2} is not defined")]
    IncompatibleTypes(Type, Opcode, Type),
}

impl Add for Value {
    type Output = Result<Value, OperationError>;

    fn add(self, other: Self) -> Self::Output {
        let value = match self {
            Value::Int(v1) => match other {
                Value::Int(v2) => Value::Int(v1 + v2),
                Value::Float(v2) => Value::Float(v1 as f32 + v2),
                Value::String(v2) => Value::String(v1.to_string() + v2.as_str()),
            },
            Value::Float(v1) => match other {
                Value::Int(v2) => Value::Float(v1 + v2 as f32),
                Value::Float(v2) => Value::Float(v1 + v2),
                Value::String(v2) => Value::String(v1.to_string() + v2.as_str()),
            },
            Value::String(v1) => match other {
                Value::Int(v2) => Value::String(v1 + v2.to_string().as_str()),
                Value::Float(v2) => Value::String(v1 + v2.to_string().as_str()),
                Value::String(v2) => Value::String(v1 + v2.as_str()),
            },
        };
        Ok(value)
    }
}

impl Sub for Value {
    type Output = Result<Value, OperationError>;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Value::Int(v1) => match other {
                Value::Int(v2) => Ok(Value::Int(v1 - v2)),
                Value::Float(v2) => Ok(Value::Float(v1 as f32 - v2)),
                Value::String(_) => error!(Int, Sub, String),
            },
            Value::Float(v1) => match other {
                Value::Int(v2) => Ok(Value::Float(v1 - v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 - v2)),
                Value::String(_) => error!(Float, Sub, String),
            },
            Value::String(_) => match other {
                Value::Int(_) => error!(String, Sub, Int),
                Value::Float(_) => error!(String, Sub, Float),
                Value::String(_) => error!(String, Sub, String),
            },
        }
    }
}

impl Mul for Value {
    type Output = Result<Value, OperationError>;

    fn mul(self, other: Self) -> Self::Output {
        match self {
            Value::Int(v1) => match other {
                Value::Int(v2) => Ok(Value::Int(v1 * v2)),
                Value::Float(v2) => Ok(Value::Float(v1 as f32 * v2)),
                Value::String(_) => error!(Int, Mul, String),
            },
            Value::Float(v1) => match other {
                Value::Int(v2) => Ok(Value::Float(v1 * v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 * v2)),
                Value::String(_) => error!(Float, Mul, String),
            },
            Value::String(_) => match other {
                Value::Int(_) => error!(String, Mul, Int),
                Value::Float(_) => error!(String, Mul, Float),
                Value::String(_) => error!(String, Mul, String),
            },
        }
    }
}

impl Div for Value {
    type Output = Result<Value, OperationError>;

    fn div(self, other: Self) -> Self::Output {
        match self {
            Value::Int(v1) => match other {
                Value::Int(v2) => Ok(Value::Int(v1 / v2)),
                Value::Float(v2) => Ok(Value::Float(v1 as f32 / v2)),
                Value::String(_) => error!(Int, Div, String),
            },
            Value::Float(v1) => match other {
                Value::Int(v2) => Ok(Value::Float(v1 / v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 / v2)),
                Value::String(_) => error!(Float, Div, String),
            },
            Value::String(_) => match other {
                Value::Int(_) => error!(String, Div, Int),
                Value::Float(_) => error!(String, Div, Float),
                Value::String(_) => error!(String, Div, String),
            },
        }
    }
}
