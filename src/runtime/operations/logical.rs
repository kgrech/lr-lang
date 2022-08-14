use crate::ast::value::Value;
use crate::runtime::operations::OperationError;
use std::ops::Not;

impl Not for Value {
    type Output = Result<Value, OperationError>;

    fn not(self) -> Self::Output {
        match self {
            Value::Int(v) => Ok(Value::Int(!v)),
            Value::Float(_) => unary_error!(Not, Float),
            Value::String(_) => unary_error!(Not, String),
            Value::Bool(v) => Ok(Value::Bool(!v)),
        }
    }
}

pub fn conjunction(value_1: Value, value_2: Value) -> Result<Value, OperationError> {
    match value_1 {
        Value::Int(_) => error_other!(Int, Conj, value_2),
        Value::Float(_) => error_other!(Float, Conj, value_2),
        Value::String(_) => error_other!(String, Conj, value_2),
        Value::Bool(v1) => match value_2 {
            Value::Int(_) => error!(Bool, Conj, Int),
            Value::Float(_) => error!(Bool, Conj, Float),
            Value::String(_) => error!(Bool, Conj, String),
            Value::Bool(v2) => Ok(Value::Bool(v1 && v2)),
        },
    }
}

pub fn disjunction(value_1: Value, value_2: Value) -> Result<Value, OperationError> {
    match value_1 {
        Value::Int(_) => error_other!(Int, Disj, value_2),
        Value::Float(_) => error_other!(Float, Disj, value_2),
        Value::String(_) => error_other!(String, Disj, value_2),
        Value::Bool(v1) => match value_2 {
            Value::Int(_) => error!(Bool, Disj, Int),
            Value::Float(_) => error!(Bool, Disj, Float),
            Value::String(_) => error!(Bool, Disj, String),
            Value::Bool(v2) => Ok(Value::Bool(v1 || v2)),
        },
    }
}

pub fn equals(value_1: Value, value_2: Value) -> Result<Value, OperationError> {
    match value_1 {
        Value::Int(v1) => match value_2 {
            Value::Int(v2) => Ok(Value::Bool(v1 == v2)),
            other => error_other!(Int, Equals, other),
        },
        Value::Float(v1) => match value_2 {
            Value::Float(v2) => Ok(Value::Bool(v1 == v2)),
            other => error_other!(Int, Equals, other),
        },
        Value::String(v1) => match value_2 {
            Value::String(v2) => Ok(Value::Bool(v1 == v2)),
            other => error_other!(String, Equals, other),
        },
        Value::Bool(v1) => match value_2 {
            Value::Bool(v2) => Ok(Value::Bool(v1 == v2)),
            other => error_other!(Bool, Equals, other),
        },
    }
}

pub fn not_equals(value_1: Value, value_2: Value) -> Result<Value, OperationError> {
    match value_1 {
        Value::Int(v1) => match value_2 {
            Value::Int(v2) => Ok(Value::Bool(v1 != v2)),
            other => error_other!(Int, Equals, other),
        },
        Value::Float(v1) => match value_2 {
            Value::Float(v2) => Ok(Value::Bool(v1 != v2)),
            other => error_other!(Int, Equals, other),
        },
        Value::String(v1) => match value_2 {
            Value::String(v2) => Ok(Value::Bool(v1 != v2)),
            other => error_other!(String, Equals, other),
        },
        Value::Bool(v1) => match value_2 {
            Value::Bool(v2) => Ok(Value::Bool(v1 != v2)),
            other => error_other!(Bool, Equals, other),
        },
    }
}

pub fn greater(value_1: Value, value_2: Value) -> Result<Value, OperationError> {
    match value_1 {
        Value::Int(v1) => match value_2 {
            Value::Int(v2) => Ok(Value::Bool(v1 > v2)),
            other => error_other!(Int, Greater, other),
        },
        Value::Float(v1) => match value_2 {
            Value::Float(v2) => Ok(Value::Bool(v1 > v2)),
            other => error_other!(Int, Greater, other),
        },
        Value::String(v1) => match value_2 {
            Value::String(v2) => Ok(Value::Bool(v1 > v2)),
            other => error_other!(String, Greater, other),
        },
        Value::Bool(_) => error_other!(Bool, Greater, value_2),
    }
}

pub fn greater_equals(value_1: Value, value_2: Value) -> Result<Value, OperationError> {
    match value_1 {
        Value::Int(v1) => match value_2 {
            Value::Int(v2) => Ok(Value::Bool(v1 >= v2)),
            other => error_other!(Int, GreaterEquals, other),
        },
        Value::Float(v1) => match value_2 {
            Value::Float(v2) => Ok(Value::Bool(v1 >= v2)),
            other => error_other!(Int, GreaterEquals, other),
        },
        Value::String(v1) => match value_2 {
            Value::String(v2) => Ok(Value::Bool(v1 >= v2)),
            other => error_other!(String, GreaterEquals, other),
        },
        Value::Bool(_) => error_other!(Bool, GreaterEquals, value_2),
    }
}

pub fn lower(value_1: Value, value_2: Value) -> Result<Value, OperationError> {
    match value_1 {
        Value::Int(v1) => match value_2 {
            Value::Int(v2) => Ok(Value::Bool(v1 < v2)),
            other => error_other!(Int, Lower, other),
        },
        Value::Float(v1) => match value_2 {
            Value::Float(v2) => Ok(Value::Bool(v1 < v2)),
            other => error_other!(Int, Lower, other),
        },
        Value::String(v1) => match value_2 {
            Value::String(v2) => Ok(Value::Bool(v1 < v2)),
            other => error_other!(String, Lower, other),
        },
        Value::Bool(_) => error_other!(Bool, Lower, value_2),
    }
}

pub fn lower_equals(value_1: Value, value_2: Value) -> Result<Value, OperationError> {
    match value_1 {
        Value::Int(v1) => match value_2 {
            Value::Int(v2) => Ok(Value::Bool(v1 <= v2)),
            other => error_other!(Int, LowerEquals, other),
        },
        Value::Float(v1) => match value_2 {
            Value::Float(v2) => Ok(Value::Bool(v1 <= v2)),
            other => error_other!(Int, LowerEquals, other),
        },
        Value::String(v1) => match value_2 {
            Value::String(v2) => Ok(Value::Bool(v1 <= v2)),
            other => error_other!(String, LowerEquals, other),
        },
        Value::Bool(_) => error_other!(Bool, LowerEquals, value_2),
    }
}
