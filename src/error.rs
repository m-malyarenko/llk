use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LlkError {
    UnknownSymbol(char),
    IllegalOperation(String),
    DerivationFailed(String),
}

impl fmt::Display for LlkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unimplemented!()
    }
}

impl Error for LlkError {}
