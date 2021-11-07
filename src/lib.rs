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

impl Llk {
    fn first(&self, string: &str) -> HashSet<Option<String>> {
        let mut first_set = HashSet::new();

        if string.is_empty() {
            /* Calculate FIRST set of ε */
            first_set.insert(None);
        } else if string.len() == 1 {
            /* Calculate FIRST set of a symbol */
            let symbol = string.chars().nth(0).unwrap();

            if self.grammar.is_term(symbol) {
                /* FIRST set of terminal symbol is itself */
                first_set.insert(Some(string.to_owned()));
            } else {
                /* FIRST set of non-termonal symbol */

                /* If non-terminal symbol derives ε then add ε to its FIRST set */
                if self.grammar.derives_epsilon(symbol) {
                    first_set.insert(None);
                }

                /* Get all non-terminal non-ε derivatives */
                let derivatives: Vec<String> = self
                    .grammar
                    .derive(symbol)
                    .drain(..)
                    .filter_map(|d| d)
                    .collect();

                /* Scan through all derivatives */
                for derivative in derivatives {
                    /* Skip symbols that have ε in their FIRST set */
                    let non_empty_suffix = if let Some(suffix) = derivative
                        .strip_prefix(|c: char| self.first(&c.to_string()).contains(&None))
                    {
                        suffix
                    } else {
                        &derivative
                    };

                    if non_empty_suffix.is_empty() {
                        first_set.insert(None);
                    } else {
                        /* Get all k-prefixes of non-ε suffix of the production's RHS */
                        first_set.extend(
                            self.grammar
                                .get_term_prefixes(non_empty_suffix)
                                .drain()
                                .map(|s| Some(s)),
                        )
                    }
                }
            }
        } else {
            /* Calculate FIRST set of a string */
            /* Most often used to define the FIRST set of the RHS of a production */
            if string
                .chars()
                .all(|symbol| self.first(&symbol.to_string()).contains(&None))
            {
                first_set.insert(None);
            };

            first_set.extend(
                self.grammar
                    .get_term_prefixes(string)
                    .drain()
                    .map(|s| Some(s)),
            )
        }

        first_set
    }

    fn follow(&self, nterm: char) -> HashSet<String> {
        unimplemented!()
    }
}

#[test]
fn first_set_test() {
    use std::iter::FromIterator;

    let mut grammar = LlkGrammar {
        term_symbols: vec!['a', 'b', '$'].drain(..).collect(),
        nterm_symbols: vec!['A'].drain(..).collect(),
        start_symbol: 'S',
        lookahead: 3,
        productions: HashMap::new(),
    };

    grammar
        .productions
        .insert('S', vec![Some("Ab$".to_string())]);
    grammar
        .productions
        .insert('A', vec![Some("aA".to_string()), Some("a".to_string())]);

    let llk = Llk {
        grammar,
        lut: HashMap::new(),
        stack: RefCell::new(Vec::new()),
    };

    assert_eq!(
        llk.first("S"),
        HashSet::from_iter(vec![
            Some("ab$".to_string()),
            Some("aab".to_string()),
            Some("aaa".to_string())
        ])
    );
    assert_eq!(
        llk.first("A"),
        HashSet::from_iter(vec![
            Some("a".to_string()),
            Some("aa".to_string()),
            Some("aaa".to_string())
        ])
    );
    assert_eq!(
        llk.first("a"),
        HashSet::from_iter(vec![Some("a".to_string())])
    );
    assert_eq!(
        llk.first("b"),
        HashSet::from_iter(vec![Some("b".to_string())])
    );
    assert_eq!(
        llk.first("$"),
        HashSet::from_iter(vec![Some("$".to_string())])
    );
    assert_eq!(
        llk.first("Ab$"),
        HashSet::from_iter(vec![
            Some("ab$".to_string()),
            Some("aab".to_string()),
            Some("aaa".to_string())
        ])
    );
    assert_eq!(
        llk.first("aA"),
        HashSet::from_iter(vec![
            Some("aaa".to_string()),
            Some("aa".to_string())
        ])
    );
}
