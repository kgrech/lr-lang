use crate::ast::value::Value;
use crate::runtime::operations::OperationError;
use std::ops::{Add, Div, Mul, Sub};

impl Add for Value {
    type Output = Result<Value, OperationError>;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Value::Int(v1) => match other {
                Value::Int(v2) => Ok(Value::Int(v1 + v2)),
                Value::Float(v2) => Ok(Value::Float(v1 as f32 + v2)),
                Value::String(v2) => Ok(Value::String(v1.to_string() + v2.as_str())),
                Value::Bool(_) => error!(Int, Add, Bool),
            },
            Value::Float(v1) => match other {
                Value::Int(v2) => Ok(Value::Float(v1 + v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 + v2)),
                Value::String(v2) => Ok(Value::String(v1.to_string() + v2.as_str())),
                Value::Bool(_) => error!(Float, Add, Bool),
            },
            Value::String(v1) => match other {
                Value::Int(v2) => Ok(Value::String(v1 + v2.to_string().as_str())),
                Value::Float(v2) => Ok(Value::String(v1 + v2.to_string().as_str())),
                Value::String(v2) => Ok(Value::String(v1 + v2.as_str())),
                Value::Bool(v2) => Ok(Value::String(v1 + v2.to_string().as_str())),
            },
            Value::Bool(v1) => match other {
                Value::Int(_) => error!(Bool, Add, Int),
                Value::Float(_) => error!(Bool, Add, Int),
                Value::String(v2) => Ok(Value::String(v1.to_string() + v2.as_str())),
                Value::Bool(_) => error!(Bool, Add, Bool),
            },
        }
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
                Value::Bool(_) => error!(Int, Sub, Bool),
            },
            Value::Float(v1) => match other {
                Value::Int(v2) => Ok(Value::Float(v1 - v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 - v2)),
                Value::String(_) => error!(Float, Sub, String),
                Value::Bool(_) => error!(Float, Sub, Bool),
            },
            Value::String(_) => match other {
                Value::Int(_) => error!(String, Sub, Int),
                Value::Float(_) => error!(String, Sub, Float),
                Value::String(_) => error!(String, Sub, String),
                Value::Bool(_) => error!(String, Sub, Bool),
            },
            Value::Bool(_) => error_other!(Bool, Sub, other),
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
                Value::Bool(_) => error!(Int, Mul, Bool),
            },
            Value::Float(v1) => match other {
                Value::Int(v2) => Ok(Value::Float(v1 * v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 * v2)),
                Value::String(_) => error!(Float, Mul, String),
                Value::Bool(_) => error!(Float, Mul, Bool),
            },
            Value::String(_) => match other {
                Value::Int(_) => error!(String, Mul, Int),
                Value::Float(_) => error!(String, Mul, Float),
                Value::String(_) => error!(String, Mul, String),
                Value::Bool(_) => error!(String, Div, Bool),
            },
            Value::Bool(_) => error_other!(Bool, Mul, other),
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
                Value::Bool(_) => error!(Int, Mul, Bool),
            },
            Value::Float(v1) => match other {
                Value::Int(v2) => Ok(Value::Float(v1 / v2 as f32)),
                Value::Float(v2) => Ok(Value::Float(v1 / v2)),
                Value::String(_) => error!(Float, Div, String),
                Value::Bool(_) => error!(Float, Div, Bool),
            },
            Value::String(_) => match other {
                Value::Int(_) => error!(String, Div, Int),
                Value::Float(_) => error!(String, Div, Float),
                Value::String(_) => error!(String, Div, String),
                Value::Bool(_) => error!(String, Div, Bool),
            },
            Value::Bool(_) => error_other!(Bool, Div, other),
        }
    }
}
