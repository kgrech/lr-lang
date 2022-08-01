use crate::runtime::error::RuntimeError;
use std::collections::HashMap;

/// Represents a stack frame.
/// It contains set of local variables and possibly a parent frame
#[derive(Debug, Default)]
pub struct Frame {
    parent: Option<Box<Frame>>,
    local_variables: HashMap<String, i32>,
}

impl Frame {
    pub fn variable_value(&self, variable_name: &str) -> Result<i32, RuntimeError> {
        if let Some(&value) = self.local_variables.get(variable_name) {
            Ok(value)
        } else if let Some(parent) = self.parent.as_ref() {
            parent.variable_value(variable_name)
        } else {
            Err(RuntimeError::UndefinedVariable(variable_name.to_owned()))
        }
    }

    pub fn assign_value(&mut self, variable_name: &str, value: i32) -> Result<(), RuntimeError> {
        if let Some(variable) = self.local_variables.get_mut(variable_name) {
            *variable = value;
            Ok(())
        } else if let Some(parent) = self.parent.as_mut() {
            parent.assign_value(variable_name, value)
        } else {
            Err(RuntimeError::UndefinedVariable(variable_name.to_owned()))
        }
    }

    pub fn define_variable(&mut self, variable_name: String, value: i32) {
        if let Some(variable) = self.local_variables.get_mut(&variable_name) {
            *variable = value;
        } else {
            self.local_variables.insert(variable_name, value);
        }
    }
}
