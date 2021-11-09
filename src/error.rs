use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LlkError {
    InvalidGrammar(String),
    UnknownSymbol(char),
    IllegalOperation(String),
    DerivationFailed(String),
    GrammarFromJsonFailed(String),
}

impl fmt::Display for LlkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        const ERROR_TYPE_NAME: &str = "llk";

        match self {
            LlkError::InvalidGrammar(description) => {
                write!(f, "{}: invalid grammar: {}", ERROR_TYPE_NAME, description)
            }
            LlkError::UnknownSymbol(symbol) => {
                write!(f, "{}: unknown symbol '{}'", ERROR_TYPE_NAME, symbol)
            }
            LlkError::IllegalOperation(description) => {
                write!(f, "{}: illegal operation: {}", ERROR_TYPE_NAME, description)
            }
            LlkError::DerivationFailed(description) => {
                write!(f, "{}: derivation failed: {}", ERROR_TYPE_NAME, description)
            }
            LlkError::GrammarFromJsonFailed(description) => {
                write!(f, "{}: parsing grammar from JSON failed: {}", ERROR_TYPE_NAME, description)
            }
        }
    }
}

impl Error for LlkError {}
