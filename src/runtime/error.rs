use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Variable {0} is not defined")]
    UndefinedVariable(String),
}
