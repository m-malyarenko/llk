pub mod error;
pub mod grammar;
pub mod tree;

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

pub use crate::error::LlkError;
pub use crate::grammar::LlkGrammar;
pub use crate::tree::LlkTree;

pub struct Llk {
    grammar: LlkGrammar,
    lut: HashMap<(char, String), String>,
    stack: RefCell<Vec<char>>,
}

impl Llk {
    pub fn new() -> Llk {
        unimplemented!()
    }

    pub fn parse(&self, string: &str) -> Result<LlkTree, LlkError> {
        unimplemented!()
    }
}
