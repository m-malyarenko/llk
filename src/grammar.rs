use std::collections::HashMap;
use std::collections::HashSet;

use crate::error::LlkError;

pub struct LlkGrammar {
    term_symbols: HashSet<char>,
    nterm_symbols: HashSet<char>,
    start_symbol: char,
    lookahead: usize,
    productions: HashMap<char, Vec<String>>,
}

impl LlkGrammar {
    pub fn new(
        term_symbols: HashSet<char>,
        nterm_symbols: HashSet<char>,
        start_symbol: char,
        lookahead: usize,
        productions: Vec<(char, String)>,
    ) -> Result<LlkGrammar, LlkError> {
        unimplemented!()
    }

    pub fn from_json(json_string: &str) -> Result<LlkGrammar, LlkError> {
        unimplemented!()
    }
}

mod grammar_assert {
    use std::collections::HashSet;
    use super::LlkGrammar;
    use crate::error::LlkError;

    fn assert_grammar(grammar: &LlkGrammar) -> Result<(), LlkError> {
        unimplemented!()
    }

    fn assert_symbols(grammar: &LlkGrammar) -> Result<(), LlkError> {
        unimplemented!()
    }

    fn assert_rules(grammar: &LlkGrammar) -> Result<(), LlkError> {
        unimplemented!()
    }

    fn assert_lookahead(grammar: &LlkGrammar) -> Result<(), LlkError> {
        unimplemented!()
    }

    fn get_reachable_nterms(grammar: &LlkGrammar) -> HashSet<char> {
        unimplemented!()
    }

    fn get_resolvable_nterms(grammar: &LlkGrammar) -> HashSet<char> {
        unimplemented!()
    }
}