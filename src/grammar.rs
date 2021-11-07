use std::collections::HashMap;
use std::collections::HashSet;

use crate::error::LlkError;

pub struct LlkGrammar {
    pub term_symbols: HashSet<char>,
    pub  nterm_symbols: HashSet<char>,
    pub  start_symbol: char,
    pub  lookahead: usize,
    pub  productions: HashMap<char, Vec<Option<String>>>,
}

pub(super) const EOF: char = '\u{0003}';

impl LlkGrammar {
    pub fn new(
        term_symbols: HashSet<char>,
        nterm_symbols: HashSet<char>,
        start_symbol: char,
        lookahead: usize,
        productions: Vec<(char, Option<String>)>,
    ) -> Result<LlkGrammar, LlkError> {
        unimplemented!()
    }

    pub fn from_json(json_string: &str) -> Result<LlkGrammar, LlkError> {
        unimplemented!()
    }
}

impl LlkGrammar {
    pub(super) fn is_term(&self, symbol: char) -> bool {
        self.term_symbols.contains(&symbol) || symbol == EOF
    }

    pub(super) fn is_nterm(&self, symbol: char) -> bool {
        self.nterm_symbols.contains(&symbol) || self.start_symbol == symbol
    }

    pub(super) fn derives_epsilon(&self, symbol: char) -> bool {
        !self.is_term(symbol) && self.productions[&symbol].contains(&None)
    }

    pub(super) fn derive(&self, symbol: char) -> Vec<Option<String>> {
        if self.is_nterm(symbol) {
            self.productions[&symbol].clone()
        } else {
            vec![Some(symbol.to_string())]
        }
    }

    pub(super) fn get_term_prefixes(&self, string: &str) -> HashSet<String> {
        /* Recursive expansion procedure */
        fn inner(grammar: &LlkGrammar, string: &str, prefix_len: usize) -> Vec<String> {
            if let Some(leftmost_nterm) = string.chars().find(|&c| grammar.is_nterm(c)) {
                /* String contains at least one non-terminal symbol */
                let (prefix, suffix) = string.split_once(leftmost_nterm).unwrap();

                if prefix.len() >= prefix_len {
                    /* If prefix length is enough return it */
                    vec![format!(
                        "{term_prefix:.limit$}",
                        term_prefix = prefix,
                        limit = prefix_len
                    )]
                } else {
                    let prefix_rest_len = prefix_len - prefix.len();

                    grammar
                        .derive(leftmost_nterm)
                        .drain(..)
                        .map(|d| d.unwrap_or_default())
                        .filter(|s| !s.starts_with(leftmost_nterm)) // TODO Сомнительное рещение для предотвращения беск. рекурсии
                        .flat_map(|s| {
                            inner(grammar, &(s + suffix), prefix_rest_len)
                        })
                        .map(|s| {
                            format!(
                                "{prefix_start}{prefix_end}",
                                prefix_start = prefix,
                                prefix_end = s,
                            )
                        })
                        .collect()
                }
            } else {
                /* If string contains only terminal symbols return its k-prefix */
                vec![format!(
                    "{term_prefix:.limit$}",
                    term_prefix = string,
                    limit = prefix_len
                )]
            }
        }

        inner(self, string, self.lookahead).drain(..).collect()
    }
}

mod grammar_assert {
    use super::LlkGrammar;
    use crate::error::LlkError;
    use std::collections::HashSet;

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
