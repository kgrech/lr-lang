use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::String => write!(f, "String"),
            Type::Bool => write!(f, "Bool"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
}

impl From<&Value> for Type {
    fn from(value: &Value) -> Self {
        match value {
            Value::Int(_) => Self::Int,
            Value::Float(_) => Self::Float,
            Value::String(_) => Self::String,
            Value::Bool(_) => Self::Bool,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "\"{}\"", v),
            Value::Bool(v) => write!(f, "{}", v),
        }
    }
}
