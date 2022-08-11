use crate::ast::value::{Type, Value};
use std::collections::HashMap;
use std::ops::Deref;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VariableError {
    #[error("Variable {0} is not defined")]
    UndefinedVariable(String),
    #[error("Unable to assign the value of type {0} to variable '{1}' of type {2}")]
    TypeMismatch(Type, String, Type),
}

/// Represents a stack frame.
/// It contains set of local variables and possibly a parent frame
#[derive(Debug, Default)]
pub struct Frame {
    parent: Option<Box<Frame>>,
    local_variables: HashMap<String, Value>,
}

impl Frame {
    pub fn variable_value(&self, variable_name: &str) -> Result<Value, VariableError> {
        if let Some(value) = self.local_variables.get(variable_name) {
            Ok(value.clone())
        } else if let Some(parent) = self.parent.as_ref() {
            parent.variable_value(variable_name)
        } else {
            Err(VariableError::UndefinedVariable(variable_name.to_owned()))
        }
    }

    pub fn assign_value(&mut self, variable_name: &str, value: Value) -> Result<(), VariableError> {
        if let Some(variable) = self.local_variables.get_mut(variable_name) {
            if Type::from(variable.deref()) == Type::from(&value) {
                *variable = value;
                Ok(())
            } else {
                Err(VariableError::TypeMismatch(
                    Type::from(&value),
                    variable_name.to_owned(),
                    Type::from(variable.deref()),
                ))
            }
        } else if let Some(parent) = self.parent.as_mut() {
            parent.assign_value(variable_name, value)
        } else {
            Err(VariableError::UndefinedVariable(variable_name.to_owned()))
        }
    }

    pub fn define_variable(
        &mut self,
        variable_name: String,
        value_type: Type,
        value: Value,
    ) -> Result<(), VariableError> {
        if value_type == Type::from(&value) {
            if let Some(variable) = self.local_variables.get_mut(&variable_name) {
                *variable = value;
            } else {
                self.local_variables.insert(variable_name, value);
            }
            Ok(())
        } else {
            Err(VariableError::TypeMismatch(
                Type::from(&value),
                variable_name,
                value_type,
            ))
        }
    }
}
